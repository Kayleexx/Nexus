use crate::parsers::{
    rust::RustParser,
    python::PythonParser,
    javascript::JavaScriptParser,
    java::JavaParser,
    LanguageParser,
};
use std::fs;

/// Returns all available language parsers.
/// Add new ones here to extend multi-language support.
pub fn get_parsers() -> Vec<Box<dyn LanguageParser>> {
    vec![
        Box::new(RustParser),
<<<<<<< HEAD
        Box::new(PythonParser),
        Box::new(JavaScriptParser),
        Box::new(JavaParser),
=======
        // more parsers here: PythonParser, JsParser, etc.
>>>>>>> e015db042a9620ae9188d631141c33cc36be9e0d
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

<<<<<<< HEAD
    if let Some(lang) = get_language_by_extension(path.to_str().unwrap_or_default()) {
        for parser in parsers {
            if parser.language_name() == lang {
                let deps = parser
                    .parse_file(&file_content)
                    .map_err(|e| format!("Parse error in {}: {:?}", path.display(), e))?;
                return Ok((path.display().to_string(), deps));
=======
    let content = fs::read_to_string(path)?; \

    for parser in parsers {
        for &ext in parser.file_extension() {
            if let Some(file_ext) = path.extension().and_then(|e| e.to_str()) {
                if file_ext == ext {
                    let deps = parser.parse_file(&content)?;
                    let file_name = path.to_string_lossy().to_string();
                    return Ok((file_name, deps));
                }
>>>>>>> e015db042a9620ae9188d631141c33cc36be9e0d
            }
        }
    }

    Err(format!("No parser found for {}", path.display()))
}
