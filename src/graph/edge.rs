use yaml_rust::Yaml;

use crate::graph::DelfGraph;

#[derive(Clone, Debug)]
pub enum DeleteType {
    Deep,
    Shallow,
    RefCount,
}

impl DeleteType {
    fn from(input: &str) -> DeleteType {
        match input.to_lowercase().as_str() {
            "deep" => DeleteType::Deep,
            "shallow" => DeleteType::Shallow,
            "refcount" => DeleteType::RefCount,
            _ => panic!("No edge type"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ToType {
    pub object_type: String,
    pub field: String,
    pub mapping_table: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DelfEdge {
    pub name: String,
    pub to: ToType,
    pub deletion: DeleteType,
    pub inverse: Option<String>,
}

impl From<&Yaml> for DelfEdge {
    fn from(obj: &Yaml) -> DelfEdge {
        DelfEdge {
            name: String::from(obj["name"].as_str().unwrap()),
            to: ToType::from(&obj["to"]),
            deletion: DeleteType::from(obj["deletion"].as_str().unwrap()),
            inverse: match obj["inverse"].as_str() {
                Some(edge_name) => Some(String::from(edge_name)),
                None => None,
            }, // gets updated later if needed
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
                None => None,
            },
        }
    }
}

impl DelfEdge {
    pub fn delete_one(&self, from_id: i64, to_id: i64, graph: &DelfGraph) {
        println!("=======\ndeleting {:#?}", self.name);
        let to_obj = graph.get_object(&self.to.object_type);
        let s = &*(graph.storages.get(&to_obj.storage).unwrap());

        match self.deletion {
            DeleteType::Deep => {
                println!("    deep deletion, following to {}", self.to.object_type);
                graph._delete_object(&to_obj.name, to_id, Some(self));
            }
            _ => println!("    shallow deletion, not deleting object"), // TODO: refcount
        }

        match &self.inverse {
            Some(inverse) => {
                println!("    need to delete a reverse edge too!");
                graph.delete_edge(&inverse, to_id, from_id);
            }
            None => (),
        }

        s.delete_edge(to_obj, from_id, None, self);
    }

    pub fn delete_all(&self, from_id: i64, graph: &DelfGraph) {
        println!("=======\ndeleting {:#?}", self.name);
        let to_obj = graph.get_object(&self.to.object_type);
        let s = &*(graph.storages.get(&to_obj.storage).unwrap());

        match self.deletion {
            DeleteType::Deep => {
                println!("    deep deletion, following to {}", self.to.object_type);
                // collect object ids to delete
                let to_ids =
                    s.get_object_ids(from_id, &self.to.field, &to_obj.name, &to_obj.id_field);
                for to_id in to_ids.iter() {
                    graph._delete_object(&to_obj.name, *to_id, Some(self));
                }
            }
            _ => println!("    shallow deletion, not deleting object"), // TODO: refcount
        }

        match &self.inverse {
            Some(inverse) => {
                let table: &String;
                match &self.to.mapping_table {
                    Some(mapping_table) => {
                        table = &mapping_table;
                    }
                    None => {
                        table = &to_obj.name;
                    }
                }
                println!("    need to delete a reverse edge too!");
                // collect object ids to delete
                let to_ids = s.get_object_ids(from_id, &self.to.field, table, &to_obj.id_field);
                for to_id in to_ids.iter() {
                    graph.delete_edge(&inverse, *to_id, from_id);
                }
            }
            None => (),
        }

        s.delete_edge(to_obj, from_id, None, self);
    }

    pub fn validate(&self, graph: &DelfGraph) -> Result<(), String> {
        let to_obj = graph.get_object(&self.to.object_type);
        let s = &*(graph.storages.get(&to_obj.storage).unwrap());
        let res = s.validate_edge(self);
        return res;
    }
}
