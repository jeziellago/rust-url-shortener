use warp::{http::Uri, Filter};

#[tokio::main]
async fn main() {
    let start = warp::path::param().map(|name: String| {
        warp::redirect(
            Uri::builder()
                .scheme("https")
                .authority("medium.com")
                .path_and_query(format!("/{}", name))
                .build()
                .unwrap(),
        )
    });

    warp::serve(start).run(([127, 0, 0, 1], 3030)).await;
}
