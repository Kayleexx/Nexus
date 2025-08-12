# Nexus

**Nexus** is a Rust-based CLI tool for analyzing and visualizing code dependencies in source files. It builds a dependency graph from Rust code and supports circular dependency detection.

## Features

- Analyze Rust project structure by parsing `mod` and `use` declarations
- Generate dependency graph in DOT format
- Detect circular dependencies
- Traverse entire source directory
- Modular parser system (currently supports Rust)

## Usage

### Build

```bash
cargo build --release
````

### Run

```bash
cargo run -- --path ./src
```

#### Options

| Flag              | Description                                 | Default      |
| ----------------- | ------------------------------------------- | ------------ |
| `--path, -p`      | Path to the source code directory           | *(required)* |
| `--output, -o`    | Output file path                            | `output.dot` |
| `--format`        | Output format: `dot` or `json`              | `dot`        |
| `--detect-cycles` | Enable or disable circular dependency check | `true`       |

Example:

```bash
cargo run -- --path ./src --output graph.dot --format dot --detect-cycles false
```

## Output

The tool outputs a `.dot` file representing the dependency graph. You can render it using Graphviz:

```bash
dot -Tpng output.dot -o graph.png
```

<img width="500" height="933" alt="image" src="https://github.com/user-attachments/assets/4b7a0ecb-8356-4586-b312-68e140b6b01d" />

<img width="500" height="930" alt="image" src="https://github.com/user-attachments/assets/e936faf2-4a80-4b95-802d-a2e9b2d1c945" />

## Project Structure

```
nexus/
├── src/
│   ├── main.rs          # CLI entrypoint
│   ├── graph/           # Graph construction and cycle detection
│   ├── parsers/         # Rust parser module
│   ├── utils/           # File system walking logic
│   └── analyzer/        # Central analyzer logic
```


