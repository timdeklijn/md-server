use comrak::{markdown_to_html, ComrakOptions};
use serde::Deserialize;
use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide::{Request, Response, StatusCode};

/// Create a 404 markdown file from an error message
fn create_404_md(e: &str) -> String {
    format!("# Error\n**{}**", e)
}

/// Add a header tag to the html string
///
/// Import fonts, bootstrap and local css
fn add_html_header(html: String) -> String {
    let style = "http://127.0.0.1:8080/static/style.css";
    format!(
        r#"<head>
<link href="https://fonts.googleapis.com/css2?family=Roboto&display=swap" rel="stylesheet">
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@4.5.3/dist/css/bootstrap.min.css">
<link rel='stylesheet' href='{}'>
</head>
{}"#,
        style, html
    )
}

/// Surround a string with a body and div tag
fn add_body_to_html(html: String) -> String {
    format!(
        r#"<body>
<div class="container-sm">
{}
<footer>Tim de Klijn, 2020</footer>
</div>
</body>"#,
        html
    )
}

/// Try to load a file from disk based on folder and file_id in request.
///
/// If the file is not found, return an error page. Else, load the markdown
/// file and convert it to html.
fn load_file_or_404(folder: String, file_id: String) -> String {
    let path = format!("{}/{}.md", folder, file_id);
    let path = Path::new(&path);
    let content = match path.is_file() {
        true => match read_to_string(path) {
            Ok(s) => s,
            Err(_) => create_404_md("Error reading file"),
        },
        false => create_404_md("Not a file"),
    };
    let html = markdown_to_html(&content, &ComrakOptions::default());
    add_html_header(add_body_to_html(html))
}

/// Recursively list all files in p and its nested folders.
fn find_all_files(p: &Path) -> io::Result<Vec<PathBuf>> {
    let mut ff = Vec::new();
    for entry in p.read_dir()? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            // Recurse to find markdown files in subfolders
            ff.extend(find_all_files(&entry_path)?.iter().cloned());
        } else {
            // Check only for markdown files
            let ex = entry_path
                .extension()
                .expect("Error getting file extension");
            if ex == "md" {
                ff.push(entry_path);
            }
        }
    }
    Ok(ff)
}

/// From a pathfub, create a list element with a link.
fn create_link(s: PathBuf) -> String {
    let f = s.file_stem().unwrap();
    let p = s.parent().unwrap();
    let addr = format!(
        "http://127.0.0.1:8080/posts?folder={}&id={}",
        p.to_string_lossy(),
        f.to_string_lossy()
    );
    format!("<li><a href='{}'>{}</a></li>", addr, s.to_string_lossy())
}

/// "/" endpoint, create overview of all files.
async fn home(_req: Request<()>) -> tide::Result {
    let p = Path::new(&"files");
    let files = find_all_files(&p)?;
    let mut links = Vec::new();
    for f in files {
        links.push(create_link(f))
    }
    let links = links.join("\n");
    let s = format!("<h1>Index</h1>\n<ul>\n{}\n</ul>", links);
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/html");
    let body = add_html_header(add_body_to_html(s.to_string()));
    res.set_body(body);
    Ok(res)
}

#[derive(Deserialize, Debug)]
struct File {
    folder: String,
    id: String,
}

/// Process the request, load the file and return a html page.
async fn render_file(req: Request<()>) -> tide::Result {
    let file: File = req.query().expect("Error parsing query string");
    let html = load_file_or_404(file.folder, file.id);
    let mut res = Response::new(StatusCode::Ok);
    res.insert_header("Content-Type", "text/html");
    res.set_body(html);
    Ok(res)
}

/// Create a server
pub fn create_server() -> tide::Server<()> {
    let cors = CorsMiddleware::new()
        .allow_methods("GET".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    let mut app = tide::new();
    app.with(cors);
    app.at("/").get(home);
    app.at("/posts").get(render_file);
    app.at("/static")
        .serve_dir("static/")
        .expect("Static folder not found");
    app
}
