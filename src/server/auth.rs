use super::{
    model::AuthState,
    utils::{decode_auth_token, generate_auth_token, hash_password},
};
use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::post,
    Router,
};

use crate::{database::model::ColType, server::model::ResponseUser, tui::model::Model};

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
            Conn::MYSQL(c) => {
                let query = "INSERT INTO users(email, password) VALUES (?, ?)";

                let hashed_password = hash_password(user.password);
                let args = vec![
                    ColType::String(Some(user.email)),
                    ColType::String(Some(hashed_password)),
                ];

                c.execute(query, args).await;
                (StatusCode::OK, "Signup successfully".to_string())
            }
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
                            let token = generate_auth_token(u.clone()).unwrap();
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
                    Ok(u) => {
                        let hashed_password = hash_password(user.password);

                        if hashed_password != u.password {
                            (
                                StatusCode::UNAUTHORIZED,
                                "Enter valid email and password".to_string(),
                            )
                        } else {
                            let token = generate_auth_token(u.clone()).unwrap();
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
    State(auth_state): State<AuthState>,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    if auth_state.curr_role.is_empty() {
        req.extensions_mut().insert(auth_state.dbconn);
        req.extensions_mut().insert::<Option<User>>(None);
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

    if let Some(current_user) = authorize_current_user(auth_state.clone(), auth_header).await {
        let mut role = current_user.role.clone();
        if role.is_empty() {
            role = auth_state.default_role;
        }

        if auth_state.curr_role.contains(&role) {
            req.extensions_mut().insert(auth_state.dbconn);
            req.extensions_mut().insert(Some(current_user));
            Ok(next.run(req).await)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(state: AuthState, auth_token: &str) -> Option<User> {
    let token_claim = decode_auth_token(auth_token);

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
