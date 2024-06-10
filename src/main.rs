use reqwest;
use serde::Deserialize;
use tokio;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::time;

#[derive(Debug, Deserialize)]
struct NgrokTunnel {
    public_url: String,
    proto: String,
    config: NgrokConfig,
}

#[derive(Debug, Deserialize)]
struct NgrokConfig {
    addr: String,
}

#[tokio::main]
async fn main() {
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
            .progress_chars("#>-"),
    );

    for _ in 0..100 {
        bar.inc(1);
        time::sleep(Duration::from_millis(255)).await;
    }

    bar.finish();

    match get_ngrok_tunnels().await {
        Ok(tunnels) => {
            if tunnels.is_empty() {
                println!("Ngrok was not found.");
            } else {
                for tunnel in tunnels {
                    println!("Found ngrok tunnel: {:?}", tunnel);
                }
                println!("Ngrok was found.");
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("Press any key to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

async fn get_ngrok_tunnels() -> Result<Vec<NgrokTunnel>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:4040/api/tunnels")
        .send()
        .await?
        .json::<NgrokApiResponse>()
        .await?;
    Ok(response.tunnels)
}

#[derive(Debug, Deserialize)]
struct NgrokApiResponse {
    tunnels: Vec<NgrokTunnel>,
}
