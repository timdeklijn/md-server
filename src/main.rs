pub mod loader;
pub mod server;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let app = server::create_server();
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
