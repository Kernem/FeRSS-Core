use std::{error::Error, io::BufReader};

use rss::Channel;

/// Fetch the contents from the given URL.
pub fn get(uri: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(uri)?.text()?;
    Ok(body)
}

/// Parse the RSS feed from the given url.
pub fn parse_rss(contents: &str) -> Result<Channel, Box<dyn Error>> {
    let channel = Channel::read_from(BufReader::new(contents.as_bytes()))?;
    Ok(channel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    /// Test wether the function get() returns an Ok(String)
    fn test_get() {
        let result = get("https://www.rust-lang.org/en-US/");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse() {
        let f = fs::read_to_string("./resources/testing/example.rss").unwrap();
        let result = parse_rss(&f);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().items().len(), 3);
    }
}