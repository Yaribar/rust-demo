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

## Setup

```bash
# Option A: Let rust-bert download libtorch automatically via PyTorch
export LIBTORCH_USE_PYTORCH=1
pip install torch --index-url https://download.pytorch.org/whl/cpu

# Option B: Download libtorch manually
# See https://pytorch.org for your platform

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
