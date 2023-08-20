use crate::database::model::ColType;

use super::{
    model::{DefaultRole, Role, RoleName},
    Model,
};

impl Model {
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

    pub async fn add_new_role(&self, name: String) -> Result<i64, String> {
        let query = "INSERT INTO roles(name) VALUES (?) RETURNING id";
        let args = vec![ColType::String(Some(name))];

        let row = self.conn.as_ref().unwrap().query_one(query, args).await;

        match row {
            Ok(r) => Ok(r.get::<i64>(0).unwrap()),
            Err(e) => Err(e),
        }
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
}
