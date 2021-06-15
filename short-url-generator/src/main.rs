use redis::Client;
use warp::Filter;

mod pathgenerator;

pub use crate::pathgenerator::ShortenerGenerator;

#[tokio::main]
async fn main() {
    let shortener = ShortenerGenerator::new(Client::open("redis://127.0.0.1:6379/").unwrap());

    let url_shortener_route = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("index.html"));

    let shortener_route = warp::get()
        .and(warp::path("new"))
        .and(warp::path::param())
        .map(move |source: String| shortener.generate_short_url_path(source));

    let routes = warp::get().and(url_shortener_route.or(shortener_route));

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
