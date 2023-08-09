use axum::{
    http::{Method, StatusCode},
    middleware,
    routing::{get, post},
    Extension, Router,
};
use axum_server::Handle;
use serde_json::Value;
use std::{collections::HashMap, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    database::model::ColType,
    parser,
    tui::model::{Conn, ExecType, Model},
};

mod auth;
pub mod model;
mod storage;
mod utils;

#[tokio::main]
pub async fn start_server(model: Model, handle: Handle) {
    let app = Router::new()
        .route("/health", get(|| async { "Ok" }))
        .nest("/auth", auth::generate_auth_routes(model.clone()))
        .nest("/storage", storage::generate_storage_routes(model.clone()))
        .nest("/api", generate_routes(model))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::POST, Method::OPTIONS])
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 3456));

    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn generate_routes(model: Model) -> Router {
    let mut router = Router::new();

    for query in model.clone().querylist {
        let path = format!("/{}", query.label);

        let (_, params) = parser::parse_query(&query.query).unwrap();

        let parsed_query = parser::replace_variables_in_query(&query.query, params.clone());

        let parsed_params = params
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>();

        let dbconn = model.conn.clone();

        let authstate = model::AuthState {
            dbconn: model.clone().conn,
            curr_role: query.role_access,
            default_role: model.clone().default_role,
        };

        router = router.route(
            &path,
            post(
                move |Extension(optional_user): Extension<Option<model::User>>, body: String| {
                    handler(
                        body,
                        parsed_query,
                        parsed_params,
                        optional_user,
                        dbconn,
                        query.exec_type,
                    )
                },
            )
            .route_layer(middleware::from_fn_with_state(authstate, auth::middleware)),
        );
    }

    router
}

async fn handler(
    body: String,
    query: String,
    params: Vec<String>,
    optional_user: Option<model::User>,
    dbconn: Conn,
    exectype: ExecType,
) -> (StatusCode, String) {
    if !params.is_empty() {
        let mod_body = if body.is_empty() { "{}" } else { &body };

        let r_json: Result<Value, serde_json::Error> = serde_json::from_str(mod_body);

        let res = match r_json {
            Ok(json) => {
                let args = params
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
                                        (
                                            String::from(".userRole"),
                                            ColType::String(Some(user.role.clone())),
                                        ),
                                    ]);
                                    match map.get(&p) {
                                        Some(m) => Some(m.clone()),
                                        None => None,
                                    }
                                }
                                None => None,
                            }
                        } else {
                            match json.get(p).clone() {
                                Some(p) => Some(match p.clone() {
                                    Value::Bool(t) => ColType::Bool(Some(t)),
                                    Value::Number(t) => ColType::Integer(t.as_i64()),
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

                run_query(dbconn, query, exectype, args).await
            }
            Err(_) => (StatusCode::BAD_REQUEST, "insufficient params".to_owned()),
        };

        return res;
    }

    run_query(dbconn, query, exectype, vec![]).await
}

async fn run_query(
    dbconn: Conn,
    query: String,
    exectype: ExecType,
    args: Vec<ColType>,
) -> (StatusCode, std::string::String) {
    match dbconn {
        Conn::SQLITE(c) => match exectype {
            ExecType::QUERY => {
                let rows = c.query_all(&query, args).await;
                let out = c.parse_all(rows);

                (StatusCode::OK, serde_json::to_string(&out).unwrap())
            }
            ExecType::EXECUTION => {
                let out = c.execute(&query, args).await;

                (StatusCode::OK, out.to_string())
            }
        },
        Conn::MYSQL(c) => match exectype {
            ExecType::QUERY => {
                let rows = c.query_all(&query, args).await;
                let out = c.parse_all(rows);

                (StatusCode::OK, serde_json::to_string(&out).unwrap())
            }
            ExecType::EXECUTION => {
                let out = c.execute(&query, args).await;

                (StatusCode::OK, out.to_string())
            }
        },
        Conn::None => panic!(),
    }
}
