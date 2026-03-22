# sqlite-hf

Rust MLOps demo that classifies song lyrics by music genre using Hugging Face zero-shot classification, with SQLite as the candidate label store.

## Project structure

```
sqlite-hf/
├── Cargo.toml        # Package manifest and dependencies
├── lyrics.txt        # Sample input: "En el Muelle de San Blas" by Mana
└── src/
    ├── lib.rs        # Core library: DB, file I/O, and classification logic
    └── main.rs       # CLI entry point
```

## Architecture

```
                  ┌──────────────┐
lyrics.txt ──>   │ read_lyrics() │
                  └──────┬───────┘
                         │ Vec<String>
                         v
                  ┌─────────────────┐      ┌────────────────────────────┐
                  │classify_lyrics() │ <──  │ get_all_zeroshotcandidates()│
                  └────────┬────────┘      └─────────────┬──────────────┘
                           │                             │
                           │                    ┌────────┴────────┐
                           │                    │   create_db()    │
                           │                    │  SQLite :memory: │
                           │                    │  ┌────────────┐  │
                           │                    │  │ rock       │  │
                           │                    │  │ pop        │  │
                           │                    │  │ hip hop    │  │
                           │                    │  │ country    │  │
                           │                    │  │ latin      │  │
                           │                    │  └────────────┘  │
                           │                    └─────────────────┘
                           v
                  Vec<Vec<Label>>
                  (genre + confidence score per input)
```

## Library API (`lib.rs`)

| Function                       | Description                                                      |
|--------------------------------|------------------------------------------------------------------|
| `create_db()`                  | Creates an in-memory SQLite DB and seeds it with 5 genre labels  |
| `get_all_zeroshotcandidates()` | Queries all genre labels from the DB, returns `Vec<String>`      |
| `read_lyrics(file: &str)`      | Reads a text file line by line, returns `Vec<String>`            |
| `classify_lyrics(lyrics)`      | Classifies lyrics against DB genres using zero-shot model        |

## Dependencies

| Crate        | Version | Purpose                                                |
|--------------|---------|--------------------------------------------------------|
| `rust-bert`  | 0.22    | Hugging Face Transformer pipelines (zero-shot, NER, etc) |
| `sqlite`     | 0.36    | SQLite bindings for in-memory candidate label storage  |
| `clap`       | 4.5     | CLI argument parsing with derive macros                |

## Prerequisites

- Rust 1.85+ (edition 2024)
- libtorch (C++ PyTorch backend, required by rust-bert)

## Why PyTorch is installed in the devcontainer

Despite being a Rust project, rust-bert has a hard dependency chain:

```
rust-bert → tch-rs → torch-sys → libtorch (C++ library)
```

libtorch is the C++ math engine that powers PyTorch. It is not Python — it is the
compiled C++ core that runs Transformer models. When `torch-sys` builds, it looks
for libtorch in this order:

1. `LIBTORCH` env var pointing to a local libtorch installation
2. `LIBTORCH_USE_PYTORCH=1` — reuses libtorch bundled inside a pip-installed PyTorch
3. If neither is set, it downloads ~2GB of libtorch during `cargo build`

Option 3 crashes Codespaces (runs out of disk/RAM). The devcontainer uses option 2:
it pre-installs CPU-only PyTorch via pip so that `torch-sys` links against its bundled
libtorch instead of downloading it at build time.

## Setup

```bash
# If running locally (not in Codespaces), install the libtorch dependency:
export LIBTORCH_USE_PYTORCH=1
pip install torch --index-url https://download.pytorch.org/whl/cpu

# If running in Codespaces, libtorch is already pre-installed by the devcontainer.

# Build
cargo build --release

# Run
cargo run --release -- <args>
```

## Usage

```bash
# Classify the sample lyrics file (defaults to lyrics.txt)
cargo run --release -- classify

# Classify a specific lyrics file
cargo run --release -- classify --file path/to/lyrics.txt

# List all genre candidates stored in the database
cargo run --release -- candidates
```
