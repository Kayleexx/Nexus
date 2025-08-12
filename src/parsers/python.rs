use regex::Regex;
use crate::parsers::LanguageParser;
use crate::analyzer::errors::ParseError;

pub struct PythonParser;

impl LanguageParser for PythonParser {
    fn parse_file(&self, content: &str) -> Result<Vec<String>, ParseError> {
        let mut deps = Vec::new();

        let import_re = Regex::new(r#"^\s*import\s+([a-zA-Z0-9_\.]+)"#)
            .map_err(|e| ParseError::InvalidSyntax(e.to_string()))?;
        let from_import_re = Regex::new(r#"^\s*from\s+([a-zA-Z0-9_\.]+)\s+import"#)
            .map_err(|e| ParseError::InvalidSyntax(e.to_string()))?;

        for line in content.lines() {
            if let Some(cap) = import_re.captures(line) {
                deps.push(cap[1].to_string());
            }
            if let Some(cap) = from_import_re.captures(line) {
                deps.push(cap[1].to_string());
            }
        }

        Ok(deps)
    }

    fn file_extension(&self) -> &[&str] {
        &["py"]
    }

    fn language_name(&self) -> &str {
        "Python"
    }
}
