extern crate diesel
extern crate diesel_dynamic_schema

pub fn delete_edge(conn: &diesel::connection::Connection, from: &DelfObject, to: &DelfObject, from_id: int32, to_id: int32, edge: &DelfEdge) {
    match edge.mapping_table {
        Some(map_table) => println!("{}", map_table), // delete the id pair from the mapping table
        None => println!("direct edge") // try to set null in object table
    }
}
