use rocket::{delete, State, http::Status};

use crate::graph::DelfGraph;
use crate::DelfYamls;

/// `DELETE` API endpoint to delete an object of the given type with specified ID.
#[delete("/object/<object_type>/<id>")]
pub fn delete_object(object_type: String, id: String, yamls: State<DelfYamls>) -> Status {
    let graph = DelfGraph::new(&yamls);
    if !graph.nodes.contains_key(&object_type) {
        return Status::NotFound;
    }
    graph.delete_object(&object_type, &id);
    Status::Ok
}

/// `DELETE` API endpoint to delete an edge of the given type and the IDs of the objects it is connecting.
#[delete("/edge/<edge_type>/<from_id>/<to_id>")]
pub fn delete_edge(edge_type: String, from_id: String, to_id: String, yamls: State<DelfYamls>) -> Status {
    let graph = DelfGraph::new(&yamls);
    if !graph.edges.contains_key(&edge_type) {
        return Status::NotFound;
    }
    graph.delete_edge(&edge_type, &from_id, &to_id);
    Status::Ok
}
