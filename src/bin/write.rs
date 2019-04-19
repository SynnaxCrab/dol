extern crate csv;
extern crate diesel;
extern crate dol;

use self::dol::*;
use self::models::NewDiscovery;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", name, EOF);
    let mut description = String::new();
    stdin().read_to_string(&mut description).unwrap();

    let new_discovery = NewDiscovery {
        uid: 12,
        category: "1",
        name: "test",
        description: "kkkk",
        ..Default::default()
    };

    let discovery = create_discovery(&connection, new_discovery);
    println!("\nSaved draft {} with id {}", name, discovery.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
