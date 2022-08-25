use rusqlite::{Connection, Result};
use crate::{QueryObject, ReadPaginationQueryObject};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Read {
    id: String,
    title: String,
    content: String,
    timestamp: i64,
    hashtag: String
}

// Initialize the local sqlite database.
pub fn database_init(){
    // Open A connection to local sqlite database.
    let connection = Connection::open("blog_main.db").unwrap();

    // Create "hashtags" database (If not exists).
    connection.execute(
        "CREATE TABLE IF NOT EXISTS \"hashtags\" (
        \"id\"	INTEGER NOT NULL UNIQUE,
        \"name\"	TEXT NOT NULL UNIQUE,
        \"comment\"	TEXT,
        PRIMARY KEY(\"id\" AUTOINCREMENT)
    );", []
    ).unwrap();

    // Create "note" database (If not exists).
    connection.execute(
        "CREATE TABLE IF NOT EXISTS \"note\" (
            \"id\"	TEXT NOT NULL UNIQUE,
            \"title\"	TEXT NOT NULL,
            \"content\"	TEXT NOT NULL,
            \"timestamp\"	INTEGER,
            \"hashtag\"	TEXT,
            PRIMARY KEY(\"id\")
        );", []
    ).unwrap();

    // Create "read" database (If not exists).
    connection.execute(
        "CREATE TABLE IF NOT EXISTS \"read\" (
            \"id\"	TEXT NOT NULL UNIQUE,
            \"title\"	TEXT NOT NULL,
            \"content\"	TEXT NOT NULL,
            \"timestamp\"	INTEGER,
            \"hashtag\"	TEXT,
            PRIMARY KEY(\"id\")
        );", []
    ).unwrap();

}

// get rows from "read" table.
pub fn database_get_read(query_object: QueryObject) -> Result<String, rusqlite::Error> {
    let connection = Connection::open("blog_main.db")?;
    let sql_string: String = String::from(format!("SELECT * FROM read WHERE id = \"{}\";", query_object.id));
    let mut stmt = connection.prepare(&sql_string)?;

    let reads = stmt
        .query_map([], |row|
            Ok(
                crate::database::Read {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    timestamp: row.get(3)?,
                    hashtag: row.get(4)?
                }
            )
        );
    let mut read_str = String::new();
    for read in reads? {
        match read {
            Ok(read1) => {
                read_str = serde_json::to_string(&read1).unwrap();
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(read_str)
}

// get rows from "read" table.
pub fn database_get_read_pagination(query_object: ReadPaginationQueryObject) -> Result<String, rusqlite::Error> {
    let connection = Connection::open("blog_main.db")?;
    let sql_string: String = String::from(format!("SELECT * FROM read LIMIT {} OFFSET {};", query_object.page_size, query_object.current_page));
    let mut stmt = connection.prepare(&sql_string)?;

    let reads = stmt
        .query_map([], |row|
            Ok(
                crate::database::Read {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    timestamp: row.get(3)?,
                    hashtag: row.get(4)?
                }
            )
        );
    let mut read_vec: Vec<Read> = Vec::new();
    for read in reads? {
        match read {
            Ok(read_single) => {
                read_vec.push(read_single)
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    let mut result_str = String::new();
    result_str = serde_json::to_string(&read_vec).unwrap();
    Ok(result_str)
}