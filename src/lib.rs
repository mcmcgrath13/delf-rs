//! # DeLF
//! A [DelF](https://cs.brown.edu/courses/csci2390/2020/readings/delf.pdf) inspired deletion framework in Rust.
//!
//! The goal of DelF is to ensure that when an item is deleted (such as a user in a social network), all of their related data is also deleted (photos, friendships, posts, etc.).  DelF achieves this by defining the backing storage as a graph then using methods on that graph to delete the object/edge.  DelF will then traverse the graph ensuring deletion of all related objects/edges.
//!
//! ## DelF Data Definition Language (DDL)
//!
//! The DelF DDL enables the functionality within the delf crate.  A DelF schema is a YAML file containing definitions of objects and their outbound edges.  The DDL is as follows:
//!
//! ```yaml
//! --- # the triple line indicates a new document
//! # root key of the document
//! object_type:
//!
//!   # The name of the object, this is a unique identifier for this object, such as a table name
//!   # in a relational database
//!   name: MyObjectName
//!
//!   # The storage is the name of a storage in the config file
//!   storage: my_storage
//!
//!   # The deletion type of the object - see delf::graph::object::DeleteType for details
//!   deletion: by_any | by_x_only | short_ttl | directly | directly_only | not_deleted
//!
//!   # The field or attribute of the object that holds a unique identifier for each instance
//!   id: 123
//!
//!   # If the deletion specified is `by_x_only`, provide an array of inbound edge names that can
//!   # delete this object
//!   x:
//!     - my_edge
//!     - my_other_edge
//!
//!   # If the deletion specified is `short_ttl`, specify the field or attribute holding the time
//!   # an instance of the object should be deleted
//!   time_field: my_delete_time
//!
//!   # Definitions of outbound edges from this object to other objects.  If no edges exists, can
//!   # pass an empty array (`[]`)
//!   edge_types:
//!
//!     # A unique name for the edge that can be referenced from other definitions and to directly
//!     # delete an edge directly
//!     - name: edge_to_somewhere
//!
//!       # The deletion type of the edge - see delf::graph::edge::DeleteType for details
//!       deletion: shallow | deep | refcount
//!
//!       # An inverse edge may be specified, typically with a deletion type of `shallow`, to
//!       # delete an edge in the opposite direction of this edge when this edge is deleted
//!       inverse: edge_to_here
//!
//!       # Fields describing the object this edge points to (from the edge's point of view)
//!       to:
//!
//!         # The name of the object_type this edge points to
//!         object_type: MyOtherObject
//!
//!         # The name of the field/attribute the id of this object references on the to object
//!         field: my_object_name_id
//!
//!         # Optionally, if there's an intermediate mapping table between the two objects, (e.g.
//!         # user -> user_photos -> photos ) - more common in relational databases
//!         mapping_table: this_2_other
//! ```
//!
//! ## Configuration
//!
//! A configuration YAML file is also required to use delf.  It contains information on the storage infrastructure delf will be acting on (such as how to connect to a database).  The format is as follows:
//!
//! ```yaml
//! # A list of the storage infrastruture delf is acting on
//! storages:
//!
//!   # The name of the storage as referenced in the DDL
//!   - name: my_storage
//!
//!     # The name of the plugin (see delf::storage::DelfStorageConnection) this storage uses
//!     plugin: diesel (other plugins may be implemented in the future)
//!
//!     # A connection string for the storage
//!     url: mysql://username:password@localhost:port/database_name
//! ```
//!
//! ## Command Line Interface (CLI)
//!
//! The DelF CLI provides two commands: `validate` and `run`.
//!
//! ### Validate
//!
//! The `validate` command checks that all of the objects defined in the schema exist as described in the storage and also that all objects in the delf graph are reachable by traversing the graph starting from a directly deletable object then following edges that can delete the connecting object.  This ensures that all objects are deletable (or excepted from deletion by using the `not_deleted` type).
//!
//! #### Example
//!
//! ```yaml
//! delf -s path/to/schema.yaml -c path/to/config.yaml validate
//! ```
//!
//! The executable will either successfully complete, or panic at the first issue it finds.
//!
//! ### Run
//!
//! The `run` command starts the api running locally and starts a thread which checks `short_ttl` deleted objects for deletable instances every 30 seconds.
//!
//! #### Example
//! ```yaml
//! delf -s path/to/schema.yaml -c path/to/config.yaml run
//! ```
//!
//! The executable will run until terminated.

#![feature(proc_macro_hygiene, decl_macro)]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;
use std::time::Duration;

/// The api module contains routes to dispatch deletes on objects and edges.
///
/// The two routes the api uses are for deletion of objects and edges:
///
/// object: `DELETE /object/<object_type>/<id>`
/// edge: `DELETE /edge/<edge_type>/<from_id>/<to_id>`
///
/// # Example
///
/// object: `DELETE /object/users/123`
/// edge: `DELETE /edge/user_photo/123/my_photo456`
pub mod api;

/// The graph module contains the core structures to run the deletion algorithms.
///
/// The graph is constructed from two yaml files:
/// * A schema defined using the DelF DDL (data definition language)
/// * A configuration file defining which storages exist and how to connect to them
pub mod graph;

///The storage module contains plugins for storage-specific (e.g. mysql) deletion implementations.
pub mod storage;

use yaml_rust::{Yaml, YamlLoader};
extern crate rocket;

pub(crate) struct DelfYamls {
    pub config: Vec<Yaml>,
    pub schema: Vec<Yaml>,
}

/// Read in the schema and config yaml files to construct the delf graph.
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

/// Read in the schema and config yaml strings to construct the delf graph.
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

/// Spawn a thread that checks short time to live objects every 30 seconds to evaluate for deletion
pub fn check_short_ttl_loop(schema_path: &String, config_path: &String) {
    let s_path = schema_path.clone();
    let c_path = config_path.clone();
    thread::spawn(move || {
        let graph = read_files(&s_path, &c_path);
        let sleep_duration = Duration::from_secs(30);
        loop {
            thread::sleep(sleep_duration);
            graph.check_short_ttl();
        }
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
