extern crate diesel
extern crate diesel_dynamic_schema

pub use self::DelfDatabaseConnection

pub struct DieselConnection {
    connection: diesel::connection::Connection
}

impl DelfDatabaseConnection for DieselConnection {

    pub fn connect(database_url: &str) -> ConnectionResult<Self> {
        let raw_connection = MysqlConnecion::establish(database_url);
        let conn = DieselConnection {
            connection: raw_connection
        }

        OK(conn);
    }

    pub fn delete_edge(&self, from: &DelfObject, to: &DelfObject, from_id: int32, to_id: int32, edge: &DelfEdge) {
        match edge.mapping_table {
            Some(map_table) => println!("{}", map_table), // delete the id pair from the mapping table
            None => println!("direct edge") // try to set null in object table
        }
    }
}
