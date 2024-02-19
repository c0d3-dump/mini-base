use crate::database::model::ColType;

use super::{
    model::{Migration, MigrationName},
    Model,
};

impl Model {
    pub async fn get_all_migrations(&self) -> Result<Vec<MigrationName>, String> {
        let query = "SELECT id, name FROM migrations ORDER BY id";

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<MigrationName>(query)
            .await
    }

    pub async fn get_migration_by_id(&self, migration_id: i64) -> Result<Migration, String> {
        let query = format!(
            "SELECT id, name, is_default, can_read, can_write, can_delete
             FROM migrations
             WHERE id={}",
            migration_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<Migration>(&query)
            .await
    }

    pub async fn add_new_migration(&self, name: String) -> Result<i64, String> {
        let query = "INSERT INTO migrations(name) VALUES (?) RETURNING id";
        let args = vec![ColType::String(Some(name))];

        let row = self.conn.as_ref().unwrap().query_one(query, args).await;

        match row {
            Ok(r) => Ok(r.get::<i64>(0).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub async fn edit_migration(&self, migration: Migration) -> Result<u64, String> {
        let query = "UPDATE migrations
                    SET up_query=?, down_query=?
                    WHERE id=?";
        let args = vec![
            ColType::String(Some(migration.up_query)),
            ColType::String(Some(migration.down_query)),
            ColType::Integer(Some(migration.id)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn delete_migration(&self, migration_id: i64) -> Result<u64, String> {
        let query = "DELETE FROM migrations WHERE id=?";
        let args = vec![ColType::Integer(Some(migration_id))];

        self.conn.as_ref().unwrap().execute(query, args).await
    }
}
