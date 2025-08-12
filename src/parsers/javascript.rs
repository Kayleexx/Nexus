use regex::Regex;
use crate::parsers::LanguageParser;
use crate::analyzer::errors::ParseError;

pub struct JavaScriptParser;

impl LanguageParser for JavaScriptParser {
    fn parse_file(&self, content: &str) -> Result<Vec<String>, ParseError> {
        let mut deps = Vec::new();

        let import_re = Regex::new(r#"^\s*import\s+.*from\s+['"]([^'"]+)['"]"#)
            .map_err(|e| ParseError::InvalidSyntax(e.to_string()))?;
        let require_re = Regex::new(r#"require\(['"]([^'"]+)['"]\)"#)
            .map_err(|e| ParseError::InvalidSyntax(e.to_string()))?;

        for line in content.lines() {
            if let Some(cap) = import_re.captures(line) {
                deps.push(cap[1].to_string());
            }
            if let Some(cap) = require_re.captures(line) {
                deps.push(cap[1].to_string());
            }
        }

        Ok(deps)
    }

    fn file_extension(&self) -> &[&str] {
        &["js", "ts"]
    }

    fn language_name(&self) -> &str {
        "JavaScript/TypeScript"
    }
}
