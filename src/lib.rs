use anyhow::{anyhow, ensure, Context};
pub use parse_id::parse_id;
use regex::Regex;
use serde_json::Value;

mod parse_id;

#[derive(Debug)]
pub struct Caption {
    pub start: f64,
    pub dur: f64,
    pub text: String,
}

async fn fetch_data(url: &str) -> anyhow::Result<String> {
    reqwest::get(url)
        .await
        .context("Failed to fetch data")?
        .text()
        .await
        .context("Failed to get response text")
}

fn parse_caption_tracks(data: &str) -> anyhow::Result<Value> {
    let regex = Regex::new(r#""captionTracks":(\[.*?])"#).context("Failed to create regex")?;
    let captures = regex.captures(data).context("No match found")?;
    let caption_tracks_json = format!("{{\"captionTracks\":{}}}", &captures[1]);
    serde_json::from_str(&caption_tracks_json).context("Failed to parse JSON")
}

fn find_subtitle_track<'a>(caption_tracks: &'a Value, lang: &str) -> anyhow::Result<&'a Value> {
    caption_tracks["captionTracks"]
        .as_array()
        .context("Invalid captionTracks format")?
        .iter()
        .find(|track| {
            track["vssId"]
                .as_str()
                .map_or(false, |id| id.ends_with(lang))
        })
        .ok_or_else(|| anyhow!("Could not find {lang} captions"))
}

fn parse_captions(transcript: &str) -> anyhow::Result<Vec<Caption>> {
    let lines_regex = Regex::new(r#"<text start="([\d.]+)" dur="([\d.]+)".*?>(.*?)</text>"#)
        .context("Failed to create lines regex")?;

    lines_regex
        .captures_iter(transcript)
        .map(|cap| {
            let start = cap[1].parse().context("Failed to parse start time")?;
            let dur = cap[2].parse().context("Failed to parse duration")?;
            let text = html_escape::decode_html_entities(&cap[3])
                .replace('\n', " ")
                .replace(r"&#39;", "'");

            Ok(Caption { start, dur, text })
        })
        .collect()
}

/// # Errors
/// Returns an error if the video does not have captions in the specified language.
pub async fn get_subtitles(video_id: &str, lang: &str) -> anyhow::Result<Vec<Caption>> {
    let url = format!("https://youtube.com/watch?v={video_id}");
    let data = fetch_data(&url).await?;

    ensure!(
        data.contains("captionTracks"),
        "Could not find captions for video: {video_id}"
    );

    let caption_tracks = parse_caption_tracks(&data)?;
    let subtitle = find_subtitle_track(&caption_tracks, lang)?;

    let base_url = subtitle["baseUrl"].as_str().context("No baseUrl found")?;
    let transcript = fetch_data(base_url).await?;

    parse_captions(&transcript)
}
