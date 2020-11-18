//! # DeLF
//! A [DelF](https://cs.brown.edu/courses/csci2390/2020/readings/delf.pdf) inspired deletion framework in Rust.

#![feature(proc_macro_hygiene, decl_macro)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// The api module contains routes to dispatch deletes on objects and edges
pub mod api;
/// The graph module contains the core structures to run the deletion algorithms.
pub mod graph;
///The storage module contains plugins for storage-specific (e.g. mysql) deletion implementations.
pub mod storage;

use yaml_rust::{Yaml, YamlLoader};
extern crate rocket;

pub struct DelfYamls {
    pub config: Vec<Yaml>,
    pub schema: Vec<Yaml>,
}

/// Read in the schema and config yaml files to build the delf graph.
pub fn read_files(schema_path: &String, config_path: &String) -> graph::DelfGraph {
    let yamls = parse_files(schema_path, config_path);

    return graph::DelfGraph::new(&yamls);
}

fn parse_files(schema_path: &String, config_path: &String) -> DelfYamls {
    let schema_str = read_file(schema_path);
    let config_str = read_file(config_path);

    return parse_yaml(&schema_str, &config_str);
}

fn parse_yaml(schema_str: &str, config_str: &str) -> DelfYamls {
    let schema_parsed = YamlLoader::load_from_str(schema_str).unwrap();
    let config_parsed = YamlLoader::load_from_str(config_str).unwrap();

    DelfYamls {
        config: config_parsed,
        schema: schema_parsed,
    }
}

/// Read in the schema and config yaml strings to build the delf graph.
pub fn read_yamls(schema_str: &str, config_str: &str) -> graph::DelfGraph {
    let yamls = parse_yaml(schema_str, config_str);

    return graph::DelfGraph::new(&yamls);
}

fn read_file(file_name: &String) -> String {
    let path = Path::new(file_name);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", file_name, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", file_name, why),
        Ok(_) => print!("{} contains:\n{}", file_name, s),
    }

    return s;
}

/// Initialize the rocket api and return the struct.  Run the `launch` method on the returned value to start the api.
pub fn init_api(schema_path: &String, config_path: &String) -> rocket::Rocket {
    let yamls = parse_files(schema_path, config_path);
    rocket::ignite()
        .mount("/", rocket::routes![api::delete_object, api::delete_edge])
        .manage(yamls)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
