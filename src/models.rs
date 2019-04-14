#[derive(Queryable)]
pub struct Discovery {
    pub id: i32,
    pub name: String,
    pub jp_name: String,
    pub description: String,
    pub level: i32,
}

use super::schema::discoveries;

#[derive(Insertable)]
#[table_name = "discoveries"]
pub struct NewDiscovery<'a> {
    pub name: &'a str,
    pub jp_name: &'a str,
    pub description: &'a str,
    pub level: i32,
}
