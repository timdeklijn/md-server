use comrak::{markdown_to_html, ComrakOptions};
use std::fs::read_to_string;
use std::path::Path;
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide::{Request, Response, StatusCode};

async fn render_file(req: Request<()>) -> tide::Result {
    let folder: String = req.param("folder")?;
    let file_id: String = req.param("id")?;
    let path = format!("{}/{}.md", folder, file_id);
    let path = Path::new(&path);
    tide::log::info!("Folder: {}, File id: {}", folder, file_id);
    tide::log::info!("Path: {:?}", path);
    let content = read_to_string(path).expect("Error while reading file");
    let html = markdown_to_html(&content, &ComrakOptions::default());
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/html");
    res.set_body(html);
    Ok(res)
}

pub fn create_server() -> tide::Server<()> {
    let cors = CorsMiddleware::new()
        .allow_methods("GET".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    let mut app = tide::new();
    app.with(cors);
    app.at("/:folder/:id").get(render_file);
    app
}
