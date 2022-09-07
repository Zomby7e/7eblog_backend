/**
 * Objects(structs) that the service needs to use.
 */

use serde_derive::{Deserialize, Serialize};

// Query "/read"
#[derive(Deserialize, Serialize)]
pub struct ReadIDQuery<'a> {
    id: &'a str
}

impl<'a> ReadIDQuery<'a> {
    pub fn get_id(&self) -> &'a str{
        self.id
    }
}

// Query "/read/pagination"
#[derive(Deserialize, Serialize)]
pub struct ReadPaginationQuery {
    current_page: i32,
    page_size: i32
}

impl ReadPaginationQuery {
    pub fn get_current_page(&self) -> i32{
        self.current_page
    }
    pub fn get_page_size(&self) -> i32{
        self.page_size
    }
}

