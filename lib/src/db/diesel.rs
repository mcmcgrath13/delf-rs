extern crate diesel
extern crate diesel_dynamic_schema

pub use self::DelfStorageConnection

pub struct DieselConnection {
    connection: diesel::connection::Connection
}

impl DelfStorageConnection for DieselConnection {

    pub fn connect(database_url: &str) -> ConnectionResult<Self> {
        let raw_connection = MysqlConnecion::establish(database_url);
        let conn = DieselConnection {
            connection: raw_connection
        }

        OK(conn);
    }

    pub fn delete_edge(&self, from: &DelfObject, to: &DelfObject, from_id: int32, to_id: int32, edge: &DelfEdge) {
        match edge.mapping_table {
            Some(map_table) => println!("deleted indirect edge {}!", map_table), // delete the id pair from the mapping table
            None => println!("deleted direct edge!") // try to set null in object table
        }
    }

    pub fn delete_object(&self, obj: &DelfObject, id: int32) {
        println!("deleted object!");
    }

    pub fn validate_edge(&self, edge: &DelfEdge) {
        println!("it's an edge!");
    }

    pub fn validate_object(&self, obj: &DelfObject) {
        println!("it's an object!");
    }
}
