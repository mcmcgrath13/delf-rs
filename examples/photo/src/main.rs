extern crate delf;

fn main() {
    let test_yaml = include_str!("scratch.yaml");

    delf::read_yaml(&test_yaml);
}
