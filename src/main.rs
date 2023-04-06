use std::str::FromStr;

use clap::Parser;
use cli::Args;
use yakudo::calc_yakudo_score;

mod cli;
mod yakudo;

#[tokio::main]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("{:#}", err);
        std::process::exit(1);
    }
}

async fn try_main() -> anyhow::Result<()> {
    let args = Args::parse();

    let score = if let Ok(url) = reqwest::Url::from_str(&args.url_or_file) {
        let response = reqwest::get(url).await?;
        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch image: {}", response.status());
        }

        let image_data = response.bytes().await?.to_vec();
        calc_yakudo_score(&image_data)?
    } else {
        let image_data = std::fs::read(&args.url_or_file)?;
        calc_yakudo_score(&image_data)?
    };

    if score >= 150.0 {
        println!("GoodYakudo!\nScore: {}", score);
    } else {
        println!("もっとyakudoしろ！\nScore: {}", score);
    }
    Ok(())
}
