use yaml_rust::{Yaml};

use petgraph::graph::{EdgeIndex};

#[derive(Clone, Debug)]
pub enum DeleteType {
    Deep,
    Shallow,
    RefCount
}

impl DeleteType {
    fn from(input: &str) -> DeleteType {
        match input.to_lowercase().as_str() {
            "deep" => DeleteType::Deep,
            "shallow" => DeleteType::Shallow,
            "refcount" => DeleteType::RefCount,
            _ => panic!("No edge type")
        }
    }
}

#[derive(Clone, Debug)]
pub struct ToType {
    pub object_type: String,
    field: String,
    pub mapping_table: Option<String>
}

#[derive(Clone, Debug)]
pub struct DelfEdge {
    pub name: String,
    pub to: ToType,
    pub deletion: DeleteType,
    pub inverse: Option<EdgeIndex>
}

impl From<&Yaml> for DelfEdge {
    fn from(obj: &Yaml) -> DelfEdge {
        DelfEdge {
            name: String::from(obj["name"].as_str().unwrap()),
            to: ToType::from(&obj["to"]),
            deletion: DeleteType::from(obj["deletion"].as_str().unwrap()),
            inverse: None // gets updated later if needed
        }
    }
}

impl From<&Yaml> for ToType {
    fn from(obj: &Yaml) -> ToType {
        ToType {
            object_type: String::from(obj["object_type"].as_str().unwrap()),
            field: String::from(obj["field"].as_str().unwrap()),
            mapping_table: match obj["mapping_table"].as_str() {
                Some(table) => Some(String::from(table)),
                None => None
            }
        }
    }
}
