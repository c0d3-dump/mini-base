use std::{
    fs::{self, File},
    io::Write,
};

use serde::{Deserialize, Serialize};

use super::model::{Conn, Db, Model, QueryList};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct IntermediateModel {
    pub rolelist: Vec<String>,
    pub default_role: String,
    pub querylist: Vec<QueryList>,
}

pub fn to_json(model: Model) {
    let inter_model = IntermediateModel {
        rolelist: model.rolelist,
        default_role: model.default_role,
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

    Model {
        db: Db::None,
        conn: Conn::None,
        handle: None,
        rolelist: inter_model.rolelist,
        default_role: inter_model.default_role,
        querylist: inter_model.querylist,
    }
}
