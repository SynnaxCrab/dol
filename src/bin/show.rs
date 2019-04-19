extern crate diesel;
extern crate dol;

use self::diesel::prelude::*;
use self::dol::*;
use self::models::*;

fn main() {
    use dol::schema::discoveries::dsl::*;

    let connection = establish_connection();
    let results = discoveries
        .filter(category.eq("化石"))
        //.limit(5)
        .load::<Discovery>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} discoveries", results.len());
    for discovery in results {
        println!("{}", discovery.name);
        println!("{}", discovery.description);
        println!("{}", discovery.level.unwrap());
        println!("----------\n");
    }
}
