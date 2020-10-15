use comrak::{markdown_to_html, ComrakOptions};
use std::fs::read_to_string;
use std::path::Path;
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide::{Request, Response, StatusCode};

fn file_to_html() -> String {
    let path = Path::new("files/tst.md");
    let content = read_to_string(path).expect("Error while reading file");
    markdown_to_html(&content, &ComrakOptions::default())
}

async fn hello_word(_req: Request<()>) -> tide::Result {
    let text = file_to_html();
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/html");
    res.set_body(text);
    Ok(res)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let cors = CorsMiddleware::new()
        .allow_methods("GET".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    tide::log::start();
    let mut app = tide::new();
    app.at("/").get(hello_word);
    app.with(cors);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
