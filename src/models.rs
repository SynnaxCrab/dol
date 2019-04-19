#[derive(Queryable)]
pub struct Discovery {
    pub id: i32,
    pub uid: i32,
    pub category: String,
    pub name: String,
    pub description: String,
    pub level: Option<i32>,
    pub points: Option<i32>,
    pub era: Option<String>,
    pub difficulty: Option<i32>,
    pub exp: Option<i32>,
    pub note: Option<String>,
    pub link: Option<String>,
}

use super::schema::discoveries;

#[derive(Insertable)]
#[table_name = "discoveries"]
#[derive(Default)]
pub struct NewDiscovery<'a> {
    pub uid: i32,
    pub category: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub level: Option<i32>,
    pub points: Option<i32>,
    pub era: Option<&'a str>,
    pub difficulty: Option<i32>,
    pub exp: Option<i32>,
    pub note: Option<&'a str>,
    pub link: Option<&'a str>,
}
