use std::collections::{HashMap, HashSet};

use yaml_rust::Yaml;

use super::edge::DelfEdge;
use crate::storage::DelfStorageConnection;

/// The deletion types for a DelfObject
#[derive(Clone, Debug)]
pub enum DeleteType {
    /// This object can be deleted by any incoming edge, but not directly
    ByAny,
    /// This object will be deleted after the specified period of time
    ShortTTL, // TODO: how to do this
    /// This object can be deleted by any incoming edge, as well as direclty
    Directly,
    /// This object can only be deleted direcly
    DirectlyOnly,
    /// This object can only be deleted by the edges listed in the vector
    ByXOnly(HashSet<String>),
    /// This object cannot be deleted
    NotDeleted,
}

impl DeleteType {
    fn from(input: &str, x_yaml: Option<&Vec<Yaml>>) -> DeleteType {
        match input.to_lowercase().as_str() {
            "by_any" => DeleteType::ByAny,
            "short_ttl" => DeleteType::ShortTTL,
            "directly" => DeleteType::Directly,
            "directly_only" => DeleteType::DirectlyOnly,
            "by_x_only" => {
                let mut x = HashSet::new();
                for x_str in x_yaml.unwrap().iter() {
                    x.insert(String::from(x_str.as_str().unwrap()));
                }
                DeleteType::ByXOnly(x)
            }
            "not_deleted" => DeleteType::NotDeleted,
            _ => panic!("No Object type"),
        }
    }
}

/// The DelfObject contains the information about the object as described in the schema
#[derive(Clone, Debug)]
pub struct DelfObject {
    pub name: String,
    pub storage: String,
    pub deletion: DeleteType,
    pub id_field: String,
    pub id_type: String,
}

impl From<&Yaml> for DelfObject {
    fn from(obj: &Yaml) -> DelfObject {
        DelfObject {
            name: String::from(obj["name"].as_str().unwrap()),
            storage: String::from(obj["storage"].as_str().unwrap()),
            id_field: String::from(obj["id"].as_str().unwrap()),
            id_type: match obj["id_type"].as_str() {
                Some(t) => t.to_string(),
                None => "integer".to_string(),
            },
            deletion: DeleteType::from(obj["deletion"].as_str().unwrap(), obj["x"].as_vec()),
        }
    }
}

impl DelfObject {
    /// Delete an instance of this object
    pub fn delete(
        &self,
        id: &String,
        from_edge: Option<&DelfEdge>,
        storages: &HashMap<String, Box<dyn DelfStorageConnection>>,
    ) -> bool {
        println!("=======\nthinking about deleting {:#?}", self.name);
        let mut to_delete = false;
        match from_edge {
            Some(edge) => match &self.deletion {
                DeleteType::ByAny | DeleteType::Directly => {
                    to_delete = true;
                }
                DeleteType::ByXOnly(x) => {
                    if x.contains(&edge.name) {
                        to_delete = true;
                    }
                }
                _ => (),
            },
            None => match &self.deletion {
                DeleteType::DirectlyOnly | DeleteType::Directly => {
                    to_delete = true;
                }
                _ => (),
            },
        }

        if to_delete {
            println!("    actually deleting!");
            let s = &*(storages.get(&self.storage).unwrap());
            return s.delete_object(self, id);
        }

        return false;
    }

    /// Validate the object exists in the storage as described in the schema
    pub fn validate(
        &self,
        storages: &HashMap<String, Box<dyn DelfStorageConnection>>,
    ) -> Result<(), String> {
        let s = &*(storages.get(&self.storage).unwrap());
        return s.validate_object(self);
    }
}
