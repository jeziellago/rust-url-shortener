use redis::Client;
use warp::Filter;

mod pathgenerator;

pub use crate::pathgenerator::{ShortenerGenerator, UrlPathWriter};

#[tokio::main]
async fn main() {
    let path_writer = UrlPathWriter::new(Client::open("redis://127.0.0.1:6379/").unwrap());
    let shortener = ShortenerGenerator::new(path_writer);

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

#[cfg(test)]
mod tests {
    use crate::pathgenerator::{ShortenerGenerator, UrlPathService};

    #[derive(Clone)]
    struct FakeService {}

    impl UrlPathService for FakeService {
        fn save_shortened_url(&self, _: &std::string::String, _: &std::string::String) {
            // fake
        }
        fn get_last_shortened_id(&self) -> usize {
            99999999
        }
    }

    #[test]
    fn test_generate_short_url_path() {
        let fake_service = FakeService{};
        let generator = ShortenerGenerator::new(fake_service);
        let url = String::from("https://google.com");
        let expected = "gVKJn";

        assert_eq!(expected, generator.generate_short_url_path(url));
    }
}
