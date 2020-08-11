use crate::artist_page::ArtistPage;
use crate::provider::post_provider::{ApiCrawler, NewPosts, RequiredApiFields};
use reqwest::blocking::Response;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TwitterPost {
    pub id_str: String,
    pub user: User,
    pub extended_entities: Option<ExtendedEntities>,
    pub retweeted_status: Option<Dummy>,
    pub retweeted: bool,
}

#[derive(Deserialize)]
pub struct Dummy {}

#[derive(Deserialize)]
pub struct User {
    pub id_str: String,
}

#[derive(Deserialize)]
pub struct ExtendedEntities {
    pub media: Vec<Media>,
}

#[derive(Deserialize)]
pub struct Media {
    pub media_url_https: String,
    pub display_url: String,
    pub expanded_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

const BASE: &'static str = "https://api.twitter.com/1.1/";

#[derive(Deserialize)]
struct TwitterUser {
    #[serde(rename(deserialize = "id_str"))]
    id: String,
}

pub struct Twitter {}

impl ApiCrawler for Twitter {
    fn get_new_posts(page: &ArtistPage) -> NewPosts {
        match &page.site_last_post_id {
            Some(_since_id) => unimplemented!(),
            None => {
                let request_inital_url = BASE.to_owned()
                    + "statuses/user_timeline.json?count=200&user_id="
                    + &page.site_user_id;
                let _request_max_id_url = request_inital_url.to_owned() + "&max_id=";
                let mut result: Vec<RequiredApiFields> = Vec::new();
                result =
                    Self::extract_posts_from_url(&request_inital_url, &page.site_user_id, result);

                NewPosts {
                    last_site_id: None, //FIXME
                    posts: result,
                }
            }
        }
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

impl Twitter {
    //TODO somehow mutate this without having to return it
    //TODO make part of Trait
    //TODO also return lowest/highest site id
    fn extract_posts_from_url(
        url: &str,
        user_id: &str,
        mut result: Vec<RequiredApiFields>,
    ) -> Vec<RequiredApiFields> {
        let text = Self::make_authenticated_request(url)
            .unwrap()
            .text()
            .unwrap();
        let json: Vec<TwitterPost> = serde_json::from_str(&text).unwrap();
        for post in json {
            if post.retweeted_status.is_none()
                && post.user.id_str.eq(user_id)
                && post.extended_entities.is_some()
            {
                for media in post.extended_entities.unwrap().media {
                    if media.type_field.eq("photo") {
                        result.push(RequiredApiFields {
                            source_url: media.expanded_url,
                            direct_url: media.media_url_https,
                            created_at: chrono::Utc::now(),
                        });
                    }
                }
            };
        }
        result
    }
}
