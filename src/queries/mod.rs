use axum_server::Handle;
use jfs::{Config, Store};

use crate::{database::Conn, server::utils::Utils};

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
    pub utils: Utils,
    pub jsondb: Store,
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
                query_string: "".to_string(),
                query_written: false,
                query_access_update: false,
                selected_role_access_id: None,
            },
            utils: Utils {
                auth_secret: String::from("secret"),
                storage_secret: String::from("secret"),
                ips: vec![],
            },
            jsondb: jfs::Store::new_with_cfg(
                "config",
                Config {
                    pretty: true,
                    indent: 4,
                    single: true,
                },
            )
            .unwrap(),
        }
    }
}
