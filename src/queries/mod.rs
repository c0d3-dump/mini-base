use axum_server::Handle;

use crate::database::Conn;

use self::model::{Offset, Temp};
pub mod model;
mod query;
mod role;
mod storage;
mod user;

#[derive(Debug, Clone)]
pub struct Model {
    pub conn: Option<Conn>,
    pub handle: Option<Handle>,
    pub offset: Offset,
    pub temp: Temp,
}

impl Model {
    pub fn default() -> Self {
        Self {
            conn: None,
            handle: None,
            offset: Offset {
                user: 0,
                storage: 0,
            },
            temp: Temp {
                query_access: vec![],
            },
        }
    }
}
