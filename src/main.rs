pub mod loader;
pub mod server;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let content = loader::create_content_list(&"files".to_string());
    let app = server::create_server(&content);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
