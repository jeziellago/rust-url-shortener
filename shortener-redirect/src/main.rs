use warp::Filter;
use redis::Client;

mod cache;

pub use crate::cache::{ShortenerCache, Cache};

#[tokio::main]
async fn main() {

    let cache = ShortenerCache::new(Client::open("redis://127.0.0.1:6379/").unwrap());

    let start = warp::path::param()
        .map(move |hash: String| warp::redirect(cache.get_uri_from_hash(hash).unwrap()));
    warp::serve(start).run(([127, 0, 0, 1], 3030)).await;
}
