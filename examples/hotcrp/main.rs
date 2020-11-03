extern crate delf;

fn main() {
    let test_schema = include_str!("schema.yaml");
    let test_config = include_str!("config.yaml");

    let graph = delf::read_yaml(&test_schema, &test_config);

    graph.delete_object(&String::from("user"), 123);
}
