use axum::{
    handler::Handler,
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Router,
};
use axum_server::Handle;
use serde_json::{Number, Value};
use std::net::SocketAddr;

use crate::{
    database::model::ColType,
    parser,
    tui::model::{Conn, Model},
};

#[tokio::main]
pub async fn start_server(model: Model, handle: Handle) {
    let app = Router::new()
        .route("/health", get(|| async { "200: ok" }))
        .nest("/api", generate_routes(model));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn generate_routes(model: Model) -> Router {
    let mut router = Router::new();

    for query in model.querylist {
        let path = format!("/{}", query.label);

        let (_, params) = parser::parse_query(&query.query).unwrap();

        let parsed_query = parser::replace_variables_in_query(
            parser::DbType::SQLITE,
            &query.query,
            params.clone(),
        );

        let parsed_params = params
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>();

        let dbconn = model.conn.clone();

        router = router.route(
            &path,
            post(move |body: String| handler(body, parsed_query, parsed_params, dbconn)),
        );
    }

    router
}

async fn handler(
    body: String,
    query: String,
    params: Vec<String>,
    dbconn: Conn,
) -> (StatusCode, String) {
    if params.len() > 0 {
        let r_json: Result<Value, serde_json::Error> = serde_json::from_str(&body);

        let res = match r_json {
            Ok(json) => {
                let args = params
                    .into_iter()
                    .map(|p| match json[p].clone() {
                        Value::Null => ColType::String(None),
                        Value::Bool(t) => ColType::Bool(Some(t)),
                        Value::Number(t) => ColType::Integer(t.as_i64()),
                        Value::String(t) => ColType::String(Some(t)),
                        _ => panic!(),
                    })
                    .collect::<Vec<ColType>>();

                match dbconn {
                    Conn::SQLITE(c) => {
                        let rows = c.query_all(&query, args).await;
                        let out = c.parse_all(rows);

                        return (StatusCode::OK, serde_json::to_string(&out).unwrap());
                    }
                    Conn::MYSQL => panic!(),
                    Conn::POSTGRES => panic!(),
                    Conn::None => panic!(),
                }
            }
            Err(_) => (StatusCode::BAD_REQUEST, "insufficient params".to_owned()),
        };

        return res;
    }

    match dbconn {
        Conn::SQLITE(c) => {
            let rows = c.query_all(&query, vec![]).await;
            let out = c.parse_all(rows);

            // println!("{}", serde_json::to_string(&out).unwrap());

            return (StatusCode::OK, serde_json::to_string(&out).unwrap());
        }
        Conn::MYSQL => panic!(),
        Conn::POSTGRES => panic!(),
        Conn::None => panic!(),
    }

    // (StatusCode::BAD_REQUEST, "okay".to_owned())
}
