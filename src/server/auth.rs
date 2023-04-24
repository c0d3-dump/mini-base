use super::{
    model::AuthState,
    utils::{decode_token, generate_token, hash_password},
};
use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::post,
    Router,
};

use crate::{
    database::model::ColType,
    server::model::ResponseUser,
    tui::model::{Conn, Model},
};

use super::model::{RegisterUserSchema, User};

pub fn generate_auth_routes(model: Model) -> Router {
    let authstate = AuthState {
        dbconn: model.conn,
        curr_role: vec![],
        default_role: model.default_role,
    };

    Router::new()
        .route("/logout", post(logout))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .with_state(authstate)
}

async fn signup(State(state): State<AuthState>, body: String) -> (StatusCode, String) {
    let r_json: Result<RegisterUserSchema, serde_json::Error> = serde_json::from_str(&body);

    match r_json {
        Ok(user) => match &state.dbconn {
            Conn::SQLITE(c) => {
                let query = "INSERT INTO users(email, password) VALUES (?, ?)";

                let hashed_password = hash_password(user.password);
                let args = vec![
                    ColType::String(Some(user.email)),
                    ColType::String(Some(hashed_password)),
                ];

                c.execute(query, args).await;
                (StatusCode::OK, "Signup successfully".to_string())
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
                        let hashed_password = hash_password(user.password);

                        if hashed_password != u.password {
                            (
                                StatusCode::UNAUTHORIZED,
                                "Enter valid email and password".to_string(),
                            )
                        } else {
                            let token = generate_token(u.clone()).unwrap();
                            let mut role = u.role;
                            if role.is_empty() {
                                role = state.default_role;
                            }
                            let res_user = ResponseUser {
                                id: u.id,
                                email: u.email,
                                role,
                                token,
                            };

                            (StatusCode::OK, serde_json::to_string(&res_user).unwrap())
                        }
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

pub async fn middleware<T>(
    State(state): State<AuthState>,
    req: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    if state.curr_role.is_empty() {
        return Ok(next.run(req).await);
    }

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(state.clone(), auth_header).await {
        let mut role = current_user.role;
        if role.is_empty() {
            role = state.default_role;
        }

        if state.curr_role.contains(&role) {
            Ok(next.run(req).await)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(state: AuthState, auth_token: &str) -> Option<User> {
    let token_claim = decode_token(auth_token);

    match token_claim {
        Ok(data) => {
            let user = data.claims.user;

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
                        Ok(u) => Some(u),
                        Err(_) => None,
                    }
                }
                Conn::MYSQL(c) => {
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
                        Ok(u) => Some(u),
                        Err(_) => None,
                    }
                }
                Conn::None => None,
            }
        }
        Err(_) => None,
    }
}
