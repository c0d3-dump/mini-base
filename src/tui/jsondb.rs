use std::{
    fs::{self, File},
    io::Write,
};

use serde::{Deserialize, Serialize};

use crate::database::{self, mysql::Mysql, sqlite::Sqlite};

use super::model::{Auth, Conn, Db, Model, QueryList, RoleList};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct IntermediateModel {
    pub db: Db,
    pub auth: Vec<Auth>,
    pub rolelist: Vec<RoleList>,
    pub querylist: Vec<QueryList>,
}

pub fn to_json(model: Model) {
    let inter_model = IntermediateModel {
        db: model.db,
        auth: model.auth,
        rolelist: model.rolelist,
        querylist: model.querylist,
    };
    let data = serde_json::to_string(&inter_model).unwrap();

    let mut file = File::create("config.json").unwrap();
    file.write(data.as_bytes()).unwrap();
}

pub fn from_json() -> Model {
    let optional_data = fs::read_to_string("config.json");

    let data = match optional_data {
        Ok(data) => data,
        Err(_) => {
            let mut file = File::create("config.json").unwrap();
            let inter_model = IntermediateModel::default();
            let data = serde_json::to_string(&inter_model).unwrap();

            file.write(data.as_bytes()).unwrap();

            fs::read_to_string("config.json").unwrap()
        }
    };

    let inter_model: IntermediateModel = serde_json::from_str(&data).unwrap();

    let conn = match inter_model.clone().db {
        Db::SQLITE { dbpath } => {
            let conn = database::sqlite::Sqlite::new(&dbpath);
            Conn::SQLITE(conn)
        }
        Db::MYSQL { dbpath } => {
            let conn = database::mysql::Mysql::new(&dbpath);
            Conn::MYSQL(conn)
        }
        Db::POSTGRES { dbpath } => todo!(),
        Db::None => Conn::None,
    };

    Model {
        db: inter_model.db,
        conn: match conn {
            Conn::SQLITE(c) => match c.connection {
                Some(con) => Conn::SQLITE(Sqlite {
                    connection: Some(con),
                    err: None,
                }),
                None => Conn::None,
            },
            Conn::MYSQL(c) => match c.connection {
                Some(con) => Conn::MYSQL(Mysql {
                    connection: Some(con),
                    err: None,
                }),
                None => Conn::None,
            },
            Conn::POSTGRES => todo!(),
            Conn::None => Conn::None,
        },
        handle: None,
        auth: inter_model.auth,
        rolelist: inter_model.rolelist,
        querylist: inter_model.querylist,
    }
}
