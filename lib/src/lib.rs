// extern crate yaml_rust;

// use std::convert::TryFrom;

pub mod graph;

use yaml_rust::{YamlLoader};
// use yaml_validator::{Context, Validate};

pub fn read_yaml(str: &str) -> graph::DelfGraph {
    // let schema_str = include_str!("schema.yaml");
    // let schema_yaml = YamlLoader::load_from_str(&schema_str).unwrap();
    // let context = Context::try_from(&schema_yaml).unwrap();
    // let schema = context.get_schema("object_type").unwrap();
    let parsed = YamlLoader::load_from_str(str).unwrap();

    // schema.validate(&context, &doc).unwrap();
    // println!("validated the yaml");

    let delf_graph = graph::DelfGraph::from(&parsed);

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
