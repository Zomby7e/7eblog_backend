use rusqlite::{Connection, Result};
use crate::{QueryObject, ReadPaginationQueryObject};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Read {
    id: String,
    title: String,
    content: String,
    timestamp: i64,
    hashtag: Vec<Hashtag>
}

#[derive(Serialize, Deserialize)]
struct Hashtag{
    id: i64,
    name: String,
    comment: String
}

// Initialize the local sqlite database.
pub fn database_init(){
    // Open A connection to local sqlite database.
    let connection = Connection::open("main.db").unwrap();

    // Create "hashtags" database (If not exists).
    connection.execute(
        "CREATE TABLE IF NOT EXISTS \"hashtag\" (
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

    // Create "read_hashtag" database (If not exists).
    // This table is used to store the relationship between the hashtag table and the read table.
    connection.execute(
        "CREATE TABLE IF NOT EXISTS \"read_hashtag\" (
            \"id\"	INTEGER NOT NULL UNIQUE,
            \"read_id\"	TEXT NOT NULL,
            \"hashtag_id\"	INTEGER NOT NULL,
            PRIMARY KEY(\"id\" AUTOINCREMENT)
            );", []
    ).unwrap();
}

// get rows from "read" table.
pub fn database_get_read(query_object: QueryObject) -> Result<String, rusqlite::Error> {
    let connection = Connection::open("main.db")?;
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
                    hashtag: database_get_read_hashtag(row.get(0)?)?
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

// 查某文章的所有的标签(待重构)
fn database_get_read_hashtag (id: String) -> Result<Vec<Hashtag>, rusqlite::Error>{
    let connection = Connection::open("main.db")?;
    let sql_string: String = format!(
        "SELECT r.hashtag_id AS id, h.name, h.comment
        FROM read_hashtag AS r JOIN hashtag AS h ON hashtag_id = h.id
        WHERE read_id = \"{}\";", &id);
    let mut stmt = connection.prepare(&sql_string)?;
    let hashtags = stmt
    .query_map([], |row|
        Ok(
            crate::database::Hashtag {
                id: row.get(0)?,
                name: row.get(1)?,
                comment: row.get(2)?
            }
        )
    );
    let mut hashtag_vec: Vec<Hashtag> = Vec::new();
    for hashtag in hashtags? {
        match hashtag {
            Ok(hashtag_single) => {
                hashtag_vec.push(hashtag_single)
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(hashtag_vec)
}

// get rows from "read" table.
pub fn database_get_read_pagination(query_object: ReadPaginationQueryObject) -> Result<String, rusqlite::Error> {
    let connection = Connection::open("main.db")?;
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
                    hashtag: database_get_read_hashtag(row.get(0)?)?
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
    let result_str = serde_json::to_string(&read_vec).unwrap();
    Ok(result_str)
}