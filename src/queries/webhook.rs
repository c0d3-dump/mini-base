use crate::database::model::ColType;

use super::{
    model::{Webhook, WebhookName},
    Model,
};

impl Model {
    pub async fn get_all_webhooks(&self) -> Result<Vec<WebhookName>, String> {
        let query = "SELECT id, name FROM webhooks ORDER BY id";

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<WebhookName>(query)
            .await
    }

    pub async fn get_webhook_by_id(&self, webhook_id: i64) -> Result<Webhook, String> {
        let query = format!(
            "SELECT id, name, exec_type, url, args FROM webhooks WHERE id={}",
            webhook_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<Webhook>(&query)
            .await
    }

    pub async fn add_new_webhook(&self, name: String) -> Result<i64, String> {
        let query = "INSERT INTO webhooks(name) VALUES (?) RETURNING id";
        let args = vec![ColType::String(Some(name))];

        let row = self.conn.as_ref().unwrap().query_one(query, args).await;

        match row {
            Ok(r) => Ok(r.get::<i64>(0).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub async fn edit_webhook(&self, w: Webhook) -> Result<u64, String> {
        let query = "UPDATE webhooks SET name=?, exec_type=?, url=?, args=? WHERE id=?";

        let args = vec![
            ColType::String(Some(w.name)),
            ColType::String(Some(w.exec_type)),
            ColType::String(Some(w.url)),
            ColType::Json(Some(w.args)),
            ColType::Integer(Some(w.id)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn delete_webhook(&self, webhook_id: i64) -> Result<u64, String> {
        let query = "DELETE FROM webhooks WHERE id=?";
        let args = vec![ColType::Integer(Some(webhook_id))];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn get_all_webhook_query_by_query_id(
        &self,
        query_id: i64,
    ) -> Result<Vec<Webhook>, String> {
        let query = format!(
            "SELECT id, name, exec_type, url, args FROM webhooks 
            INNER JOIN webhook_query ON webhooks.id = webhook_id
            WHERE query_id={}",
            query_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<Webhook>(&query)
            .await
    }
}
