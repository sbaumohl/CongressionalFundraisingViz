extern crate dotenv;
pub mod entities;
use warp::{http::Response, Filter};

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let warp_test = warp::any().map(|| {
        Response::builder()
            .header("my-custom-header", "some-value")
            .body("and a custom body")
    });

    warp::serve(warp_test).run(([127, 0, 0, 1], 8000)).await
}

// https://www.sea-ql.org/sea-orm-tutorial/ch01-01-project-setup.html
