use crate::graph::{DelfEdge, DelfObject};

pub trait DelfStorageConnection {
    fn connect(database_url: &str) -> DelfStorageConnection<Self>;

    fn delete_edge(&self, from: &DelfObject, to: &DelfObject, from_id: int32, to_id: int32, edge: &DelfEdge);

    fn delete_object(&self, obj: &DelfObject, id: int32);

    fn validate_edge(&self, edge: &DelfEdge);

    fn validate_object(&self, obj: &DelfObject);
}

pub fn get_connection(plugin: &str, url: &str) {
    match plugin {
        "diesel" => diesel::DieselConnection::connect(url),
        _ => panic!("no DelfStorageConnection with that name")
    }
}
