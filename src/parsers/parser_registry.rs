use crate::parsers::{
    rust::RustParser,
    python::PythonParser,
    javascript::JavaScriptParser,
    java::JavaParser,
    LanguageParser,
};
use std::fs;

/// Returns all available language parsers.

pub fn get_parsers() -> Vec<Box<dyn LanguageParser>> {
    vec![
        Box::new(RustParser),
        Box::new(PythonParser),
        Box::new(JavaScriptParser),
        Box::new(JavaParser),
    ]
}

/// Match file extensions to language names.
fn get_language_by_extension(path: &str) -> Option<&'static str> {
    match std::path::Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("rs") => Some("Rust"),
        Some("py") => Some("Python"),
        Some("js") | Some("ts") => Some("JavaScript/TypeScript"),
        Some("java") => Some("Java"),
        _ => None,
    }
}

/// Try to find a matching parser for a given file.
/// If a parser matches, it parses the file and returns dependencies.
pub fn parse_file_with_matching_parser(
    path: &std::path::Path,
    parsers: &[Box<dyn LanguageParser>],
) -> Result<(String, Vec<String>), String> {
    let file_content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

    if let Some(lang) = get_language_by_extension(path.to_str().unwrap_or_default()) {
        for parser in parsers {
            if parser.language_name() == lang {
                let deps = parser
                    .parse_file(&file_content)
                    .map_err(|e| format!("Parse error in {}: {:?}", path.display(), e))?;
                return Ok((path.display().to_string(), deps));
            }
        }
    }

    Err(format!("No parser found for {}", path.display()))
}

