//! # DeLF
//! A [DelF](https://cs.brown.edu/courses/csci2390/2020/readings/delf.pdf) inspired deletion framework in Rust.

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// The graph module contains the core structures to run the deletion algorithms.
pub mod graph;
///The storage module contains plugins for storage-specific (e.g. mysql) deletion implementations.
pub mod storage;

use yaml_rust::YamlLoader;

/// Read in the schema and config yaml files to build the delf graph.
pub fn read_files(schema_path: &String, config_path: &String) -> graph::DelfGraph {
    let config_str = read_file(config_path);
    let schema_str = read_file(schema_path);

    return read_yamls(&schema_str, &config_str);
}

/// Read in the schema and config yaml strings to build the delf graph.
pub fn read_yamls(schema_str: &str, config_str: &str) -> graph::DelfGraph {
    let schema_parsed = YamlLoader::load_from_str(schema_str).unwrap();
    let config_parsed = YamlLoader::load_from_str(config_str).unwrap();

    return graph::DelfGraph::new(&schema_parsed, &config_parsed);
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
