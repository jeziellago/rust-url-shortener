extern crate redis;

use redis::Client;
use warp::http::Uri;
use percent_encoding::percent_decode;

pub trait Cache {
    fn new(client: Client) -> ShortenerCache;
    fn get_uri_from_hash(&self, hash: String) -> Option<Uri>;
}

#[derive(Clone)]
pub struct ShortenerCache {
    client: Client,
}

impl Cache for ShortenerCache {
    fn new(client: Client) -> ShortenerCache {
        return ShortenerCache { client: client };
    }

    fn get_uri_from_hash(&self, hash: String) -> Option<Uri> {
        println!("Tracking for {}", hash);
        let mut con = self.client.get_connection().unwrap();
        let hash_str = hash.as_str();
        let mut shortened_url: String = redis::cmd("GET")
            .arg(hash_str)
            .query(&mut con)
            .expect(format!("failed to execute GET for {}", hash_str).as_str());
        shortened_url = percent_decode(shortened_url.as_bytes()).decode_utf8().unwrap().to_string();
        Some(shortened_url.parse::<Uri>().unwrap())
    }
}
