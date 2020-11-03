use std::collections::HashMap;

use yaml_rust::{Yaml};

use super::edge::{ DelfEdge };
use crate::storage::{ DelfStorageConnection };

#[derive(Clone, Debug)]
pub enum DeleteType {
    ByAny,
    ShortTTL,
    Directly,
    DirectlyOnly,
    // ByXOnly(String),
    NotDeleted
}

impl DeleteType {
    fn from(input: &str) -> DeleteType {
        match input.to_lowercase().as_str() {
            "by_any" => DeleteType::ByAny,
            "short_ttl" => DeleteType::ShortTTL,
            "directly" => DeleteType::Directly,
            "directly_only" => DeleteType::DirectlyOnly,
            "by_x_only" => DeleteType::ByAny, // TODO: how to get X
            "not_deleted" => DeleteType::NotDeleted,
            _ => panic!("No Object type")
        }
    }
}

#[derive(Clone, Debug)]
pub struct DelfObject {
    pub name: String,
    pub storage: String,
    pub deletion: DeleteType,
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
            deletion: DeleteType::from(obj["deletion"].as_str().unwrap())
        }
    }
}

impl DelfObject {
    pub fn delete(&self, id: i64, from_edge: Option<&DelfEdge>, storages: &HashMap<String, Box<dyn DelfStorageConnection>>) -> bool {
        println!("=======\nthinking about deleting {:#?}", self.name);
        let mut to_delete = false;
        match self.deletion {
            DeleteType::DirectlyOnly => {
                match from_edge {
                    Some(_) => println!("    not deleting, can only be deleted directly"),
                    None => {
                        println!("    directly_only satisfied");
                        to_delete = true;
                    }
                }
            },
            DeleteType::Directly | DeleteType::ShortTTL | DeleteType::ByAny => {
                println!("    delete away");
                to_delete = true;
            },
            DeleteType::NotDeleted => println!("    can't delete this"),
        }

        if to_delete {
            println!("    actually deleting!");
            let s = &*(storages.get(&self.storage).unwrap());
            s.delete_object(self, id);
        }

        return to_delete;
    }
}
