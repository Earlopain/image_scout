use crate::artist_page::ArtistPage;
use crate::provider::post_provider::{ApiCrawler, RequiredApiFields};
use reqwest::blocking::Response;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;

const BASE: &'static str = "https://api.twitter.com/1.1/";

#[derive(Deserialize)]
struct TwitterUser {
    #[serde(rename(deserialize = "id_str"))]
    id: String,
}

pub struct Twitter {}

impl ApiCrawler for Twitter {
    fn get_new_posts(_page: &ArtistPage) -> Vec<RequiredApiFields> {
        unimplemented!()
    }

    //TODO error handling
    fn get_user_id(url: &str) -> String {
        let username = &url[20..url.len() - 1];
        let response = Self::make_authenticated_request(
            &(BASE.to_owned() + "users/lookup.json?screen_name=" + username),
        );
        let text = response.unwrap().text().unwrap();
        //TODO check if the result vec is empty
        let mut users: Vec<TwitterUser> = serde_json::from_str(&text).unwrap();
        users.remove(0).id
    }

    fn make_authenticated_request(url: &str) -> reqwest::Result<Response> {
        reqwest::blocking::Client::new()
            .get(url)
            .header(
                AUTHORIZATION,
                "Bearer ".to_owned()
                    + &std::env::var("TWITTER_BEARER").expect("TODO: Conditional provider"),
            )
            .send()
    }
}
