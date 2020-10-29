use crate::graph::{DelfEdge, DelfObject};

pub trait DelfDatabaseConnection {
    fn connect(database_url: &str) -> DelfDatabaseConnection<Self>;

    fn delete_edge(&self, from: &DelfObject, to: &DelfObject, from_id: int32, to_id: int32, edge: &DelfEdge);

    fn delete_object(&self, obj: &DelfObject, id: int32);

    fn validate_object(&self, obj: &DelfObject);

    fn validate_edge(&self, edge: &DelfEdge);
}
