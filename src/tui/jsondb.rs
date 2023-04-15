use async_std::{fs::File, io::WriteExt};
use serde::{Deserialize, Serialize};

use super::model::{Auth, Db, QueryList, RoleList};

#[derive(Debug, Serialize, Deserialize)]
struct IntermediateModel {
    pub db: Db,
    pub auth: Auth,
    pub rolelist: Vec<RoleList>,
    pub querylist: Vec<QueryList>,
}

pub fn to_json(name: &str) {}

pub fn from_json(name: &str) {}
