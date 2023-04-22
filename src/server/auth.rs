use super::utils::{generate_token, verify_token};
use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::post,
    Router,
};

#[derive(Debug, Clone)]
pub struct AuthState {
    dbconn: Conn,
}

use crate::{
    database::model::ColType,
    server::model::ResponseUser,
    tui::model::{Conn, Model},
};

use super::model::{RegisterUserSchema, User};

pub fn generate_auth_routes(model: Model) -> Router {
    let authstate = AuthState { dbconn: model.conn };

    let router = Router::new()
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn(middleware))
        .route("/signup", post(signup))
        .route("/login", post(login))
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

async fn login(State(state): State<AuthState>, body: String) -> (StatusCode, String) {
    let r_json: Result<RegisterUserSchema, serde_json::Error> = serde_json::from_str(&body);

    match r_json {
        Ok(user) => match &state.dbconn {
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
                    Ok(u) => {
                        let res_user = ResponseUser {
                            id: u.id,
                            email: u.email,
                            token: generate_token().unwrap(),
                        };

                        (StatusCode::OK, serde_json::to_string(&res_user).unwrap())
                    }
                    Err(_) => (StatusCode::BAD_REQUEST, "Something went wrong".to_string()),
                }
            }
            Conn::MYSQL(_) => todo!(),
            Conn::None => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database not connected".to_string(),
            ),
        },
        Err(_) => (StatusCode::BAD_REQUEST, "insufficient params".to_string()),
    }
}

async fn logout() -> (StatusCode, String) {
    (StatusCode::OK, "logout successfully".to_string())
}

pub async fn middleware<T>(req: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if verify_token(auth_header) {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
