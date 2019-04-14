extern crate diesel;
extern crate dol;
extern crate scraper;

use self::dol::*;

use std::fs;

use scraper::{Html, Selector};

fn main() {
    let connection = establish_connection();

    let mut contents =
        fs::read_to_string("../1.html").expect("Something went wrong reading the file");
    let fragment = Html::parse_fragment(&mut contents);

    let selector = Selector::parse("tbody").unwrap();
    let tbody = fragment.select(&selector).next().unwrap();

    let selector = Selector::parse("tr").unwrap();
    let mut count = 0;
    for node in tbody.select(&selector) {
        let texts = node.text().collect::<Vec<_>>();
        let mut texts: Vec<&str> = texts.into_iter().map(|x| x.trim()).collect();

        texts.retain(|&x| x != "");

        let name = texts[1];
        let jp_name = texts[0];
        //let description = vec![texts[7], texts[9], texts[11]].join("");
        let description = vec![texts[7], texts[9]].join("");
        let level = texts[2].chars().count() as i32;

        //println!("{:?}", texts);
        println!("name: {}", name);
        println!("jp_name: {}", jp_name);
        println!("description: {}", description);
        println!("level: {}", level);
        //println!("{:?}", node.value());

        let discovery = create_discovery(&connection, name, jp_name, &description, level);
        println!("\nSaved draft {} with id {}", name, discovery.id);
        count += 1;
    }
    println!("count: {}", count);
}
