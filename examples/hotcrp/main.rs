extern crate delf;

fn main() {
    let test_schema = include_str!("schema.yaml");
    let test_config = include_str!("config.yaml");

    let graph = delf::read_yamls(&test_schema, &test_config);

    graph.validate().unwrap();

    let id = "48".to_string();

    graph.delete_object(&String::from("ContactInfo"), &id);
}
