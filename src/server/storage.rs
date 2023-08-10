use crate::{
    database::model::ColType,
    tui::model::{Conn, Model},
};
use axum::{
    extract::State,
    http::StatusCode,
    middleware::{self},
    routing::post,
    Extension, Router,
};
use axum_extra::extract::{multipart::Field, Multipart};
use futures::StreamExt;
use sqlx::Row;
use tokio::{fs, io::AsyncWriteExt};

use super::{
    auth::{self},
    model::{self, AuthState, DeleteFileSchema, StorageState},
};

pub fn generate_storage_routes(model: Model) -> Router {
    let authstate = AuthState {
        dbconn: model.conn.clone(),
        curr_role: vec!["user".to_string()],
        default_role: model.default_role.clone(),
    };

    let storagestate = StorageState {
        default_role: model.default_role,
        storage_access: model.storage_access,
    };

    Router::new()
        .route("/upload", post(upload))
        .route("/delete", post(delete))
        .route_layer(middleware::from_fn_with_state(authstate, auth::middleware))
        .with_state(storagestate)
}

async fn upload(
    State(storage_state): State<StorageState>,
    Extension(db_conn): Extension<Conn>,
    Extension(optional_user): Extension<Option<model::User>>,
    mut multipart: Multipart,
) -> (StatusCode, String) {
    if optional_user.is_none() {
        return (StatusCode::UNAUTHORIZED, "please login first".to_string());
    }

    let mut role = optional_user.clone().unwrap().role;
    if role.is_empty() {
        role = storage_state.default_role;
    }

    let optional_storage_access = storage_state.storage_access.get(&role);

    if optional_storage_access.is_none() || !optional_storage_access.unwrap().write {
        return (
            StatusCode::UNAUTHORIZED,
            "Unauthorized to upload file".to_string(),
        );
    }

    let mut ids = vec![];

    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap().to_string();

        let filename_arr = filename.split(".").collect::<Vec<&str>>();
        let random_id = uuid::Uuid::new_v4().to_string();
        let generated_name = format!("{}.{}", random_id, filename_arr.last().unwrap());

        let save_path = format!("uploads/{}", &generated_name);

        if let Err(_) = save_file(field, &save_path).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error saving file".to_string(),
            );
        }

        match &db_conn {
            Conn::SQLITE(c) => {
                let query =
                    "INSERT INTO storage(file_name, unique_name, uploaded_by) VALUES (?, ?, ?) returning id";

                let mut args = vec![
                    ColType::String(Some(filename)),
                    ColType::String(Some(generated_name)),
                ];

                match optional_user.clone() {
                    Some(user) => args.push(ColType::Integer(Some(user.id))),
                    None => todo!(),
                }

                let optional_row = c.query_one(query, args).await;
                match optional_row {
                    Some(row) => {
                        let id = row.get::<Option<i64>, _>(0).unwrap();
                        ids.push(id);
                    }
                    None => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "error saving file".to_string(),
                        );
                    }
                }
            }
            Conn::MYSQL(c) => {
                let query =
                    "INSERT INTO storage(file_name, unique_name, uploaded_by) VALUES (?, ?, ?)";

                let mut args = vec![
                    ColType::String(Some(filename)),
                    ColType::String(Some(generated_name)),
                ];

                match optional_user.clone() {
                    Some(user) => args.push(ColType::Integer(Some(user.id))),
                    None => todo!(),
                }

                let optional_row = c.query_one(query, args).await;
                match optional_row {
                    Some(row) => {
                        let id = row.get::<Option<i64>, _>(0).unwrap();
                        ids.push(id);
                    }
                    None => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "error saving file".to_string(),
                        );
                    }
                }
            }
            Conn::None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error saving file".to_string(),
                );
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
    State(storage_state): State<StorageState>,
    Extension(db_conn): Extension<Conn>,
    Extension(optional_user): Extension<Option<model::User>>,
    body: String,
) -> (StatusCode, String) {
    if optional_user.is_none() {
        return (StatusCode::UNAUTHORIZED, "please login first".to_string());
    }

    let mut role = optional_user.clone().unwrap().role;
    if role.is_empty() {
        role = storage_state.default_role;
    }

    let optional_storage_access = storage_state.storage_access.get(&role);

    if optional_storage_access.is_none() || !optional_storage_access.unwrap().delete {
        return (
            StatusCode::UNAUTHORIZED,
            "Unauthorized to delete file".to_string(),
        );
    }

    let r_json: Result<DeleteFileSchema, serde_json::Error> = serde_json::from_str(&body);

    match r_json {
        Ok(file) => {
            match &db_conn {
                Conn::SQLITE(c) => {
                    let query = "SELECT unique_name FROM storage WHERE id=?";

                    let args = vec![ColType::Integer(Some(file.id))];

                    let optional_row = c.query_one(query, args).await;
                    match optional_row {
                        Some(row) => {
                            let unique_name = row.get::<Option<String>, _>(0).unwrap();
                            let _ = fs::remove_file(format!("uploads/{}", &unique_name)).await;
                        }
                        None => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "error deleting file".to_string(),
                            );
                        }
                    }

                    let query = "DELETE FROM storage WHERE id=?";

                    let args = vec![ColType::Integer(Some(file.id))];

                    c.execute(query, args).await;
                }
                Conn::MYSQL(c) => {
                    let query = "SELECT unique_name FROM storage WHERE id=?";

                    let args = vec![ColType::Integer(Some(file.id))];

                    let optional_row = c.query_one(query, args).await;
                    match optional_row {
                        Some(row) => {
                            let unique_name = row.get::<Option<String>, _>(0).unwrap();
                            let _ = fs::remove_file(format!("uploads/{}", &unique_name)).await;
                        }
                        None => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "error deleting file".to_string(),
                            );
                        }
                    }

                    let query = "DELETE FROM storage WHERE id=?";

                    let args = vec![ColType::Integer(Some(file.id))];

                    c.execute(query, args).await;
                }
                Conn::None => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "error deleting file".to_string(),
                    );
                }
            }

            (StatusCode::OK, "File deleted successfully".to_string())
        }
        Err(_) => (StatusCode::BAD_REQUEST, "insufficient params".to_string()),
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
