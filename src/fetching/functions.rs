// std imports
use std::{error::Error, io::BufReader};

// third-party imports
use regex::Regex;
use rss::Channel;

/// Fetch the contents from the given URL.
fn get(uri: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(uri)?.text()?;
    Ok(body)
}

/// Parse the RSS feed from the given url.
fn parse_rss(contents: &str) -> Result<Channel, Box<dyn Error>> {
    let channel = Channel::read_from(BufReader::new(contents.as_bytes()))?;
    Ok(channel)
}

/// Transforms feed tags into rss tags and wraps them around a channel tag in an attempt to improve the success rate of the rss parser
fn sanitize(contents: &str) -> String {
    let start_re = Regex::new("<feed (.*?)>").unwrap();
    let end_re = Regex::new("</feed>").unwrap();
    if start_re.is_match(contents) && end_re.is_match(contents) {
        let start = start_re.replace_all(contents, "<rss $1><channel>");
        let end = end_re.replace_all(&start, "</channel></rss>");
        end.to_string()
    } else {
        contents.to_string()
    }
}

/// Fetch the contents from the given URLs and parse it as an RSS feed. Returning a vector of channels.
pub fn get_channels(urls: &[&str]) -> Vec<Result<Channel, Box<dyn Error>>> {
    let mut channels = Vec::new();
    for url in urls {
        let contents = get(url);
        match contents {
            Ok(contents) => {
                let contents = sanitize(&contents);
                let channel = parse_rss(&contents);
                channels.push(channel);
            }
            Err(e) => channels.push(Err(e)),
        }
    }
    channels
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    /// Test wether the function get() returns an Ok(String)
    fn test_get() {
        let result = get("https://www.rust-lang.org/en-US/");
        // Check that the function succeeded
        assert!(result.is_ok());
    }

    #[test]
    /// Test wether the function parse_rss() returns an Ok(Channel) and has an appropriate length
    fn test_parse() {
        let f = fs::read_to_string("./resources/testing/example.rss").unwrap();
        let result = parse_rss(&f);
        // Check that the function succeeded
        assert!(result.is_ok());
        // Check that we got three items
        assert_eq!(result.unwrap().items().len(), 3);
    }

    #[test]
    /// Test wether the function get_channels returns an Ok(Vec<Channel>)
    fn test_get_channels() {
        let urls = [
            "https://blog.rust-lang.org/feed.xml",
            "https://github.com/timeline",
        ];
        let results = get_channels(&urls);
        println!("{:?}", results);
        // Check that the function succeeded
        for result in &results {
            assert!(result.is_ok());
        }
        // Check that we got two channels
        assert_eq!(results.len(), 2);
    }
}
