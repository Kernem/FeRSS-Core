use std::error::Error;

pub fn get(uri: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::blocking::get(uri)?.text()?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    /// Test wether the function get() returns an Ok(String)
    fn test_get() {
        let result = get("https://www.rust-lang.org/en-US/");
        assert!(result.is_ok());
    }
}