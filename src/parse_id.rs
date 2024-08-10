use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:https?://)?(?:www\.)?(?:youtube\.com/watch\?v=|youtu\.be/)([a-zA-Z0-9_-]{11})")
        .expect("Failed to compile YouTube URL regex")
});

static ID_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_-]{11}$").expect("Failed to compile YouTube ID regex"));

pub fn parse_id(input: &str) -> anyhow::Result<&str> {
    // First, check if the input is a direct video ID
    if ID_REGEX.is_match(input) {
        return Ok(input);
    }

    // If not, try to extract the ID from a URL
    let captures = URL_REGEX
        .captures(input)
        .context("Failed to match YouTube URL pattern")?;

    captures
        .get(1)
        .map(|m| m.as_str())
        .context("No YouTube ID found in the provided URL")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id() -> anyhow::Result<()> {
        let test_cases = vec![
            ("https://www.youtube.com/watch?v=xpUtDk79dww", "xpUtDk79dww"),
            ("http://www.youtube.com/watch?v=xpUtDk79dww", "xpUtDk79dww"),
            ("https://youtube.com/watch?v=xpUtDk79dww", "xpUtDk79dww"),
            ("http://youtube.com/watch?v=xpUtDk79dww", "xpUtDk79dww"),
            ("https://youtu.be/xpUtDk79dww", "xpUtDk79dww"),
            ("http://youtu.be/xpUtDk79dww", "xpUtDk79dww"),
            (
                "https://www.youtube.com/watch?v=xpUtDk79dww&feature=share",
                "xpUtDk79dww",
            ),
            (
                "https://youtu.be/xpUtDk79dww?si=9FP8TtAI2kYzpdrC",
                "xpUtDk79dww",
            ),
            ("xpUtDk79dww", "xpUtDk79dww"), // Direct video ID
        ];

        for (input, expected) in test_cases {
            let result = parse_id(input)
                .with_context(|| format!("Failed to parse ID from input: {input}"))?;
            assert_eq!(result, expected, "Mismatch for input: {input}");
        }

        Ok(())
    }

    #[test]
    fn test_parse_id_invalid_input() {
        let invalid_inputs = vec![
            "https://www.example.com",
            "https://youtube.com",
            "https://youtu.be",
            "xpUtDk79dw",   // Too short
            "xpUtDk79dwww", // Too long
            "xpUtDk79dw!",  // Invalid character
        ];

        for input in invalid_inputs {
            assert!(
                parse_id(input).is_err(),
                "Expected error for input: {input}"
            );
        }
    }
}
