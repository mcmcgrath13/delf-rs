// extern crate yaml_rust;

// use std::convert::TryFrom;

pub mod graph;

use yaml_rust::{YamlLoader};
// use yaml_validator::{Context, Validate};

pub fn read_yaml(schema_str: &str, config_str: &str) -> graph::DelfGraph {
    let schema_parsed = YamlLoader::load_from_str(schema_str).unwrap();
    let config_parsed = YamlLoader::load_from_str(config_str).unwrap();

    let delf_graph = graph::DelfGraph::from(&schema_parsed, &config_parsed);

    delf_graph.print();

    return delf_graph;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
