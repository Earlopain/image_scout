use crate::artist_post::ArtistPost;
use crate::image_info::ImageInfo;
use crate::schema::upload_cache;
use chrono::{DateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use upload_cache::columns;

#[derive(Insertable)]
#[table_name = "upload_cache"]
pub struct NewUploadCache<'a> {
    pub blob: &'a Vec<u8>,
    pub perceptual_hash: &'a Vec<u8>,
    pub file_type: &'a str,
    pub added_at: &'a DateTime<Utc>,
}

#[derive(Queryable)]
pub struct UploadCache {
    pub id: i64,
    pub perceptual_hash: Vec<u8>,
    pub file_type: String,
    pub added_at: DateTime<Utc>,
}

#[derive(Queryable)]
pub struct UploadCacheBlobOnly {
    pub id: i64,
    pub blob: Vec<u8>,
    pub file_type: String,
}

impl UploadCache {
    pub fn create_from_vec(
        image: Vec<u8>,
        conn: &PgConnection,
    ) -> Result<UploadCache, diesel::result::Error> {
        UploadCache::create(ImageInfo::create_from_vec(image), conn)
    }

    pub fn create_from_url(
        url: &str,
        conn: &PgConnection,
    ) -> Result<UploadCache, diesel::result::Error> {
        UploadCache::create(ImageInfo::create_from_url(url), conn)
    }

    fn create(
        image_info: Result<ImageInfo, Box<dyn std::error::Error>>,
        conn: &PgConnection,
    ) -> Result<UploadCache, diesel::result::Error> {
        match image_info {
            Ok(info) => {
                let cache = NewUploadCache {
                    blob: info.get_blob(),
                    perceptual_hash: &info.get_perceptual_hash(),
                    file_type: &info.get_format(),
                    added_at: &Utc::now(),
                };

                let inserted = diesel::insert_into(upload_cache::table)
                    .values(&cache)
                    .returning((
                        columns::id,
                        columns::perceptual_hash,
                        columns::file_type,
                        columns::added_at,
                    ))
                    .get_result(conn)?;
                Ok(inserted)
            }
            Err(e) => {
                panic!("Passed invalid image to insert into db\n".to_string() + &e.to_string())
            }
        }
    }

    pub fn delete(id: &i64, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(upload_cache::table.filter(columns::id.eq(id))).execute(conn)
    }

    pub fn get_info(
        search_for: &i64,
        conn: &PgConnection,
    ) -> Result<UploadCacheBlobOnly, diesel::result::Error> {
        upload_cache::table
            .select((columns::id, columns::blob, columns::file_type))
            .filter(columns::id.eq(search_for))
            .first(conn)
    }

    pub fn get_similar_artist_posts(
        self: &Self,
        conn: &PgConnection,
    ) -> Result<Vec<i64>, diesel::result::Error> {
        let all_posts = ArtistPost::get_all_for_compare(conn)?;

        Ok(all_posts
            .into_iter()
            .filter(|post| hamming_distance(&self.perceptual_hash, &post.perceptual_hash) < 10)
            .map(|post| post.id)
            .collect())
    }
}

fn hamming_distance(a: &Vec<u8>, b: &Vec<u8>) -> u32 {
    a.iter()
        .zip(b.iter())
        .fold(0, |acc, (&x1, &x2)| acc + (x1 ^ x2).count_ones())
}
