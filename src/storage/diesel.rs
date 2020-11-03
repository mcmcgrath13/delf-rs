use diesel::{
    Connection,
    mysql::{MysqlConnection}
};
// use diesel_dynamic_schema;

use crate::graph::{edge::DelfEdge, object::DelfObject};
pub use super::DelfStorageConnection;

pub struct DieselConnection {
    connection: MysqlConnection
}

impl std::fmt::Debug for DieselConnection {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "A DieselConnection")
    }
}

impl DelfStorageConnection for DieselConnection {

    fn connect(database_url: &str) -> DieselConnection {
        let raw_connection = MysqlConnection::establish(database_url);
        match raw_connection {
            Ok(conn) => DieselConnection {
                connection: conn
            },
            Err(_) => panic!("Couldn't connect to mysql")
        }
    }

    fn delete_edge(&self, from: &DelfObject, to: &DelfObject, from_id: i64, to_id: i64, edge: &DelfEdge) {
        match &edge.to.mapping_table {
            Some(map_table) => println!("deleted indirect edge {}!", map_table), // delete the id pair from the mapping table
            None => println!("deleted direct edge!") // try to set null in object table
        }
    }

    fn delete_object(&self, obj: &DelfObject, id: i64) {
        println!("deleted object!");
    }

    fn validate_edge(&self, edge: &DelfEdge) {
        println!("it's an edge!");
    }

    fn validate_object(&self, obj: &DelfObject) {
        println!("it's an object!");
    }
}
