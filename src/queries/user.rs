use crate::database::model::ColType;

use super::{model::User, Model};

impl Model {
    pub async fn get_all_users(&self, search_term: &str, offset: i64) -> Result<Vec<User>, String> {
        let query = format!(
            "SELECT id, email, password 
             FROM users 
             WHERE id LIKE %{}% OR email LIKE %{}%
             ORDER BY id 
             LIMIT 25 
             OFFSET {}
            ",
            search_term, search_term, offset
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<User>(&query)
            .await
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> Result<User, String> {
        let query = format!(
            "SELECT id, email, password, roles.name AS role
             FROM users
             INNER JOIN roles ON roles.id=role_id
             WHERE id={}
            ",
            user_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<User>(&query)
            .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, String> {
        let query = format!(
            "SELECT users.id, email, password, 
             CASE WHEN roles.name IS NULL 
              THEN (SELECT name FROM roles WHERE is_default=1) 
              ELSE roles.name END 
             AS role
             FROM users
             LEFT JOIN roles ON roles.id=role_id
             WHERE email='{}'
            ",
            email
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<User>(&query)
            .await
    }

    pub async fn create_user(&self, email: String, password: String) -> Result<u64, String> {
        let query = "INSERT INTO users(email, password) VALUES (?, ?)";

        let args = vec![
            ColType::String(Some(email)),
            ColType::String(Some(password)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }
}
