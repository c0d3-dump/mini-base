use crate::{
    database::model::ColType,
    tui::model::{Conn, Model},
};
use axum::{http::StatusCode, middleware, routing::post, Extension, Router};
use axum_extra::extract::{multipart::Field, Multipart};
use futures::StreamExt;
use tokio::{fs, io::AsyncWriteExt};

use super::{
    auth::{self},
    model::{self, AuthState},
};

pub fn generate_storage_routes(model: Model) -> Router {
    let authstate = AuthState {
        dbconn: model.conn,
        curr_role: vec!["user".to_string()],
        default_role: model.default_role,
    };

    Router::new()
        .route(
            "/upload",
            post(
                |Extension(db_conn): Extension<Conn>,
                 Extension(optional_user): Extension<Option<model::User>>,
                 multipart: Multipart| {
                    upload(db_conn, optional_user, multipart)
                },
            ),
        )
        .route_layer(middleware::from_fn_with_state(authstate, auth::middleware))
}

async fn upload(
    db_conn: Conn,
    optional_user: Option<model::User>,
    mut multipart: Multipart,
) -> (StatusCode, String) {
    if optional_user.is_none() {
        return (StatusCode::UNAUTHORIZED, "please login first".to_string());
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        let content_type = field.content_type().unwrap().to_string();
        let filename = field.file_name().unwrap().to_string();

        let filename_arr = filename.split(".").collect::<Vec<&str>>();
        let random_id = uuid::Uuid::new_v4().to_string();
        let generated_name = format!("{}.{}", random_id, filename_arr[1]);

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
                    "INSERT INTO storage(file_name, unique_name, uploaded_by) VALUES (?, ?, ?)";

                let mut args = vec![
                    ColType::String(Some(filename)),
                    ColType::String(Some(generated_name)),
                ];

                match optional_user.clone() {
                    Some(user) => args.push(ColType::Integer(Some(user.id))),
                    None => todo!(),
                }

                c.execute(query, args).await;
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

                c.execute(query, args).await;
            }
            Conn::None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error saving file".to_string(),
                );
            }
        }
    }

    (StatusCode::OK, "File uploaded successfully".to_string())
}

async fn save_file(mut field: Field, save_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(save_path).await?;

    while let Some(chunk) = field.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}
