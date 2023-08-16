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
}
