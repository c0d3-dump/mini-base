use axum::{
    extract::{Json, Path, Query, State},
    http::{Method, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post, put},
    Extension, Router,
};
use serde_json::{json, Value};
use std::{collections::HashMap, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    database::model::ColType,
    parser,
    queries::{model::User, Model},
    server::utils::extract_type_from_string,
};

mod auth;
pub mod model;
// mod storage;
mod utils;

#[tokio::main]
pub async fn start_server(model: Model) {
    let app = Router::new()
        .route("/health", get(|| async { "Ok" }))
        .nest("/auth", auth::generate_auth_routes(model.clone()))
        // .nest("/storage", storage::generate_storage_routes(model.clone()))
        .nest("/api", generate_routes(model.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::POST, Method::GET, Method::OPTIONS])
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3456));

    axum_server::bind(addr)
        .handle(model.handle.unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn generate_routes(model: Model) -> Router {
    let router = Router::new()
        .route("/:name", get(get_handler))
        .route("/:name", post(post_handler))
        .route("/:name", put(put_handler))
        .route("/:name", delete(delete_handler))
        .route_layer(middleware::from_fn(auth_middleware))
        .route_layer(middleware::from_fn_with_state(model, name_middleware));

    router
}

async fn auth_middleware<T>(
    Extension(model): Extension<Model>,
    Extension(query_id): Extension<i64>,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    req.extensions_mut().insert::<Option<User>>(None);

    return Ok(next.run(req).await);
}

async fn name_middleware<T>(
    State(model): State<Model>,
    Path(name): Path<String>,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    log::info!("endpoint: {}", &name);

    let optional_query = model.get_query_by_name(&name).await;

    match optional_query {
        Ok(query) => {
            req.extensions_mut().insert(model);
            req.extensions_mut().insert(query.id);

            match (query.exec_type.as_ref(), req.method()) {
                ("get", &Method::GET) => {}
                ("post", &Method::POST) => {}
                ("put", &Method::PUT) => {}
                ("delete", &Method::DELETE) => {}
                _ => {
                    log::error!("invalid endpoint method: {}", req.method());
                    return Err(StatusCode::NOT_FOUND);
                }
            }
        }
        Err(e) => {
            log::error!("invalid endpoint: {}", e);
            return Err(StatusCode::NOT_FOUND);
        }
    }

    return Ok(next.run(req).await);
}

async fn get_handler(
    Extension(model): Extension<Model>,
    Extension(query_id): Extension<i64>,
    Extension(user): Extension<Option<User>>,
    Query(query): Query<HashMap<String, String>>,
) -> (StatusCode, String) {
    let mut json = json!({});
    for (key, val) in query {
        json[key] = extract_type_from_string(&val);
    }

    handler(model, query_id, user, json).await
}

async fn post_handler(
    Extension(model): Extension<Model>,
    Extension(query_id): Extension<i64>,
    Extension(user): Extension<Option<User>>,
    Json(body): Json<Value>,
) -> (StatusCode, String) {
    handler(model, query_id, user, body).await
}

async fn put_handler(
    Extension(model): Extension<Model>,
    Extension(query_id): Extension<i64>,
    Extension(user): Extension<Option<User>>,
    Json(body): Json<Value>,
) -> (StatusCode, String) {
    handler(model, query_id, user, body).await
}

async fn delete_handler(
    Extension(model): Extension<Model>,
    Extension(query_id): Extension<i64>,
    Extension(user): Extension<Option<User>>,
    Json(body): Json<Value>,
) -> (StatusCode, String) {
    handler(model, query_id, user, body).await
}

async fn handler(
    model: Model,
    query_id: i64,
    optional_user: Option<User>,
    data: Value,
) -> (StatusCode, String) {
    dbg!(&data);

    let optional_query_string = model.get_query_string_by_id(query_id).await;

    match optional_query_string {
        Ok(query_string) => {
            let (_, params) = parser::parse_query(&query_string.query).unwrap();

            let parsed_query =
                parser::replace_variables_in_query(&query_string.query, params.clone());

            let parsed_params = params
                .into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>();

            let args = parsed_params
                .clone()
                .into_iter()
                .filter_map(|p| {
                    if p.starts_with(".") && !optional_user.is_none() {
                        match &optional_user {
                            Some(user) => {
                                let map = HashMap::from([
                                    (String::from(".userId"), ColType::Integer(Some(user.id))),
                                    (
                                        String::from(".userEmail"),
                                        ColType::String(Some(user.email.clone())),
                                    ),
                                    // (
                                    //     String::from(".userRole"),
                                    //     ColType::String(Some(user.role.clone())),
                                    // ),
                                ]);
                                match map.get(&p) {
                                    Some(m) => Some(m.clone()),
                                    None => None,
                                }
                            }
                            None => None,
                        }
                    } else {
                        match data.get(&p).clone() {
                            Some(p) => Some(match p.clone() {
                                Value::Bool(t) => ColType::Bool(Some(t)),
                                Value::Number(t) => ColType::Real(t.as_f64()),
                                Value::String(t) => ColType::String(Some(t)),
                                Value::Array(_) => todo!(),
                                Value::Object(_) => todo!(),
                                Value::Null => ColType::Bool(None),
                            }),
                            None => None,
                        }
                    }
                })
                .collect::<Vec<ColType>>();

            run_query(model, parsed_query, args).await
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
    }
}

async fn run_query(
    model: Model,
    query: String,
    args: Vec<ColType>,
) -> (StatusCode, std::string::String) {
    let optional_rows = model.conn.as_ref().unwrap().query_all(&query, args).await;

    match optional_rows {
        Ok(rows) => {
            let out = model.conn.as_ref().unwrap().parse_all(rows);
            (StatusCode::OK, serde_json::to_string(&out).unwrap())
        }
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}
