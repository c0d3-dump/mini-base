use crate::database::model::ColType;

use super::{model::Storage, Model};

impl Model {
    pub async fn upload_file(
        &self,
        file_name: String,
        unique_name: String,
        uploaded_by: i64,
    ) -> Result<i64, String> {
        let query = "INSERT INTO storage(file_name, unique_name, uploaded_by) VALUES (?, ?, ?) returning id";

        let args = vec![
            ColType::String(Some(file_name)),
            ColType::String(Some(unique_name)),
            ColType::Integer(Some(uploaded_by)),
        ];

        let res = self.conn.as_ref().unwrap().query_one(query, args).await;
        match res {
            Ok(r) => Ok(r.get::<i64>(0).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_file(&self, file_id: i64) -> Result<u64, String> {
        let query = "DELETE FROM storage WHERE id=?";

        let args = vec![ColType::Integer(Some(file_id))];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn get_file_by_id(&self, file_id: i64) -> Result<Storage, String> {
        let query = format!(
            "SELECT id, file_name, unique_name FROM storage WHERE id={}",
            file_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<Storage>(&query)
            .await
    }
}
