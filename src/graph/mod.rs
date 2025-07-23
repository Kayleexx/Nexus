use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Write as IoWrite};
use std::fmt::Write as FmtWrite; // 🔧 bring `write!` and `writeln!` for String

#[derive(Debug)]
pub struct DepGraph {
    edges: HashMap<String, Vec<String>>,
}

impl DepGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, filename: String, deps: Vec<String>) {
        self.edges.insert(filename, deps);
    }

    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        let mut all_cycles = Vec::new();

        for node in self.edges.keys() {
            if !visited.contains(node) {
                self.dfs(node, &mut visited, &mut stack, &mut all_cycles);
            }
        }

        all_cycles
    }

    fn dfs(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        stack: &mut Vec<String>,
        all_cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        stack.push(node.to_string());

        if let Some(neighbors) = self.edges.get(node) {
            for neighbor in neighbors {
                if stack.contains(neighbor) {
                    let start = stack.iter().position(|x| x == neighbor).unwrap();
                    let cycle = stack[start..].to_vec();
                    all_cycles.push(cycle);
                } else if !visited.contains(neighbor) {
                    self.dfs(neighbor, visited, stack, all_cycles);
                }
            }
        }

        stack.pop();
    }

    pub fn export_to_json(&self, path: &str) -> io::Result<()> {
        let json_map = &self.edges;
        let json = serde_json::to_string_pretty(json_map)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn export_to_dot(&self, path: &str) -> io::Result<()> {
        let mut dot = String::from("digraph dependencies {\n");

        for (file, deps) in &self.edges {
            for dep in deps {
                writeln!(dot, "    \"{}\" -> \"{}\";", file, dep)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            }
        }

        dot.push_str("}\n");

        let mut file = File::create(path)?;
        file.write_all(dot.as_bytes())?;

        Ok(())
    }
}
