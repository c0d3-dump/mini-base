use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::post,
    Json, Router,
};
use serde_json::Value;

use crate::queries::Model;

use super::{
    model::ResponseUser,
    utils::{generate_auth_token, hash_password},
};

pub fn generate_auth_routes(model: Model) -> Router {
    Router::new()
        .route("/logout", post(logout))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .with_state(model)
}

async fn signup(State(model): State<Model>, Json(body): Json<Value>) -> (StatusCode, String) {
    let email = body.get("email");
    let password = body.get("password");

    if email.is_none() || password.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            "Insufficient parameters".to_string(),
        );
    }

    match (email, password) {
        (Some(Value::String(email)), Some(Value::String(password))) => {
            let hashed_password = hash_password(password.to_string());
            let res = model.create_user(email.to_string(), hashed_password).await;

            match res {
                Ok(_) => (StatusCode::OK, "Signup successfull".to_string()),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            }
        }
        (_, _) => (
            StatusCode::BAD_REQUEST,
            "Insufficient parameters".to_string(),
        ),
    }
}

async fn login(State(model): State<Model>, Json(body): Json<Value>) -> (StatusCode, String) {
    let email = body.get("email");
    let password = body.get("password");

    if email.is_none() || password.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            "Insufficient parameters".to_string(),
        );
    }

    match (email, password) {
        (Some(Value::String(email)), Some(Value::String(password))) => {
            log::info!("email: {}", email);
            log::info!("password: {}", password);
            let res = model.get_user_by_email(email.as_str()).await;

            match res {
                Ok(user) => {
                    let hashed_password = hash_password(password.to_string());

                    if hashed_password != user.password {
                        (
                            StatusCode::UNAUTHORIZED,
                            "Enter valid email and password".to_string(),
                        )
                    } else {
                        let token = generate_auth_token(user.clone()).unwrap();
                        let role = user.role;

                        let res_user = ResponseUser {
                            id: user.id,
                            email: user.email,
                            role,
                            token,
                        };

                        (StatusCode::OK, serde_json::to_string(&res_user).unwrap())
                    }
                }
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            }
        }
        (_, _) => (
            StatusCode::BAD_REQUEST,
            "Insufficient parameters".to_string(),
        ),
    }
}

async fn logout() -> (StatusCode, String) {
    (StatusCode::OK, "logout successfully".to_string())
}

// pub async fn middleware<T>(
//     State(model): State<Model>,
//     mut req: Request<T>,
//     next: Next<T>,
// ) -> Result<Response, StatusCode> {
//     if auth_state.curr_role.is_empty() {
//         req.extensions_mut().insert(auth_state.dbconn);
//         req.extensions_mut().insert::<Option<User>>(None);
//         return Ok(next.run(req).await);
//     }

//     let auth_header = req
//         .headers()
//         .get(header::AUTHORIZATION)
//         .and_then(|header| header.to_str().ok());

//     let auth_header = if let Some(auth_header) = auth_header {
//         auth_header
//     } else {
//         return Err(StatusCode::UNAUTHORIZED);
//     };

//     if let Some(current_user) = authorize_current_user(auth_state.clone(), auth_header).await {
//         let mut role = current_user.role.clone();
//         if role.is_empty() {
//             role = auth_state.default_role;
//         }

//         if auth_state.curr_role.contains(&role) {
//             req.extensions_mut().insert(auth_state.dbconn);
//             req.extensions_mut().insert(Some(current_user));
//             Ok(next.run(req).await)
//         } else {
//             Err(StatusCode::UNAUTHORIZED)
//         }
//     } else {
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }

// async fn authorize_current_user(state: AuthState, auth_token: &str) -> Option<User> {
//     let token_claim = decode_auth_token(auth_token);

//     match token_claim {
//         Ok(data) => {
//             let user = data.claims.user;

//             match &state.dbconn {
//                 Conn::SQLITE(c) => {
//                     let query = "SELECT * FROM users WHERE email = ?";

//                     let conn = match c.clone().connection {
//                         Some(conn) => conn,
//                         None => panic!("database not connected"),
//                     };

//                     let r_out: Result<User, sqlx::Error> = sqlx::query_as(query)
//                         .bind(user.email)
//                         .fetch_one(&conn)
//                         .await;

//                     match r_out {
//                         Ok(u) => Some(u),
//                         Err(_) => None,
//                     }
//                 }
//                 Conn::MYSQL(c) => {
//                     let query = "SELECT * FROM users WHERE email = ?";

//                     let conn = match c.clone().connection {
//                         Some(conn) => conn,
//                         None => panic!("database not connected"),
//                     };

//                     let r_out: Result<User, sqlx::Error> = sqlx::query_as(query)
//                         .bind(user.email)
//                         .fetch_one(&conn)
//                         .await;

//                     match r_out {
//                         Ok(u) => Some(u),
//                         Err(_) => None,
//                     }
//                 }
//                 Conn::None => None,
//             }
//         }
//         Err(_) => None,
//     }
// }
