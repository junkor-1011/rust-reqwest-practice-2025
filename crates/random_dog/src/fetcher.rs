use reqwest::Client;
use validator::Validate;

use crate::models::Woof;

pub async fn fetch_random_dog(client: &Client) -> Result<Woof, anyhow::Error> {
    let response = client.get("https://random.dog/woof.json").send().await?;
    let body: Woof = response.json().await?;
    body.validate()?;

    Ok(body)
}
