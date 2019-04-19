extern crate csv;
extern crate diesel;
extern crate dol;
#[macro_use]
extern crate serde_derive;

use self::dol::*;
use self::models::NewDiscovery;
use std::io::stdin;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "ID")]
    uid: i32,
    #[serde(rename = "種別")]
    category: String,
    #[serde(rename = "発見物名")]
    name: String,
    #[serde(rename = "ランク")]
    level: Option<i32>,
    #[serde(rename = "説明")]
    description: String,
    #[serde(rename = "時代")]
    era: String,
    #[serde(rename = "ポイント")]
    points: Option<i32>,
    #[serde(rename = "難易度")]
    difficulty: Option<i32>,
    #[serde(rename = "経験値")]
    exp: Option<i32>,
    #[serde(rename = "備考")]
    note: String,
    #[serde(rename = "最終更新日時")]
    updated_at: String,
    #[serde(rename = "関連クエスト")]
    link: String,
}

fn main() -> Result<(), Box<csv::Error>> {
    let connection = establish_connection();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .delimiter(b'\t')
        .from_reader(stdin());
    let mut count = 0;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
        count += 1;

        let new_discovery = NewDiscovery {
            uid: record.uid,
            category: &record.category,
            name: &record.name,
            description: &record.description,
            level: record.level,
            points: record.points,
            era: Some(&record.era),
            difficulty: record.difficulty,
            exp: record.exp,
            note: Some(&record.note),
            link: Some(&record.link),
        };

        let discovery = create_discovery(&connection, new_discovery);
        println!(
            "\nSaved discovery {} with id {}",
            discovery.name, discovery.id
        );
    }

    println!("count: {}", count);

    Ok(())
}
