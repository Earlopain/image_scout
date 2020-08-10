use crate::artist_page::ArtistPage;
use crate::provider::twitter::Twitter;
use chrono::{DateTime, Utc};
use reqwest::blocking::Response;

pub struct RequiredApiFields {
    _source_url: String,
    _direct_url: String,
    _created_at: DateTime<Utc>,
}

pub trait ApiCrawler {
    fn get_new_posts(page: &ArtistPage) -> Vec<RequiredApiFields>;
    fn get_user_id(url: &str) -> Option<String>;
    fn get_api_identifier(page: &ArtistPage) -> &String;
    fn make_authenticated_request(url: &str) -> reqwest::Result<Response>;
}

pub struct PostProvider {}

impl PostProvider {
    pub fn get_user_id<'a>(page_type: &'a i64, url: &'a str) -> Option<String> {
        match page_type {
            1 => Twitter::get_user_id(url),
            2 => None,
            _ => panic!("Unimplemented Crawler! {}", url),
        }
    }
}
