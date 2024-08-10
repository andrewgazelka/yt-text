use anyhow::Context;
use itertools::Itertools;
use yt_text::{get_subtitles, parse_id};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let video_id = std::env::args().nth(1).context("No video ID provided")?;
    let video_id = parse_id(&video_id).context("Invalid video ID")?;

    let locale = sys_locale::get_locale().context("Failed to get locale")?;

    // get first part of locale {x}-{y} get x
    let locale = locale.split('-').next().context("Failed to get locale")?;

    let captions = get_subtitles(video_id, &locale).await?;

    let captions = captions.iter().map(|caption| &caption.text).join(" ");

    println!("{captions}");

    Ok(())
}
