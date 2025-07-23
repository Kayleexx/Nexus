use clap::Parser;

mod utils;
mod parsers;
pub mod analyzer;
mod graph;

use graph::DepGraph;
use parsers::parser_registry::{get_parsers, parse_file_with_matching_parser};
use utils::file_walker::walk_source_dir;

#[derive(Parser, Debug)]
#[command(name = "nexus")]
struct Args {
    /// Path to the source code directory
    #[arg(short, long)]
    path: String,

    /// Output file path
    #[arg(short, long, default_value = "output.dot")]
    output: String,

    /// Output format: dot or json
    #[arg(long, default_value = "dot")]
    format: String,

    /// Whether to detect circular dependencies
#[arg(long, default_value = "true", action = clap::ArgAction::Set)]
detect_cycles: bool,




}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let parsers = get_parsers();
    let mut graph = DepGraph::new();

    println!("Scanning directory: {}", args.path);

    match walk_source_dir(&args.path) {
        Ok(files) => {
            println!("Found {} files", files.len());

            for file in &files {
                match parse_file_with_matching_parser(file, &parsers) {
                    Ok((filename, deps)) => {
                        let file_str = file.to_string_lossy().to_string();
                        graph.add_file(file_str.clone(), deps.clone());

                        println!("File: {filename}");
                        for dep in &deps {
                            println!("   depends on: {dep}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse {}: {:?}", file.display(), e);
                    }
                }
            }

            match args.format.as_str() {
                "dot" => {
                    if let Err(e) = graph.export_to_dot(&args.output) {
                        eprintln!("Failed to write DOT file: {}", e);
                    } else {
                        println!("DOT graph written to {}", args.output);
                    }
                }
                "json" => {
                    if let Err(e) = graph.export_to_json(&args.output) {
                        eprintln!("Failed to write JSON file: {}", e);
                    } else {
                        println!("JSON graph written to {}", args.output);
                    }
                }
                _ => {
                    eprintln!("Unknown format: {}", args.format);
                }
            }

            if args.detect_cycles {
                let cycles = graph.detect_cycles();
                if !cycles.is_empty() {
                    println!("\nCircular Dependencies Found:");
                    for cycle in cycles {
                        println!(" - {}", cycle.join(" -> "));
                    }
                } else {
                    println!("\nNo circular dependencies found.");
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}
