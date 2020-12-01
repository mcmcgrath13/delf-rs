use clap::Clap;

use delf;

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
            validate(&opts.schema, &opts.config);
        }
        SubCommand::Run => {
            println!("Starting delf api...");
            run(&opts.schema, &opts.config);
        }
    }
}

fn validate(schema_path: &String, config_path: &String) {
    let graph = delf::read_files(schema_path, config_path);
    graph.validate();
}

fn run(schema_path: &String, config_path: &String) {
    delf::check_short_ttl_loop(schema_path, config_path);
    delf::init_api(schema_path, config_path).launch();
}
