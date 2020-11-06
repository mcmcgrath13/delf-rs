use diesel;
use diesel::sql_types::Integer;
use diesel::Connection;
use diesel::QueryableByName;
use diesel::RunQueryDsl;

pub use super::DelfStorageConnection;
use crate::graph::{edge::DelfEdge, object::DelfObject};

pub struct DieselConnection {
    connection: diesel::mysql::MysqlConnection,
}

impl std::fmt::Debug for DieselConnection {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "A DieselConnection")
    }
}

#[derive(QueryableByName)]
struct ObjectIdResult {
    #[sql_type = "Integer"]
    id_field: i32,
}

impl DelfStorageConnection for DieselConnection {
    fn connect(database_url: &str) -> DieselConnection {
        let raw_connection = diesel::mysql::MysqlConnection::establish(database_url);
        match raw_connection {
            Ok(conn) => DieselConnection { connection: conn },
            Err(_) => panic!("failed to connect to mysql"),
        }
    }

    fn get_object_ids(
        &self,
        from_id: i64,
        edge_field: &String,
        table: &String,
        id_field: &String,
    ) -> Vec<i64> {
        let query = diesel::sql_query(format!(
            "SELECT {} as id_field FROM {} WHERE {} = {}",
            id_field, table, edge_field, from_id
        ))
        .load::<ObjectIdResult>(&self.connection);

        let mut obj_ids = Vec::new();
        let res = query.unwrap();
        for o_id in res {
            obj_ids.push(o_id.id_field as i64);
        }

        return obj_ids;
    }

    fn delete_edge(&self, to: &DelfObject, from_id: i64, to_id: Option<i64>, edge: &DelfEdge) {
        match &edge.to.mapping_table {
            Some(map_table) => self.delete_indirect_edge(edge, to, from_id, to_id, map_table), // delete the id pair from the mapping table
            None => self.delete_direct_edge(to, from_id, edge), // try to set null in object table
        }
    }

    fn delete_object(&self, obj: &DelfObject, id: i64) {
        diesel::sql_query(format!(
            "DELETE FROM {} WHERE {} = {}",
            obj.table(),
            obj.key(),
            id
        ))
        .execute(&self.connection)
        .unwrap();
        println!("deleted object!");
    }

    fn validate_edge(&self, edge: &DelfEdge) {
        println!("it's an edge!");
    }

    fn validate_object(&self, obj: &DelfObject) {
        println!("it's an object!");
    }
}

impl DieselConnection {
    fn delete_indirect_edge(
        &self,
        edge: &DelfEdge,
        to: &DelfObject,
        from_id: i64,
        to_id: Option<i64>,
        table: &String,
    ) {
        match to_id {
            Some(id) => {
                diesel::sql_query(format!(
                    "DELETE FROM {} WHERE {} = {} AND {} = {}",
                    table,
                    to.key(),
                    id,
                    edge.to.field,
                    from_id
                ))
                .execute(&self.connection)
                .unwrap();
            }
            None => {
                diesel::sql_query(format!(
                    "DELETE FROM {} WHERE {} = {}",
                    table, edge.to.field, from_id
                ))
                .execute(&self.connection)
                .unwrap();
            }
        }

        println!("deleted indirect edge {}!", table);
    }

    fn delete_direct_edge(&self, to: &DelfObject, from_id: i64, edge: &DelfEdge) {
        diesel::sql_query(format!(
            "UPDATE {} SET {} = NULL WHERE {} = {}",
            to.table(),
            edge.to.field,
            edge.to.field,
            from_id
        ))
        .execute(&self.connection)
        .unwrap();
        println!("deleted direct edge!");
    }
}
