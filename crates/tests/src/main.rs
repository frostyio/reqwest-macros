use request_macro::request;
use reqwest;
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Debug, Serialize, Deserialize, Default)]
#[request(get, "https://httpbin.org/get")]
#[serde(default)]
struct Get {
	origin: String,
	url: String,
}

async fn make_request() -> Result<Get, Box<dyn std::error::Error>> {
	let mut client = reqwest::Client::new();

	Ok(Get::json(Get::from(&mut client).send().await?).await?)
	// Ok(Get::json(Get::new().send().await?).await?)
}

#[tokio::main]
async fn main() {
	println!(
		"{:?}",
		make_request().await.expect("failed to make request")
	);
}
