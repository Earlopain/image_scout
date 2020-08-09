use crate::schema::page_types;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use regex::Regex;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<i64, Regex>> = Mutex::new(HashMap::new());
}
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Queryable)]
pub struct PageType {
    pub id: i64,
    pub name: String,
    pub regex: String,
}

impl PageType {
    pub fn get_supported_pages(
        conn: &PgConnection,
    ) -> Result<Vec<PageType>, diesel::result::Error> {
        page_types::table.load(conn)
    }

    pub fn get_type_from_url(
        url: &String,
        conn: &PgConnection,
    ) -> Result<Option<i64>, diesel::result::Error> {
        let mut map = HASHMAP.lock().unwrap();
        for page_type in Self::get_supported_pages(conn)? {
            let matcher = match map.contains_key(&page_type.id) {
                true => map.get(&page_type.id),
                false => {
                    let created_regex = Regex::new(&page_type.regex).unwrap();
                    map.insert(page_type.id, created_regex);
                    map.get(&page_type.id)
                }
            };
            if matcher.unwrap().is_match(url) {
                return Ok(Some(page_type.id));
            }
        }
        Ok(None)
    }
}
