use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::Clap;

use delf::read_yaml;

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "config.yaml")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap(short, long, default_value = "schema.yaml")]
    schema: String,
}

#[derive(Clap)]
enum SubCommand {
    Validate,
    Run,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);
    println!("Value for schema: {}", opts.schema);

    match opts.subcmd {
        SubCommand::Validate => {
            println!("Validating schema...");
            validate(&opts.config, &opts.schema);
        }
        SubCommand::Run => {
            println!("Starting delf api...");
        }
    }
}

fn validate(config_path: &String, schema_path: &String) {
    let config_str = read_file(config_path);
    let schema_str = read_file(schema_path);

    let graph = read_yaml(&schema_str, &config_str);
    graph.validate().unwrap();
    println!("Validation successful!")
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
