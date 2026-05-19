#[tokio::main]
async fn main() {
    url_shortener::run().await;
}
