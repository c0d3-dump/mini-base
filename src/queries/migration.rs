use crate::database::model::ColType;

use super::{
    model::{Migration, MigrationDown, MigrationName, MigrationUp},
    Model,
};

impl Model {
    pub async fn get_all_migrations(&self) -> Result<Vec<MigrationName>, String> {
        let query = "SELECT id, name, executed FROM migrations ORDER BY id";

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<MigrationName>(query)
            .await
    }

    pub async fn get_migration_name_by_id(
        &self,
        migration_id: i64,
    ) -> Result<MigrationName, String> {
        let query = format!(
            "SELECT id, name, executed FROM migrations WHERE id={}",
            migration_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<MigrationName>(&query)
            .await
    }

    pub async fn get_up_migration_by_id(&self, migration_id: i64) -> Result<MigrationUp, String> {
        let query = format!(
            "SELECT id, up_query FROM migrations WHERE id={}",
            migration_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<MigrationUp>(&query)
            .await
    }

    pub async fn get_up_migrations(&self) -> Result<Vec<MigrationUp>, String> {
        let query = "SELECT id, up_query FROM migrations WHERE executed=0 ORDER BY id ASC";

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<MigrationUp>(query)
            .await
    }

    pub async fn get_down_migration_by_id(
        &self,
        migration_id: i64,
    ) -> Result<MigrationDown, String> {
        let query = format!(
            "SELECT id, down_query FROM migrations WHERE id={}",
            migration_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<MigrationDown>(&query)
            .await
    }

    pub async fn get_down_migrations(&self) -> Result<Vec<MigrationDown>, String> {
        let query = "SELECT id, down_query FROM migrations WHERE executed=1 ORDER BY id DESC";

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<MigrationDown>(query)
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

    pub async fn edit_up_migration(&self, migration: Migration) -> Result<u64, String> {
        let query = "UPDATE migrations
                  SET up_query=?
                  WHERE id=?";
        let args = vec![
            ColType::String(Some(migration.up_query)),
            ColType::Integer(Some(migration.id)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn edit_down_migration(&self, migration: Migration) -> Result<u64, String> {
        let query = "UPDATE migrations SET down_query=? WHERE id=?";
        let args = vec![
            ColType::String(Some(migration.down_query)),
            ColType::Integer(Some(migration.id)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn run_migration(&self, query: String) -> Result<u64, String> {
        self.conn.as_ref().unwrap().execute(&query, vec![]).await
    }

    pub async fn update_executed_migration(
        &self,
        migration_id: i64,
        executed: bool,
    ) -> Result<u64, String> {
        let query = "UPDATE migrations SET executed=? WHERE id=?";
        let args = vec![
            ColType::Bool(Some(executed)),
            ColType::Integer(Some(migration_id)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn delete_migration(&self, migration_id: i64) -> Result<u64, String> {
        let query = "DELETE FROM migrations WHERE id=?";
        let args = vec![ColType::Integer(Some(migration_id))];

        self.conn.as_ref().unwrap().execute(query, args).await
    }
}
