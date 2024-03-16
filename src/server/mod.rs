use axum::{
    body::Body,
    extract::{Json, Path, Query, State},
    http::{HeaderValue, Method, Request, StatusCode, Uri},
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post, put},
    Extension, Router, ServiceExt,
};
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use std::{collections::HashMap, net::SocketAddr, str::FromStr};
use tower::Layer;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    database::model::ColType,
    parser,
    queries::{model::User, Model},
    server::utils::extract_type_from_string,
};

use self::auth::auth_middleware;

mod auth;
pub mod model;
mod storage;
pub mod utils;

#[tokio::main]
pub async fn start_server(model: Model) {
    let mut cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    if !model.utils.ips.is_empty() {
        let origins: Vec<HeaderValue> = model
            .utils
            .ips
            .clone()
            .into_iter()
            .map(|ip| ip.parse().unwrap())
            .collect::<Vec<HeaderValue>>();

        cors = cors.allow_origin(origins);
    }

    let app = Router::new()
        .route("/health", get(|| async { "Ok" }))
        .nest("/auth", auth::generate_auth_routes(model.clone()))
        .nest("/storage", storage::generate_storage_routes(model.clone()))
        .nest("/api", generate_routes(model.clone()))
        .layer(CookieManagerLayer::new())
        .layer(cors);

    fn rewrite_request_uri<B>(mut req: Request<B>) -> Request<B> {
        let base_uri = req.uri();

        if base_uri.path().contains("/api/") {
            let rem = base_uri.path().replacen("/api/", "", 1);
            let path = rem.replace('/', "_");
            let mut uri = format!("/api/{}", path);

            if let Some(q) = base_uri.query() {
                uri += "?";
                uri += q;
            }

            *req.uri_mut() = uri.parse::<Uri>().unwrap();
        }

        req
    }

    let middleware = tower::util::MapRequestLayer::new(rewrite_request_uri);

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3456));
    axum_server::bind(addr)
        .handle(model.handle.unwrap())
        .serve(middleware.layer(app).into_make_service())
        .await
        .unwrap();
}

fn generate_routes(model: Model) -> Router {
    Router::new()
        .route("/:name", get(get_handler))
        .route("/:name", post(post_handler))
        .route("/:name", put(put_handler))
        .route("/:name", delete(delete_handler))
        .route_layer(middleware::from_fn(auth_middleware))
        .route_layer(middleware::from_fn_with_state(model, name_middleware))
}

async fn name_middleware(
    State(model): State<Model>,
    Path(name): Path<String>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let modified_name = &name.clone().replace('_', "/");
    log::info!("endpoint: {}", &modified_name);

    let optional_query = model.get_query_by_name(modified_name).await;

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

    Ok(next.run(req).await)
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

            let mut args_map = HashMap::new();

            let args = parsed_params
                .clone()
                .into_iter()
                .filter_map(|p| {
                    if p.starts_with('.') && optional_user.is_some() {
                        match &optional_user {
                            Some(user) => {
                                let map = HashMap::from([
                                    (String::from(".USER_ID"), ColType::Integer(Some(user.id))),
                                    (
                                        String::from(".USER_EMAIL"),
                                        ColType::String(Some(user.email.clone())),
                                    ),
                                    (
                                        String::from(".USER_ROLE"),
                                        ColType::String(user.role.clone()),
                                    ),
                                ]);
                                map.get(&p.to_uppercase()).cloned()
                            }
                            None => None,
                        }
                    } else {
                        let d = data
                            .get(&p)
                            .map(|p| ColType::get_col_type_from_value(p.clone()));

                        args_map.insert(p, d.clone());
                        d
                    }
                })
                .collect::<Vec<ColType>>();

            if let Some(user) = optional_user {
                args_map.insert(
                    ".USER_ID".to_string(),
                    Some(ColType::Integer(Some(user.id))),
                );
                args_map.insert(
                    ".USER_EMAIL".to_string(),
                    Some(ColType::String(Some(user.email.clone()))),
                );
                args_map.insert(
                    ".USER_ROLE".to_string(),
                    Some(ColType::String(user.role.clone())),
                );
            }

            let wb = run_webhook(model.clone(), args_map.clone(), query_id, "before").await;
            if let Err(w) = wb {
                return w;
            }

            let res = run_query(model.clone(), parsed_query, args).await;
            if res.0 == StatusCode::BAD_REQUEST {
                return (StatusCode::BAD_REQUEST, res.1);
            }

            let res_args = serde_json::from_str::<Value>(&res.1).unwrap();

            let d = ColType::get_col_type_from_value(res_args);
            args_map.insert("res".to_string(), Some(d));

            let wa = run_webhook(model.clone(), args_map, query_id, "after").await;
            if let Err(w) = wa {
                return w;
            }

            res
        }
        Err(e) => (StatusCode::NOT_FOUND, e),
    }
}

async fn run_webhook(
    model: Model,
    args_map: HashMap<String, Option<ColType>>,
    query_id: i64,
    action_type: &str,
) -> Result<HashMap<String, Value>, (StatusCode, String)> {
    let optional_webhooks = model.get_all_webhook_query_by_query_id(query_id).await;

    match optional_webhooks {
        Ok(webhooks) => {
            let res_map = HashMap::new();
            for webhook in webhooks {
                if webhook.action != action_type {
                    continue;
                }

                let client = reqwest::Client::new();

                let mut builder = match webhook.exec_type.as_str() {
                    "get" => client.get(webhook.url),
                    "post" => client.post(webhook.url),
                    "put" => client.put(webhook.url),
                    "delete" => client.delete(webhook.url),
                    _ => return Err((StatusCode::BAD_REQUEST, "invalid type".to_string())),
                };

                let mut webhook_args = webhook.args.clone();
                webhook_args =
                    parser::replace_variables_with_values(&webhook_args, args_map.clone());

                let args: Value = serde_json::from_str(&webhook_args).unwrap_or(Value::default());

                let header = &args["header"];
                let query = &args["query"];
                let body = &args["body"];

                let mut headermap = HeaderMap::new();
                if let Some(h) = header.as_object() {
                    for (k, v) in h.iter() {
                        let key = reqwest::header::HeaderName::from_str(k);
                        let val = reqwest::header::HeaderValue::from_str(v.as_str().unwrap_or(""));

                        if let (Ok(k), Ok(v)) = (key, val) {
                            headermap.insert(k, v);
                        }
                    }
                }

                if !headermap.is_empty() {
                    builder = builder.headers(headermap);
                }
                if !query.is_null() {
                    builder = builder.query(query);
                }
                if !body.is_null() {
                    builder = builder.json(body);
                }

                match builder.send().await {
                    Ok(res) => {
                        // TODO: might able to get data here and send it to res_map for use in query
                        if webhook.is_returned {
                            if let Err(e) = res.error_for_status() {
                                let mut err_code = StatusCode::BAD_REQUEST;
                                if let Some(s) = e.status() {
                                    err_code = StatusCode::from_u16(s.as_u16())
                                        .unwrap_or(StatusCode::BAD_REQUEST);
                                }
                                return Err((err_code, e.without_url().to_string()));
                            }
                        }
                    }
                    Err(e) => {
                        if webhook.is_returned {
                            return Err((StatusCode::BAD_REQUEST, e.to_string()));
                        }
                    }
                }
            }
            Ok(res_map)
        }
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
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
            (
                StatusCode::OK,
                serde_json::to_string(&out.unwrap()).unwrap(),
            )
        }
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}
