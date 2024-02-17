use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::post,
    Extension, Json, Router,
};
use cookie::time::{Duration, OffsetDateTime};
use serde_json::Value;
use tower_cookies::{Cookie, Cookies};

use crate::queries::{
    model::{User, UserId, UserStorage},
    Model,
};

use super::{model::ResponseUser, utils::hash_password};

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
                Ok(_) => (StatusCode::OK, "Signup successful".to_string()),
                Err(_) => (StatusCode::BAD_REQUEST, "Invalid Credentials".to_string()),
            }
        }
        (_, _) => (
            StatusCode::BAD_REQUEST,
            "Insufficient parameters".to_string(),
        ),
    }
}

async fn login(
    State(model): State<Model>,
    cookies: Cookies,
    Json(body): Json<Value>,
) -> (StatusCode, String) {
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
                        let token = model.utils.generate_auth_token(user.clone()).unwrap();

                        let mut cookie = Cookie::new("auth", token);
                        cookie.set_http_only(true);
                        cookie.set_path("/");

                        let mut fut = OffsetDateTime::now_utc();
                        fut += Duration::days(7);
                        cookie.set_expires(fut);

                        cookies.add(cookie);

                        let res_user = ResponseUser {
                            id: user.id,
                            email: user.email,
                            role: user.role,
                        };

                        (StatusCode::OK, serde_json::to_string(&res_user).unwrap())
                    }
                }
                Err(_) => (StatusCode::BAD_REQUEST, "Invalid Credentials".to_string()),
            }
        }
        (_, _) => (
            StatusCode::BAD_REQUEST,
            "Insufficient parameters".to_string(),
        ),
    }
}

async fn logout(cookies: Cookies) -> (StatusCode, String) {
    let mut cookie = Cookie::from("auth");
    cookie.set_path("/");
    cookies.remove(cookie);

    (StatusCode::OK, "logout successfully".to_string())
}

pub async fn auth_middleware(
    Extension(model): Extension<Model>,
    Extension(query_id): Extension<i64>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let optional_role_access = model.get_all_role_access_by_query_id(query_id).await;

    let role_access;
    match optional_role_access {
        Ok(ra) => {
            if ra.is_empty() {
                req.extensions_mut().insert::<Option<User>>(None);
                return Ok(next.run(req).await);
            }
            role_access = ra;
        }
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    match cookies.get("auth") {
        Some(cookie) => {
            let token = cookie.value();
            let optional_user = authorize_current_user(&model, token).await;
            match optional_user {
                Some(user) => {
                    if user.role_id.is_none() {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                    if role_access
                        .into_iter()
                        .map(|ra| ra.role_id)
                        .collect::<Vec<i64>>()
                        .contains(&user.role_id.unwrap())
                    {
                        req.extensions_mut().insert(Some(User {
                            id: user.id,
                            email: user.email,
                            password: user.password,
                            role: user.role_name,
                        }));
                    } else {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
                None => return Err(StatusCode::UNAUTHORIZED),
            }
        }
        None => return Err(StatusCode::UNAUTHORIZED),
    }

    Ok(next.run(req).await)
}

pub async fn storage_middleware(
    State(model): State<Model>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    match cookies.get("auth") {
        Some(cookie) => {
            let token = cookie.value();
            let optional_user = authorize_current_user(&model, token).await;
            match optional_user {
                Some(user) => {
                    if user.role_id.is_none() {
                        return Err(StatusCode::UNAUTHORIZED);
                    }

                    let optional_access = model.get_role_by_id(user.role_id.unwrap()).await;
                    match optional_access {
                        Ok(role) => {
                            req.extensions_mut().insert(model);
                            req.extensions_mut().insert(Some(UserStorage {
                                id: user.id,
                                role_id: user.role_id,
                                can_read: role.can_read,
                                can_write: role.can_write,
                                can_delete: role.can_delete,
                            }));
                        }
                        Err(_) => return Err(StatusCode::UNAUTHORIZED),
                    }
                }
                None => return Err(StatusCode::UNAUTHORIZED),
            }
        }
        None => {
            req.extensions_mut().insert(model);
            req.extensions_mut().insert::<Option<UserStorage>>(None);
        }
    }

    Ok(next.run(req).await)
}

async fn authorize_current_user(model: &Model, auth_token: &str) -> Option<UserId> {
    let token_claim = model.utils.decode_auth_token(auth_token);

    match token_claim {
        Ok(data) => {
            let user = data.claims.user;

            let res = model.get_user_by_id(user.id).await;

            match res {
                Ok(u) => Some(u),
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}
