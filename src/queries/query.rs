use crate::database::model::ColType;

use super::{
    model::{Query, QueryAccess, QueryName, QueryString, RoleAccess},
    Model,
};

impl Model {
    pub async fn get_all_queries(&self) -> Result<Vec<QueryName>, String> {
        let query = "SELECT id, name FROM queries ORDER BY id";

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<QueryName>(query)
            .await
    }

    pub async fn get_all_role_access_by_query_id(
        &self,
        query_id: i64,
    ) -> Result<Vec<RoleAccess>, String> {
        let query = format!(
            "SELECT role_id FROM role_access WHERE query_id={}",
            query_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<RoleAccess>(&query)
            .await
    }

    pub async fn get_all_apis(&self) -> Result<Vec<Query>, String> {
        let query = "SELECT id, name, exec_type FROM queries ORDER BY id";

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<Query>(query)
            .await
    }

    pub async fn get_query_by_id(&self, role_id: i64) -> Result<Query, String> {
        let query = format!(
            "SELECT id, name, exec_type 
             FROM queries 
             WHERE id={}",
            role_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<Query>(&query)
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

    pub async fn get_query_by_name(&self, query_name: &str) -> Result<Query, String> {
        let query = format!(
            "SELECT id, name, exec_type FROM queries WHERE name='{}'",
            query_name
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_one_with_type::<Query>(&query)
            .await
    }

    pub async fn add_new_query(&self, name: String) -> Result<i64, String> {
        let query = "INSERT INTO queries(name) VALUES (?) RETURNING id";
        let args = vec![ColType::String(Some(name))];

        let row = self.conn.as_ref().unwrap().query_one(query, args).await;

        match row {
            Ok(r) => Ok(r.get::<i64>(0).unwrap()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_query(&self, role_id: i64) -> Result<u64, String> {
        let query = "DELETE FROM queries WHERE id=?";
        let args = vec![ColType::Integer(Some(role_id))];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn get_query_access_by_id(&self, query_id: i64) -> Result<Vec<QueryAccess>, String> {
        let query = format!(
            "SELECT id, 
             name, 
             (SELECT TRUE FROM role_access WHERE role_id=roles.id AND query_id={}) AS has_access 
             FROM roles",
            query_id
        );

        self.conn
            .as_ref()
            .unwrap()
            .query_all_with_type::<QueryAccess>(&query)
            .await
    }

    pub async fn edit_query_string(
        &self,
        query_id: i64,
        query_string: String,
    ) -> Result<u64, String> {
        dbg!(&query_string);

        let query = "UPDATE queries SET query=? WHERE id=?";

        let args = vec![
            ColType::String(Some(query_string)),
            ColType::Integer(Some(query_id)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn edit_query(&self, q: Query) -> Result<u64, String> {
        let query = "UPDATE queries SET name=?, exec_type=? WHERE id=?";

        let args = vec![
            ColType::String(Some(q.name)),
            ColType::String(Some(q.exec_type)),
            ColType::Integer(Some(q.id)),
        ];

        self.conn.as_ref().unwrap().execute(query, args).await
    }

    pub async fn edit_query_access(
        &self,
        query_id: i64,
        query_access: Vec<QueryAccess>,
    ) -> Result<u64, String> {
        let old_access = self.get_query_access_by_id(query_id).await.unwrap();

        let (insertable, deletable) = remaining_ids(
            old_access
                .iter()
                .filter(|q| q.has_access)
                .map(|q| q.id)
                .collect(),
            query_access
                .iter()
                .filter(|q| q.has_access)
                .map(|q| q.id)
                .collect(),
        );

        for q in insertable {
            let query = "INSERT INTO role_access(role_id, query_id) VALUES(?, ?)";

            let args = vec![
                ColType::Integer(Some(q)),
                ColType::Integer(Some(query_id)),
            ];
            let res = self.conn.as_ref().unwrap().execute(query, args).await;
            match res {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        for q in deletable {
            let query = "DELETE FROM role_access WHERE role_id=? AND query_id=?";

            let args = vec![
                ColType::Integer(Some(q)),
                ColType::Integer(Some(query_id)),
            ];
            let res = self.conn.as_ref().unwrap().execute(query, args).await;
            match res {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(1)
    }
}

fn remaining_ids(arr1: Vec<i64>, arr2: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let mut i = 0;
    let mut j = 0;

    let len1 = arr1.len();
    let len2 = arr2.len();

    let mut insertable: Vec<i64> = vec![];
    let mut deletable: Vec<i64> = vec![];

    while i < len1 && j < len2 {
        if arr1[i] == arr2[j] {
            i += 1;
            j += 1;
        } else if arr1[i] < arr2[j] {
            deletable.push(arr1[i]);
            i += 1;
        } else {
            insertable.push(arr2[j]);
            j += 1;
        }
    }

    while i < len1 {
        deletable.push(arr1[i]);
        i += 1;
    }

    while j < len2 {
        insertable.push(arr2[j]);
        j += 1;
    }

    (insertable, deletable)
}

#[test]
fn test1() {
    let arr1 = vec![1, 2];
    let arr2 = vec![1, 3];

    let (insertable, deletable) = remaining_ids(arr1, arr2);

    assert_eq!(insertable, vec![3]);
    assert_eq!(deletable, vec![2]);
}
