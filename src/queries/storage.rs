use super::{model::Storage, Model};

impl Model {
    pub async fn get_all_storage(
        &self,
        search_term: &str,
        offset: i64,
    ) -> Result<Vec<Storage>, String> {
        let query = format!(
            "SELECT id, file_name, unique_name 
          FROM storage 
          WHERE id LIKE %{}% OR file_name LIKE %{}%
          ORDER BY id 
          LIMIT 25 
          OFFSET {}
      ",
            search_term, search_term, offset
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<Storage>(&query)
            .await
    }
}
