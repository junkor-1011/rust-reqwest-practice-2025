use random_dog::fetcher::fetch_random_dog;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = reqwest::Client::new();
    let content = fetch_random_dog(&client).await?;

    println!(
        "url: {}, file_size_bytes: {}",
        content.url, content.file_size_bytes
    );

    Ok(())
}
