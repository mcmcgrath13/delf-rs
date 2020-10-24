extern crate delf;

fn main() {
    let test_yaml = include_str!("scratch.yaml");

    let graph = delf::read_yaml(&test_yaml);

    graph.delete_object(&String::from("user"), None);
}
