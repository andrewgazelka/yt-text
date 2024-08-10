# YouTube Subtitle Extractor

A Rust program to extract subtitles from YouTube videos.

## Features

- Extract subtitles using a YouTube video ID or URL
- Automatically detects language based on your system locale
- Supports various YouTube URL formats
- Outputs subtitles as plain text

## How to Run

Run the program with a YouTube video ID or URL:

   ```
   cargo run --release -- [VIDEO_ID_OR_URL]
   ```

Examples:

   ```
   cargo run --release -- 'xpUtDk79dww'
   cargo run --release -- 'https://www.youtube.com/watch?v=xpUtDk79dww'
   cargo run --release -- 'https://youtu.be/xpUtDk79dww'
   ```

The program will output the extracted subtitles as plain text.
