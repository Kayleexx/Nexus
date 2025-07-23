use std::fs;
use std::path::Path;

use crate::parsers::{LanguageParser, rust::RustParser};
use crate::analyzer::errors::ParseError;

/// Holds all available language parsers
pub fn get_parsers() -> Vec<Box<dyn LanguageParser>> {
    vec![
        Box::new(RustParser),
        // Add more parsers here: PythonParser, JsParser, etc.
    ]
}

/// Select appropriate parser based on file extension
pub fn parse_file_with_matching_parser<P: AsRef<Path>>(
    path: P,
    parsers: &[Box<dyn LanguageParser>],
) -> Result<(String, Vec<String>), ParseError> {
    let path = path.as_ref();

    let content = fs::read_to_string(path)?; // Uses `From<std::io::Error>` for `?`

    for parser in parsers {
        for &ext in parser.file_extension() {
            if let Some(file_ext) = path.extension().and_then(|e| e.to_str()) {
                if file_ext == ext {
                    let deps = parser.parse_file(&content)?;
                    let file_name = path.to_string_lossy().to_string();
                    return Ok((file_name, deps));
                }
            }
        }
    }

    Err(ParseError::InvalidSyntax(format!(
        "No parser found for file: {}",
        path.display()
    )))
}
