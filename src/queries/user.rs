use crate::database::model::ColType;

use super::{
    model::{User, UserId, UserRoleAccess},
    Model,
};

impl Model {
    pub async fn get_all_users(&self) -> Result<Vec<User>, String> {
        let query = "SELECT users.id, email, password, roles.name AS role
                 FROM users 
                 INNER JOIN roles ON roles.id=role_id
                 ORDER BY users.id
                ";

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<User>(query)
            .await
    }

    pub async fn add_default_user(&self, user_email: String) -> Result<i64, String> {
        let query = "UPDATE users 
                    SET role_id=(SELECT id FROM roles WHERE is_default=1)
                    WHERE role_id IS NULL AND users.email=? RETURNING users.id";

        let args = vec![ColType::String(Some(user_email))];

        let res = self.conn.as_ref().unwrap().query_one(query, args).await;
        match res {
            Ok(r) => Ok(r.get::<i64>(0).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub async fn remove_default_user(&self, user_id: i64) -> Result<u64, String> {
        let query = "UPDATE users 
                    SET role_id=NULL
                    WHERE users.id=?";

        let args = vec![ColType::Integer(Some(user_id))];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn update_user_role(&self, user_id: i64, role_id: i64) -> Result<u64, String> {
        let query = "UPDATE users 
                    SET role_id=?
                    WHERE users.id=?";

        let args = vec![
            ColType::Integer(Some(role_id)),
            ColType::Integer(Some(user_id)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn get_user_role_access_by_id(
        &self,
        user_id: i64,
    ) -> Result<Vec<UserRoleAccess>, String> {
        let query = format!(
            "SELECT roles.id AS role_id, name,
             (CASE WHEN users.id IS NULL THEN FALSE ELSE TRUE END) AS is_selected 
             FROM roles
             LEFT JOIN users 
              ON roles.id = users.role_id 
               AND users.id={}
            ",
            user_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<UserRoleAccess>(&query)
            .await
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> Result<UserId, String> {
        let query = format!(
            "SELECT users.id, email, password, 
             CASE WHEN role_id IS NULL 
              THEN (SELECT id FROM roles WHERE is_default=1) 
              ELSE roles.id END 
             AS role_id,
             CASE WHEN role_id IS NULL 
              THEN (SELECT name FROM roles WHERE is_default=1) 
              ELSE roles.id END 
             AS role_name
             FROM users
             LEFT JOIN roles ON roles.id=role_id
             WHERE users.id={}
            ",
            user_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<UserId>(&query)
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
