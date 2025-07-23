pub mod rust;
pub mod parser_registry;
use crate::analyzer::errors::ParseError;

pub trait LanguageParser {
    fn parse_file(&self, content: &str) -> Result<Vec<String>, ParseError>;
    fn file_extension(&self) -> &[&str];
    fn language_name(&self) -> &str;
}