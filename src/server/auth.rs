use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use axum::{extract::State, http::StatusCode, routing::post, Router};

#[derive(Debug, Clone)]
struct AuthState {
    dbconn: Conn,
}

use crate::{
    database::model::ColType,
    tui::model::{Conn, Model},
};

use super::model::{RegisterUserSchema, User};

pub fn generate_auth_routes(model: Model) -> Router {
    let authstate = AuthState { dbconn: model.conn };

    let router = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .with_state(authstate);

    router
}

async fn signup(State(state): State<AuthState>, body: String) -> (StatusCode, String) {
    let r_json: Result<RegisterUserSchema, serde_json::Error> = serde_json::from_str(&body);

    match r_json {
        Ok(user) => {
            match &state.dbconn {
                Conn::SQLITE(c) => {
                    let query = "INSERT INTO users(email, password) VALUES (?, ?)";
                    let args = vec![
                        ColType::String(Some(user.email)),
                        ColType::String(Some(user.password)),
                    ];

                    c.execute(query, args).await;
                    (StatusCode::OK, "Signup successfully".to_string())
                }
                Conn::MYSQL(_) => todo!(),
                Conn::None => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database not connected".to_string(),
                ),
            }

            // (StatusCode::OK, "".to_string())
        }
        Err(_) => (StatusCode::BAD_REQUEST, "insufficient params".to_string()),
    }
}

async fn login(state: State<AuthState>, body: String) -> (StatusCode, String) {
    let r_json: Result<RegisterUserSchema, serde_json::Error> = serde_json::from_str(&body);

    match r_json {
        Ok(user) => {
            match &state.dbconn {
                Conn::SQLITE(c) => {
                    let query = "SELECT * FROM users WHERE email = ?";

                    let conn = match c.clone().connection {
                        Some(conn) => conn,
                        None => panic!("database not connected"),
                    };

                    let r_out: Result<User, sqlx::Error> = sqlx::query_as(query)
                        .bind(user.email)
                        .fetch_one(&conn)
                        .await;

                    match r_out {
                        Ok(out) => (StatusCode::OK, serde_json::to_string(&out).unwrap()),
                        Err(_) => (StatusCode::BAD_REQUEST, "Something went wrong".to_string()),
                    }
                }
                Conn::MYSQL(_) => todo!(),
                Conn::None => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database not connected".to_string(),
                ),
            }

            // (StatusCode::OK, "".to_string())
        }
        Err(_) => (StatusCode::BAD_REQUEST, "insufficient params".to_string()),
    }
}

async fn logout(state: State<AuthState>, body: String) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, "okay".to_string())
}
