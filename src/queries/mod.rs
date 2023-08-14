use axum_server::Handle;

use crate::database::{model::ColType, Conn};

use self::model::{
    Index, Offset, Query, QueryString, Role, RoleAccess, RoleName, SearchTerm, Storage, User,
};
pub mod model;

#[derive(Debug, Clone)]
pub struct Model {
    pub conn: Option<Conn>,
    pub handle: Option<Handle>,
    pub offset: Offset,
    pub search_term: SearchTerm,
    pub index: Index,
}

impl Model {
    pub fn default() -> Self {
        Self {
            conn: None,
            handle: None,
            offset: Offset {
                user: 0,
                query: 0,
                storage: 0,
            },
            search_term: SearchTerm {
                user: "".to_string(),
                query: "".to_string(),
                storage: "".to_string(),
            },
            index: Index {
                role: 0,
                user: 0,
                query: 0,
                storage: 0,
            },
        }
    }

    pub async fn get_all_roles(&self) -> Result<Vec<RoleName>, String> {
        let query = "SELECT id, name FROM roles ORDER BY id";

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<RoleName>(query)
            .await
    }

    pub async fn get_role_by_id(&self, role_id: i64) -> Result<Role, String> {
        let query = format!(
            "SELECT id, name, is_default, can_read, can_write, can_delete 
             FROM roles 
             WHERE id={}",
            role_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<Role>(&query)
            .await
    }

    pub async fn add_new_role(&self, name: String) -> Result<u64, String> {
        let query = "INSERT INTO roles(name) VALUES (?)";
        let args = vec![ColType::String(Some(name))];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn edit_role(&self, role: Role) -> Result<u64, String> {
        let query = "UPDATE roles 
            SET name=?, is_default=?, can_read=?, can_write=?, can_delete=? 
            WHERE id=?";
        let args = vec![
            ColType::String(Some(role.name)),
            ColType::Bool(Some(role.is_default)),
            ColType::Bool(Some(role.can_read)),
            ColType::Bool(Some(role.can_write)),
            ColType::Bool(Some(role.can_delete)),
            ColType::Integer(Some(role.id)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn delete_role(&self, role_id: i64) -> Result<u64, String> {
        let query = "DELETE FROM roles WHERE id=?";
        let args = vec![ColType::Integer(Some(role_id))];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, String> {
        let query = format!(
            "SELECT id, email, password 
              FROM users 
              WHERE id LIKE %{}% OR email LIKE %{}%
              ORDER BY id 
              LIMIT 25 
              OFFSET {}
            ",
            self.search_term.user, self.search_term.user, self.offset.user
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<User>(&query)
            .await
    }

    pub async fn get_all_queries(&self) -> Result<Vec<Query>, String> {
        let query = format!(
            "SELECT id, name, exec_type 
              FROM queries 
              WHERE id LIKE %{}% OR name LIKE %{}% OR exec_type LIKE %{}%
              ORDER BY id 
              LIMIT 25 
              OFFSET {}
          ",
            self.search_term.query,
            self.search_term.query,
            self.search_term.query,
            self.offset.query
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<Query>(&query)
            .await
    }

    pub async fn get_all_storage(&self) -> Result<Vec<Storage>, String> {
        let query = format!(
            "SELECT id, file_name, unique_name 
              FROM storage 
              WHERE id LIKE %{}% OR file_name LIKE %{}%
              ORDER BY id 
              LIMIT 25 
              OFFSET {}
          ",
            self.search_term.storage, self.search_term.storage, self.offset.storage
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<Storage>(&query)
            .await
    }

    pub async fn get_query_access_roles_by_id(
        &self,
        query_id: i64,
    ) -> Result<Vec<RoleAccess>, String> {
        let query = format!("SELECT role_id FROM role_access WHERE id={}", query_id);

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<RoleAccess>(&query)
            .await
    }

    pub async fn get_query_string_by_id(&self, query_id: i64) -> Result<QueryString, String> {
        let query = format!("SELECT query FROM queries WHERE id={}", query_id);

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<QueryString>(&query)
            .await
    }
}
