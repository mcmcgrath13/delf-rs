use std::fmt::Debug;

use crate::graph::{edge::DelfEdge, object::DelfObject};

mod diesel;

pub trait DelfStorageConnection: Debug {
    fn connect(database_url: &str) -> Self
    where
        Self: Sized;

    fn get_object_ids(
        &self,
        from_id: &String,
        from_id_type: &String,
        edge_field: &String,
        table: &String,
        id_field: &String,
        id_type: &String,
    ) -> Vec<String>;

    fn get_object_ids_by_time(
        &self,
        table: &String,
        time_field: &String,
        id_field: &String,
        id_type: &String,
    ) -> Vec<String>;

    fn delete_edge(
        &self,
        to: &DelfObject,
        from_id: &String,
        to_id: Option<&String>,
        edge: &DelfEdge,
    ) -> bool;

    fn delete_object(&self, obj: &DelfObject, id: &String) -> bool;

    fn validate_edge(&self, edge: &DelfEdge) -> Result<(), String>;

    fn validate_object(&self, obj: &DelfObject) -> Result<(), String>;

    fn has_edge(&self, obj: &DelfObject, id: &String, edge: &DelfEdge) -> bool;
}

pub fn get_connection(plugin: &str, url: &str) -> Box<dyn DelfStorageConnection> {
    match plugin {
        "diesel" => Box::new(diesel::DieselConnection::connect(url)),
        _ => panic!("no DelfStorageConnection with that name"),
    }
}
