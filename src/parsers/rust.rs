use regex::Regex;
use crate::parsers::LanguageParser;
use crate::analyzer::errors::ParseError;

pub struct RustParser;

impl LanguageParser for RustParser {
    fn parse_file(&self, content: &str) -> Result<Vec<String>, ParseError> {
        let mut deps = Vec::new();
        let re = Regex::new(r"^\s*use\s+([a-zA-Z0-9_:]+)").unwrap();

        for line in content.lines() {
            if let Some(cap) = re.captures(line) {
                if let Some(dep) = cap.get(1) {
                    deps.push(dep.as_str().to_string());
                }
            }
        }
        Ok(deps)
    }

    fn file_extension(&self) -> &[&str] {
        &["rs"]
    }
    fn language_name(&self) -> &str {
        "Rust"
    }

}