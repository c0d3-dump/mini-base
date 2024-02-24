use axum_server::Handle;
use jfs::{Config, Store};

use crate::{database::Conn, parser::sql_parser::Trie, server::utils::Utils};

use self::model::{Offset, Temp};
mod migration;
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
    pub trie: Trie,
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
                restart_required: false,
                up_migration_string: "".to_string(),
                down_migration_string: "".to_string(),
                up_migration_written: false,
                down_migration_written: false,
                editor_popup_active: false,
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
            trie: Trie::new(),
        }
    }
}
