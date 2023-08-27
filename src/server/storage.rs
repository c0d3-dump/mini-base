use std::collections::HashMap;

use crate::queries::{model::UserStorage, Model};
use axum::{
    body::{Bytes, Full},
    extract::Query,
    http::StatusCode,
    middleware::{self},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::{multipart::Field, Multipart};
use futures::StreamExt;
use serde_json::Value;
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

use super::{
    auth::{self},
    model::TokenFile,
    utils::{decode_storage_token, generate_storage_token},
};

pub fn generate_storage_routes(model: Model) -> Router {
    Router::new()
        .route("/upload", post(upload))
        .route("/delete", post(delete))
        .route("/generate-token", post(generate_token))
        .route_layer(middleware::from_fn_with_state(
            model,
            auth::storage_middleware,
        ))
        .route("/get", get(get_file))
}

async fn upload(
    Extension(model): Extension<Model>,
    Extension(user_storage): Extension<Option<UserStorage>>,
    mut multipart: Multipart,
) -> (StatusCode, String) {
    let user_id;
    match user_storage {
        Some(user) => {
            if !user.can_write {
                return (
                    StatusCode::UNAUTHORIZED,
                    "Unauthorized to upload file".to_string(),
                );
            }
            user_id = user.id;
        }
        None => return (StatusCode::UNAUTHORIZED, "please login first".to_string()),
    }

    let mut ids = vec![];
    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                "error retrieving field".to_string(),
            );
        }
    } {
        let file_name = field.file_name().unwrap().to_string();

        let filename_arr = file_name.split(".").collect::<Vec<&str>>();
        let random_id = uuid::Uuid::new_v4().to_string();
        let generated_name = format!("{}.{}", random_id, filename_arr.last().unwrap());

        let save_path = format!("uploads/{}", &generated_name);

        if let Err(_) = save_file(field, &save_path).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error saving file".to_string(),
            );
        }

        let res = model.upload_file(file_name, generated_name, user_id).await;
        match res {
            Ok(id) => {
                ids.push(id);
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error saving file".to_string(),
                )
            }
        }
    }

    let ids_str = ids
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    (StatusCode::OK, ids_str)
}

async fn delete(
    Extension(model): Extension<Model>,
    Extension(user_storage): Extension<Option<UserStorage>>,
    Json(body): Json<Value>,
) -> (StatusCode, String) {
    match user_storage {
        Some(user) => {
            if !user.can_delete {
                return (
                    StatusCode::UNAUTHORIZED,
                    "Unauthorized to delete file".to_string(),
                );
            }
        }
        None => return (StatusCode::UNAUTHORIZED, "please login first".to_string()),
    }

    let file_id: i64;
    match body.get("file_id") {
        Some(Value::Number(f_id)) => {
            file_id = f_id.as_i64().unwrap();

            let res = model.get_file_by_id(file_id).await;
            match res {
                Ok(s) => {
                    let _ = fs::remove_file(format!("uploads/{}", &s.unique_name)).await;
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "error deleting file".to_string(),
                    );
                }
            }
        }
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error deleting file".to_string(),
            );
        }
    }

    match model.delete_file(file_id).await {
        Ok(r) => {
            if r <= 0 {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error deleting file".to_string(),
                );
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error deleting file".to_string(),
            );
        }
    }

    (StatusCode::OK, "File deleted successfully".to_string())
}

async fn generate_token(
    Extension(model): Extension<Model>,
    Extension(user_storage): Extension<Option<UserStorage>>,
    Json(body): Json<Value>,
) -> (StatusCode, String) {
    match user_storage {
        Some(user) => {
            if !user.can_read {
                return (
                    StatusCode::UNAUTHORIZED,
                    "Unauthorized to generate token".to_string(),
                );
            }
        }
        None => return (StatusCode::UNAUTHORIZED, "please login first".to_string()),
    }

    let file_id: i64;
    match body.get("file_id") {
        Some(Value::Number(f_id)) => {
            file_id = f_id.as_i64().unwrap();
        }
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error generating token".to_string(),
            );
        }
    }

    match model.get_file_by_id(file_id).await {
        Ok(s) => {
            let optional_token = generate_storage_token(TokenFile {
                unique_name: s.unique_name,
            });
            match optional_token {
                Ok(token) => {
                    let url = format!("http://localhost:3456/storage/get?token={}", token);
                    return (StatusCode::OK, url);
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "error generating token".to_string(),
                    );
                }
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error generating token".to_string(),
            );
        }
    }
}

async fn get_file(
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let token;
    match query.get("token") {
        Some(t) => {
            token = t;
        }
        None => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    match decode_storage_token(token) {
        Ok(token_file) => {
            let file_path = format!("./uploads/{}", token_file.claims.file.unique_name);

            let mut file = match File::open(&file_path).await {
                Ok(file) => file,
                Err(_) => {
                    return Err(StatusCode::NOT_FOUND);
                }
            };

            let mut content = Vec::new();
            if let Err(_) = file.read_to_end(&mut content).await {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }

            let response = Response::builder()
                .header(
                    "Content-Disposition",
                    format!(
                        "attachment; filename={}",
                        token_file.claims.file.unique_name
                    ),
                )
                .header("Content-Type", "application/octet-stream")
                .body(Full::new(Bytes::from(content)));

            match response {
                Ok(r) => return Ok(r),
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

async fn save_file(mut field: Field, save_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(save_path).await?;

    while let Some(chunk) = field.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}
