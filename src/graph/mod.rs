use std::collections::HashMap;

use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    Directed, Graph, Outgoing,
};
use yaml_rust::Yaml;

/// The edge of a DelfGraph is a DelfEdge
pub mod edge;
/// The node of a DelfGraph is a DelfObject
pub mod object;

use crate::storage::{get_connection, DelfStorageConnection};

/// The DelfGraph is the core structure for delf's functionality.  It contains the algorithm to traverse the graph, as well as metadata to perform the deletions.
#[derive(Debug)]
pub struct DelfGraph {
    nodes: HashMap<String, NodeIndex>,
    edges: HashMap<String, EdgeIndex>,
    graph: Graph<object::DelfObject, edge::DelfEdge, Directed>,
    storages: HashMap<String, Box<dyn DelfStorageConnection>>,
}

impl DelfGraph {
    /// Create a new DelfGraph from a schema and a config.  See [yaml_rust](../../yaml_rust/index.html) for information on creating the Yaml structs, or alternately use the helper functions: [read_files](../fn.read_files.html), [read_yamls](../fn.read_yamls.html) for constructing a DelfGraph from either paths or `&str` of yaml.
    pub fn new(schema: &Vec<Yaml>, config: &Vec<Yaml>) -> DelfGraph {
        let mut edges_to_insert = Vec::new();
        let mut nodes = HashMap::<String, NodeIndex>::new();
        let mut edges = HashMap::<String, EdgeIndex>::new();

        let mut graph = Graph::<object::DelfObject, edge::DelfEdge>::new();

        // each yaml is an object
        for yaml in schema.iter() {
            let obj_name = String::from(yaml["object_type"]["name"].as_str().unwrap());
            let obj_node = object::DelfObject::from(&yaml["object_type"]);

            let node_id = graph.add_node(obj_node);
            nodes.insert(obj_name.clone(), node_id);

            // need to make sure all the nodes exist before edges can be added to the graph
            for e in yaml["object_type"]["edge_types"].as_vec().unwrap().iter() {
                let delf_edge = edge::DelfEdge::from(e);
                edges_to_insert.push((obj_name.clone(), delf_edge));
            }
        }

        // add all the edges to the graph
        for (from, e) in edges_to_insert.iter_mut() {
            let edge_id = graph.add_edge(nodes[from], nodes[&e.to.object_type], e.clone());
            edges.insert(String::from(&e.name), edge_id);
        }

        // create the storage map
        let mut storages = HashMap::<String, Box<dyn DelfStorageConnection>>::new();

        for yaml in config.iter() {
            for storage in yaml["storages"].as_vec().unwrap().iter() {
                let storage_name = String::from(storage["name"].as_str().unwrap());
                storages.insert(
                    storage_name,
                    get_connection(
                        storage["plugin"].as_str().unwrap(),
                        storage["url"].as_str().unwrap(),
                    ),
                );
            }
        }

        return DelfGraph {
            nodes,
            edges,
            graph,
            storages,
        };
    }

    /// Pretty print the graph's contents.
    pub fn print(&self) {
        println!("{:#?}", self.graph);
    }

    /// Given an edge name, get the corresponding DelfEdge
    pub fn get_edge(&self, edge_name: &String) -> &edge::DelfEdge {
        let edge_id = self.edges.get(edge_name).unwrap();
        return self.graph.edge_weight(*edge_id).unwrap();
    }

    /// Given an edge name and the ids of the to/from object instances, delete the edge
    pub fn delete_edge(&self, edge_name: &String, from_id: &String, to_id: &String) {
        let e = self.get_edge(edge_name);
        e.delete_one(from_id, to_id, self);
    }

    /// Given an object name, get the corresponding DelfObject
    pub fn get_object(&self, object_name: &String) -> &object::DelfObject {
        let object_id = self.nodes.get(object_name).unwrap();
        return self.graph.node_weight(*object_id).unwrap();
    }

    /// Given the object name and the id of the instance, delete the object
    pub fn delete_object(&self, object_name: &String, id: &String) {
        self._delete_object(object_name, id, None);
    }

    fn _delete_object(
        &self,
        object_name: &String,
        id: &String,
        from_edge: Option<&edge::DelfEdge>,
    ) {
        let obj = self.get_object(object_name);

        let deleted = obj.delete(id, from_edge, &self.storages);

        if deleted {
            let edges = self.graph.edges_directed(self.nodes[&obj.name], Outgoing);
            for e in edges {
                e.weight().delete_all(id, &obj.id_type, self);
            }
        }
    }

    /// Validate that the objects and edges described in the schema exist in the corresponding storage as expected
    pub fn validate(&self) -> Result<(), String> {
        for (_, node_id) in self.nodes.iter() {
            self.graph
                .node_weight(*node_id)
                .unwrap()
                .validate(&self.storages)?;
        }

        for (_, edge_id) in self.edges.iter() {
            self.graph.edge_weight(*edge_id).unwrap().validate(self)?;
        }
        return Ok(());
    }
}
