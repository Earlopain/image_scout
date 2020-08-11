use crate::artist_page::ArtistPage;
use crate::artist_post::ArtistPost;
use crate::error::InsertImageFromUrlError;
use crate::provider::twitter::Twitter;
use chrono::{DateTime, Utc};
use diesel::PgConnection;
use reqwest::blocking::Response;

pub struct NewPosts {
    last_site_id: Option<String>,
    posts: Vec<RequiredApiFields>,
}

pub struct RequiredApiFields {
    source_url: String,
    direct_url: String,
    created_at: DateTime<Utc>,
}

pub trait ApiCrawler {
    fn get_new_posts(page: &ArtistPage) -> NewPosts;
    fn get_user_id(url: &str) -> String;
    fn make_authenticated_request(url: &str) -> reqwest::Result<Response>;
}

pub struct PostProvider {}

impl PostProvider {
    pub fn get_user_id(page_type: &i64, url: &str) -> String {
        match page_type {
            1 => Twitter::get_user_id(url),
            2 => "".to_string(),
            _ => panic!("Unimplemented Crawler! {}", url),
        }
    }

    pub fn insert_new_posts(
        page: &ArtistPage,
        conn: &PgConnection,
    ) -> Result<(), InsertImageFromUrlError> {
        let posts_to_insert = match page.id {
            1 => Twitter::get_new_posts(page),
            _ => panic!("Unimplemented Crawler!"),
        };

        for post in &posts_to_insert.posts {
            ArtistPost::create(
                &page.artist_id,
                &page.page_type,
                &post.source_url,
                &post.direct_url,
                &post.created_at,
                conn,
            )?;
        }
        page.update(posts_to_insert.last_site_id, conn)?;
        Ok(())
    }
}
