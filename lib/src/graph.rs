use std::collections::HashMap;

use petgraph::{Graph, Directed, Outgoing,
    graph:: {NodeIndex, EdgeIndex}
};
use yaml_rust::{Yaml};

#[derive(Clone, Debug)]
enum EdgeDeleteType {
    Deep,
    Shallow,
    RefCount
}

impl EdgeDeleteType {
    fn from(input: &str) -> EdgeDeleteType {
        match input.to_lowercase().as_str() {
            "deep" => EdgeDeleteType::Deep,
            "shallow" => EdgeDeleteType::Shallow,
            "refcount" => EdgeDeleteType::RefCount,
            _ => panic!("No edge type")
        }
    }
}

#[derive(Clone, Debug)]
enum ObjectDeleteType {
    ByAny,
    ShortTTL,
    Directly,
    DirectlyOnly,
    // ByXOnly(String),
    NotDeleted
}

impl ObjectDeleteType {
    fn from(input: &str) -> ObjectDeleteType {
        match input.to_lowercase().as_str() {
            "by_any" => ObjectDeleteType::ByAny,
            "short_ttl" => ObjectDeleteType::ShortTTL,
            "directly" => ObjectDeleteType::Directly,
            "directly_only" => ObjectDeleteType::DirectlyOnly,
            "by_x_only" => ObjectDeleteType::ByAny, // TODO: how to get X
            "not_deleted" => ObjectDeleteType::NotDeleted,
            _ => panic!("No Object type")
        }
    }
}

#[derive(Clone, Debug)]
pub struct DelfObject {
    name: String,
    storage: String,
    deletion: ObjectDeleteType,
    id_field: String,
    id_type: String
}

impl From<&Yaml> for DelfObject {
    fn from(obj: &Yaml) -> DelfObject {
        DelfObject {
            name: String::from(obj["name"].as_str().unwrap()),
            storage: String::from(obj["storage"].as_str().unwrap()),
            id_field: String::from(obj["id"]["field"].as_str().unwrap()),
            id_type: String::from(obj["id"]["type"].as_str().unwrap()),
            deletion: ObjectDeleteType::from(obj["deletion"].as_str().unwrap())
        }
    }
}

#[derive(Clone, Debug)]
pub struct DelfEdge {
    name: String,
    to: String,
    deletion: EdgeDeleteType,
    inverse: Option<EdgeIndex>
    // TODO: inverse edges
}

impl From<&Yaml> for DelfEdge {
    fn from(obj: &Yaml) -> DelfEdge {
        DelfEdge {
            name: String::from(obj["name"].as_str().unwrap()),
            to: String::from(obj["to"].as_str().unwrap()),
            deletion: EdgeDeleteType::from(obj["deletion"].as_str().unwrap()),
            inverse: None // gets updated later if needed
        }
    }
}

#[derive(Debug)]
pub struct DelfGraph {
    nodes: HashMap<String, NodeIndex>,
    edges: HashMap<String, EdgeIndex>,
    graph: Graph<DelfObject, DelfEdge, Directed>
}

impl DelfGraph {
    pub fn from(yamls: &Vec<Yaml>) -> DelfGraph {
        let mut edges_to_insert = Vec::new();
        let mut nodes = HashMap::<String, NodeIndex>::new();
        let mut edges = HashMap::<String, EdgeIndex>::new();
        let mut inverses = HashMap::<String, String>::new();

        let mut graph = Graph::<DelfObject, DelfEdge>::new();

        // each yaml is an object
        for yaml in yamls.iter() {
            let obj_name = String::from(yaml["object_type"]["name"].as_str().unwrap());
            let obj_node = DelfObject::from(&yaml["object_type"]);

            let node_id = graph.add_node(obj_node);
            nodes.insert(obj_name.clone(), node_id);

            // need to make sure all the nodes exist before edges can be added to the graph
            for edge in yaml["object_type"]["edge_types"].as_vec().unwrap().iter() {
                let delf_edge = DelfEdge::from(edge);
                match edge["inverse"].as_str() {
                    Some(edge_name) => {
                        inverses.insert(String::from(edge_name), String::from(&delf_edge.name));
                    },
                    None => ()
                }
                edges_to_insert.push((obj_name.clone(), delf_edge));
            }
        }

        // add all the edges to the graph
        for (from, edge) in edges_to_insert.iter_mut() {
            let edge_id = graph.add_edge(nodes[from], nodes[&edge.to], edge.clone());
            edges.insert(String::from(&edge.name), edge_id);
        }

        // update the edges with inverse edges to delete
        for (inverse_edge_name, edge_name) in inverses.iter_mut() {
            let edge_id = edges.get(edge_name).unwrap();
            let mut edge = graph.edge_weight_mut(*edge_id).unwrap();
            let inverse_edge_id = edges.get(inverse_edge_name).unwrap();
            edge.inverse = Some(*inverse_edge_id);
        }

        return DelfGraph {
            nodes,
            edges,
            graph
        }
    }

    pub fn print(&self) {
        println!("{:#?}", self.graph);
    }

    pub fn delete_edge(&self, edge_name: &String) {
        let edge_id = self.edges.get(edge_name).unwrap();
        let edge = self.graph.edge_weight(*edge_id).unwrap();
        self._delete_edge(edge)
    }

    fn _delete_edge(&self, edge: &DelfEdge) {
        println!("=======\ndeleting {:#?}", edge.name);
        match edge.deletion {
            EdgeDeleteType::Deep => {
                println!("    deep deletion, following to {}", edge.to);
                self.delete_object(&edge.to, Some(edge));
            },
            _ => println!("    shallow deletion, not deleting object")
        }

        match edge.inverse {
            Some(edge_id) => println!("    need to delete a reverse edge too!"),
            _ => ()
        }
    }

    pub fn delete_object(&self, object_name: &String, from_edge: Option<&DelfEdge>) {
        let object_id = self.nodes.get(object_name).unwrap();
        let object = self.graph.node_weight(*object_id).unwrap();
        println!("=======\nthinking about deleting {:#?}", object_name);
        let mut to_delete = false;
        match object.deletion {
            ObjectDeleteType::DirectlyOnly => {
                match from_edge {
                    Some(_) => println!("    not deleting, can only be deleted directly"),
                    None => {
                        println!("    directly_only satisfied");
                        to_delete = true;
                    }
                }
            },
            ObjectDeleteType::Directly | ObjectDeleteType::ShortTTL | ObjectDeleteType::ByAny => {
                println!("    delete away");
                to_delete = true;
            },
            ObjectDeleteType::NotDeleted => println!("    can't delete this"),
        }

        if to_delete {
            println!("    actually deleting!");
            let edges = self.graph.edges_directed(*object_id, Outgoing);
            for edge in edges {
                self._delete_edge(edge.weight());
            }
        }
    }
}
