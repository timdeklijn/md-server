use comrak::{markdown_to_html, ComrakOptions};
use std::fs::read_to_string;
use std::path::Path;
use std::sync::Arc;
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide::{Middleware, Next, Request, Response, StatusCode};

use crate::loader::Content;

struct DB {
    l: Arc<Vec<Content>>,
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for DB {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> tide::Result {
        tide::log::trace!("Hit middleware");
        Ok(self.l)
    }
}

fn file_to_html() -> String {
    let path = Path::new("files/tst.md");
    let content = read_to_string(path).expect("Error while reading file");
    markdown_to_html(&content, &ComrakOptions::default())
}

async fn render_file(req: Request<()>) -> tide::Result {
    let mut file_id: String = req.param("id")?;
    file_id = file_id.parse().unwrap();
    let text = file_to_html();
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/html");
    res.set_body("<h1>HALLO</h1>".to_string());
    Ok(res)
}

pub fn create_server(content: &Vec<Content>) -> tide::Server<()> {
    let db = DB { l: *content };
    let cors = CorsMiddleware::new()
        .allow_methods("GET".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    let mut app = tide::new();
    app.with(db);
    app.with(cors);
    app.at("/post/:id").get(render_file);
    app
}
