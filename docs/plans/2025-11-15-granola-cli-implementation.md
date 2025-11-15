# Granola CLI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust CLI tool that reads Granola's local cache and outputs JSON optimized for LLM consumption.

**Architecture:** Single binary CLI using clap for argument parsing, serde for JSON handling, and custom double-parse logic for Granola's nested cache format. Commands output JSON to stdout, errors to stderr, with Unix exit codes.

**Tech Stack:** Rust 2021, clap 4 (derive), serde/serde_json, anyhow, chrono

---

## Task 1: Project Setup & Dependencies

**Files:**
- Modify: `Cargo.toml`
- Create: `src/main.rs`
- Create: `CLAUDE.md`
- Create: `README.md`

**Step 1: Initialize Cargo project**

Run: `cargo init --name granola-cli`
Expected: Creates basic Cargo.toml and src/main.rs

**Step 2: Add dependencies to Cargo.toml**

Replace the `[dependencies]` section with:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
chrono = { version = "0.4", features = ["serde"] }
```

**Step 3: Verify dependencies resolve**

Run: `cargo check`
Expected: Downloads dependencies, compiles successfully

**Step 4: Create CLAUDE.md**

Create `CLAUDE.md` with the following content:

```markdown
# Granola CLI - AI Assistant Context

## Project Overview

A Rust CLI tool that reads Granola's local cache and outputs JSON data optimized for LLM consumption. Replaces the need for an MCP server by providing direct command-line access to meeting data.

## Key Design Principles

1. **LLM-First**: All output is JSON, schemas optimized for token efficiency
2. **Context-Aware Naming**: Clear names at top-level, compact in repeated arrays
3. **Self-Documenting**: `granola workflow` provides comprehensive usage guide
4. **Unix Philosophy**: stdout=data, stderr=errors, exit codes=status

## Architecture Decisions

### Why Rust?
- Fast JSON parsing (sub-2s for 100+ meetings)
- Single binary distribution
- Strong typing prevents runtime errors

### Why Not MCP?
- Simpler deployment (just a binary)
- No protocol overhead
- Direct Bash invocation from Claude Code

### Cache Parsing Strategy
The cache uses double-nested JSON (`{"cache": "{...}"}`) requiring two parse operations. This is intentional replication of the Python MCP server's proven approach.

### Token Optimization
Transcript segments use short keys (`s`, `t`, `ts`) because they repeat 100+ times. This saves ~200 tokens per transcript. Top-level fields use clear names because they appear once.

## Code Organization

- `main.rs` - CLI setup, arg parsing with clap
- `cache.rs` - Double-parse logic, loading from disk
- `models.rs` - Serde structs for cache format and output
- `commands/` - One file per command (search, details, transcript, documents, workflow)
- `error.rs` - Custom error types, exit code mapping
- `output.rs` - JSON serialization, stderr vs stdout logic

## Testing Strategy

- Unit tests for cache parsing (mock JSON)
- Integration tests with sample cache files
- Schema validation tests (ensure output matches documented schemas)
- Error handling tests (verify exit codes and messages)

## Common Tasks

### Adding a new command
1. Add variant to `Commands` enum in `main.rs`
2. Create `commands/newcommand.rs`
3. Add output struct to `models.rs`
4. Update `granola workflow` markdown
5. Add tests

### Modifying output schema
1. Update struct in `models.rs`
2. Update workflow guide in `commands/workflow.rs`
3. Update this CLAUDE.md if it's a significant change
4. Add schema validation test

### Performance optimization
- Profile with real cache files (use `--release` builds)
- Target: <2s load time for 1000 meetings
- Watch for unnecessary clones (cache is read-only)

## Design Constraints

### Must Not
- Modify the cache file (read-only tool)
- Add interactive prompts (breaks LLM usage)
- Output to stdout except JSON or errors (with --json-errors)
- Use complex scoring algorithms (LLM does ranking)

### Should
- Keep dependencies minimal (faster builds)
- Validate cache structure gracefully
- Provide helpful error messages
- Follow Unix conventions

## Future Considerations

Not in v1, but documented for later:

- Watch mode: `granola watch` for cache updates
- Export formats: `--format csv` or `--format markdown`
- Advanced filters: `--from DATE --to DATE --participants "Dave"`
- Analytics: Implement the `analyze` command from MCP server
- Streaming: Large transcripts could stream to stdout

## Questions for Humans

If you're uncertain about:
- Output schema changes → Ask (affects LLM consumers)
- New dependencies → Check if really needed
- Performance trade-offs → Profile first, then discuss
- Breaking changes → Definitely ask

## Related Documentation

- `/Users/lucio/Library/Application Support/Granola/CONTEXT.md` - Cache format analysis
- Python MCP server: `github.com/proofgeist/granola-ai-mcp-server` - Reference implementation
- Workflow guide: `granola workflow` output - User-facing docs
```

**Step 5: Create basic README.md**

Create `README.md` with:

```markdown
# Granola CLI

A command-line tool for querying Granola meeting data. Optimized for AI assistant consumption.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Search meetings
granola search "moose" --limit 10

# Get meeting details
granola details <meeting-id>

# Get transcript
granola transcript <meeting-id>

# Get documents/notes
granola documents <meeting-id>

# Show usage guide (for AI assistants)
granola workflow
```

## Configuration

Cache path priority:
1. `--cache-path` flag
2. `GRANOLA_CACHE_PATH` environment variable
3. Default: `~/Library/Application Support/Granola/cache-v3.json`

## For AI Assistants

Run `granola workflow` to get comprehensive usage patterns, output schemas, and best practices.
```

**Step 6: Commit initial setup**

Run:
```bash
git add Cargo.toml CLAUDE.md README.md
git commit -m "chore: initial project setup with dependencies"
```

Expected: Clean commit with project structure

---

## Task 2: Error Types & Exit Codes

**Files:**
- Create: `src/error.rs`
- Modify: `src/main.rs` (add mod declaration)

**Step 1: Create error module with custom types**

Create `src/error.rs`:

```rust
use std::fmt;

#[derive(Debug)]
pub enum GranolaError {
    CacheNotFound(String),
    InvalidCacheFormat(String),
    MeetingNotFound(String),
    TranscriptNotFound(String),
    InvalidArguments(String),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for GranolaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GranolaError::CacheNotFound(path) => {
                write!(f, "Error: Cache file not found\nPath: {}\nSuggestion: Ensure Granola is installed and has been run at least once", path)
            }
            GranolaError::InvalidCacheFormat(msg) => {
                write!(f, "Error: Invalid cache format\nDetails: {}\nSuggestion: Cache might be corrupted or version mismatch", msg)
            }
            GranolaError::MeetingNotFound(id) => {
                write!(f, "Error: Meeting not found\nID: {}\nSuggestion: Use 'granola search' to find valid meeting IDs", id)
            }
            GranolaError::TranscriptNotFound(id) => {
                write!(f, "Error: Transcript not found for meeting\nID: {}\nSuggestion: This meeting may not have been transcribed", id)
            }
            GranolaError::InvalidArguments(msg) => {
                write!(f, "Error: Invalid arguments\nDetails: {}", msg)
            }
            GranolaError::IoError(e) => write!(f, "Error: IO error\nDetails: {}", e),
            GranolaError::JsonError(e) => write!(f, "Error: JSON parsing error\nDetails: {}", e),
        }
    }
}

impl std::error::Error for GranolaError {}

impl From<std::io::Error> for GranolaError {
    fn from(err: std::io::Error) -> Self {
        GranolaError::IoError(err)
    }
}

impl From<serde_json::Error> for GranolaError {
    fn from(err: serde_json::Error) -> Self {
        GranolaError::JsonError(err)
    }
}

impl GranolaError {
    pub fn exit_code(&self) -> i32 {
        match self {
            GranolaError::CacheNotFound(_) => 2,
            GranolaError::InvalidCacheFormat(_) => 3,
            GranolaError::MeetingNotFound(_) => 4,
            GranolaError::TranscriptNotFound(_) => 4,
            GranolaError::InvalidArguments(_) => 5,
            GranolaError::IoError(_) => 1,
            GranolaError::JsonError(_) => 3,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "error": {
                "code": self.exit_code(),
                "type": self.error_type(),
                "message": self.error_message(),
                "suggestion": self.suggestion(),
            }
        })
    }

    fn error_type(&self) -> &str {
        match self {
            GranolaError::CacheNotFound(_) => "cache_not_found",
            GranolaError::InvalidCacheFormat(_) => "invalid_cache_format",
            GranolaError::MeetingNotFound(_) => "meeting_not_found",
            GranolaError::TranscriptNotFound(_) => "transcript_not_found",
            GranolaError::InvalidArguments(_) => "invalid_arguments",
            GranolaError::IoError(_) => "io_error",
            GranolaError::JsonError(_) => "json_error",
        }
    }

    fn error_message(&self) -> String {
        match self {
            GranolaError::CacheNotFound(_) => "Cache file not found".to_string(),
            GranolaError::InvalidCacheFormat(msg) => format!("Invalid cache format: {}", msg),
            GranolaError::MeetingNotFound(id) => format!("Meeting not found: {}", id),
            GranolaError::TranscriptNotFound(id) => format!("Transcript not found for meeting: {}", id),
            GranolaError::InvalidArguments(msg) => format!("Invalid arguments: {}", msg),
            GranolaError::IoError(e) => format!("IO error: {}", e),
            GranolaError::JsonError(e) => format!("JSON parsing error: {}", e),
        }
    }

    fn suggestion(&self) -> Option<String> {
        match self {
            GranolaError::CacheNotFound(_) => Some("Ensure Granola is installed and has been run at least once".to_string()),
            GranolaError::InvalidCacheFormat(_) => Some("Cache might be corrupted or version mismatch".to_string()),
            GranolaError::MeetingNotFound(_) => Some("Use 'granola search' to find valid meeting IDs".to_string()),
            GranolaError::TranscriptNotFound(_) => Some("This meeting may not have been transcribed".to_string()),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, GranolaError>;
```

**Step 2: Add module declaration to main.rs**

In `src/main.rs`, add at the top:

```rust
mod error;

use error::{GranolaError, Result};
```

**Step 3: Test compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 4: Commit error types**

Run:
```bash
git add src/error.rs src/main.rs
git commit -m "feat: add custom error types with exit codes"
```

---

## Task 3: Data Models (Cache & Output Structures)

**Files:**
- Create: `src/models.rs`
- Modify: `src/main.rs` (add mod declaration)

**Step 1: Create models module with cache structures**

Create `src/models.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Cache Input Structures (matches Granola format)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CacheDocument {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(rename = "type")]
    pub doc_type: Option<String>,
    pub notes_plain: Option<String>,
    pub notes_markdown: Option<String>,
    pub overview: Option<String>,
    pub people: Option<DocumentPeople>,
}

#[derive(Debug, Deserialize)]
pub struct DocumentPeople {
    pub title: Option<String>,
    pub creator: Option<Person>,
    pub attendees: Option<Vec<Person>>,
}

#[derive(Debug, Deserialize)]
pub struct Person {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TranscriptSegment {
    pub text: String,
    pub source: String,
    pub timestamp: i64,
}

// ============================================================================
// Output Structures (optimized for JSON output)
// ============================================================================

#[derive(Debug, Serialize)]
pub struct SearchOutput {
    pub query: String,
    pub total_matches: usize,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub date: String,
    pub participants: Vec<String>,
    pub summary: Option<String>,
    pub has_transcript: bool,
    pub has_notes: bool,
}

#[derive(Debug, Serialize)]
pub struct MeetingDetails {
    pub id: String,
    pub title: String,
    pub date: String,
    pub duration_minutes: Option<i32>,
    pub participants: Vec<ParticipantInfo>,
    #[serde(rename = "type")]
    pub meeting_type: String,
    pub has_transcript: bool,
    pub has_notes: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ParticipantInfo {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TranscriptOutput {
    pub id: String,
    pub title: String,
    pub duration_seconds: Option<i64>,
    pub speakers: Vec<String>,
    pub total_segments: usize,
    pub segments: Vec<CompactSegment>,
}

#[derive(Debug, Serialize)]
pub struct CompactSegment {
    pub s: String,  // speaker
    pub t: String,  // text
    pub ts: i64,    // timestamp
}

#[derive(Debug, Serialize)]
pub struct DocumentsOutput {
    pub id: String,
    pub title: String,
    pub total_documents: usize,
    pub documents: Vec<Document>,
}

#[derive(Debug, Serialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub doc_type: String,
    pub format: String,
    pub content: String,
    pub word_count: usize,
    pub created_at: String,
}

// ============================================================================
// Cache Container
// ============================================================================

#[derive(Debug)]
pub struct Cache {
    pub documents: HashMap<String, CacheDocument>,
    pub transcripts: HashMap<String, Vec<TranscriptSegment>>,
}
```

**Step 2: Add module declaration to main.rs**

In `src/main.rs`, add:

```rust
mod models;

use models::*;
```

**Step 3: Test compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 4: Commit models**

Run:
```bash
git add src/models.rs src/main.rs
git commit -m "feat: add data models for cache and output"
```

---

## Task 4: Cache Loading & Double-Parse Logic

**Files:**
- Create: `src/cache.rs`
- Modify: `src/main.rs` (add mod declaration)

**Step 1: Create cache module with double-parse logic**

Create `src/cache.rs`:

```rust
use crate::error::{GranolaError, Result};
use crate::models::{Cache, CacheDocument, TranscriptSegment};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn load_cache(cache_path: &PathBuf) -> Result<Cache> {
    // Read file
    let file_content = fs::read_to_string(cache_path)
        .map_err(|_| GranolaError::CacheNotFound(cache_path.display().to_string()))?;

    // First parse: outer JSON
    let raw: Value = serde_json::from_str(&file_content)?;

    // Extract "cache" string field
    let cache_str = raw["cache"]
        .as_str()
        .ok_or_else(|| GranolaError::InvalidCacheFormat("Missing 'cache' field".to_string()))?;

    // Second parse: inner JSON
    let inner: Value = serde_json::from_str(cache_str)?;

    // Extract "state" object
    let state = inner["state"]
        .as_object()
        .ok_or_else(|| GranolaError::InvalidCacheFormat("Missing 'state' field".to_string()))?;

    // Parse documents
    let documents = parse_documents(state)?;

    // Parse transcripts
    let transcripts = parse_transcripts(state)?;

    Ok(Cache {
        documents,
        transcripts,
    })
}

fn parse_documents(state: &serde_json::Map<String, Value>) -> Result<HashMap<String, CacheDocument>> {
    let mut documents = HashMap::new();

    if let Some(docs_value) = state.get("documents") {
        if let Some(docs_obj) = docs_value.as_object() {
            for (id, doc_value) in docs_obj {
                if let Ok(doc) = serde_json::from_value::<CacheDocument>(doc_value.clone()) {
                    documents.insert(id.clone(), doc);
                }
            }
        }
    }

    Ok(documents)
}

fn parse_transcripts(state: &serde_json::Map<String, Value>) -> Result<HashMap<String, Vec<TranscriptSegment>>> {
    let mut transcripts = HashMap::new();

    if let Some(trans_value) = state.get("transcripts") {
        if let Some(trans_obj) = trans_value.as_object() {
            for (id, segments_value) in trans_obj {
                if let Some(segments_array) = segments_value.as_array() {
                    let segments: Vec<TranscriptSegment> = segments_array
                        .iter()
                        .filter_map(|v| serde_json::from_value(v.clone()).ok())
                        .collect();
                    if !segments.is_empty() {
                        transcripts.insert(id.clone(), segments);
                    }
                }
            }
        }
    }

    Ok(transcripts)
}

pub fn resolve_cache_path(cli_path: Option<PathBuf>) -> PathBuf {
    // Priority: CLI flag > env var > default
    if let Some(path) = cli_path {
        return path;
    }

    if let Ok(env_path) = std::env::var("GRANOLA_CACHE_PATH") {
        return PathBuf::from(env_path);
    }

    // Default path
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join("Library/Application Support/Granola/cache-v3.json")
}
```

**Step 2: Add module declaration to main.rs**

In `src/main.rs`, add:

```rust
mod cache;

use cache::{load_cache, resolve_cache_path};
```

**Step 3: Test compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 4: Commit cache loading**

Run:
```bash
git add src/cache.rs src/main.rs
git commit -m "feat: add cache loading with double-parse logic"
```

---

## Task 5: CLI Argument Parsing Setup

**Files:**
- Modify: `src/main.rs`

**Step 1: Define CLI structure with clap**

Replace `src/main.rs` content with:

```rust
mod error;
mod models;
mod cache;

use clap::{Parser, Subcommand};
use error::{GranolaError, Result};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "granola")]
#[command(about = "Query Granola meeting data", long_about = None)]
#[command(after_help = "For AI assistants: Run `granola workflow` for usage patterns, output schemas, and best practices.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to cache file (overrides env var and default)
    #[arg(long, global = true)]
    cache_path: Option<PathBuf>,

    /// Output errors as JSON to stdout (default: stderr)
    #[arg(long, global = true)]
    json_errors: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Search meetings by query
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(long, default_value = "30")]
        limit: usize,
    },

    /// Get meeting metadata
    Details {
        /// Meeting ID
        meeting_id: String,
    },

    /// Get meeting transcript
    Transcript {
        /// Meeting ID
        meeting_id: String,
    },

    /// Get meeting notes/documents
    Documents {
        /// Meeting ID
        meeting_id: String,
    },

    /// Show usage patterns (for AI assistants)
    Workflow,
}

fn main() {
    let cli = Cli::parse();

    let result = run(cli);

    match result {
        Ok(output) => {
            println!("{}", output);
            std::process::exit(0);
        }
        Err(e) => {
            if cli.json_errors {
                println!("{}", serde_json::to_string_pretty(&e.to_json()).unwrap());
            } else {
                eprintln!("{}", e);
            }
            std::process::exit(e.exit_code());
        }
    }
}

fn run(cli: Cli) -> Result<String> {
    // Handle workflow command separately (no cache needed)
    if matches!(cli.command, Commands::Workflow) {
        return Ok("Workflow guide placeholder".to_string());
    }

    // Load cache for other commands
    let cache_path = cache::resolve_cache_path(cli.cache_path);
    let cache = cache::load_cache(&cache_path)?;

    // Dispatch to appropriate command
    match cli.command {
        Commands::Search { query, limit } => {
            // Placeholder
            Ok(format!("{{\"query\": \"{}\", \"results\": []}}", query))
        }
        Commands::Details { meeting_id } => {
            // Placeholder
            Ok(format!("{{\"id\": \"{}\"}}", meeting_id))
        }
        Commands::Transcript { meeting_id } => {
            // Placeholder
            Ok(format!("{{\"id\": \"{}\"}}", meeting_id))
        }
        Commands::Documents { meeting_id } => {
            // Placeholder
            Ok(format!("{{\"id\": \"{}\"}}", meeting_id))
        }
        Commands::Workflow => unreachable!(),
    }
}
```

**Step 2: Test CLI help output**

Run: `cargo run -- --help`
Expected: Shows help with commands and global flags

**Step 3: Test CLI with placeholder command**

Run: `cargo run -- search "test"`
Expected: Returns placeholder JSON

**Step 4: Commit CLI setup**

Run:
```bash
git add src/main.rs
git commit -m "feat: add CLI argument parsing with clap"
```

---

## Task 6: Implement Search Command

**Files:**
- Create: `src/commands/mod.rs`
- Create: `src/commands/search.rs`
- Modify: `src/main.rs`

**Step 1: Create commands module structure**

Run: `mkdir -p src/commands`

Create `src/commands/mod.rs`:

```rust
pub mod search;

pub use search::search_meetings;
```

**Step 2: Implement search command**

Create `src/commands/search.rs`:

```rust
use crate::error::Result;
use crate::models::{Cache, SearchOutput, SearchResult};

pub fn search_meetings(query: &str, limit: usize, cache: &Cache) -> Result<SearchOutput> {
    let query_lower = query.to_lowercase();

    let mut results: Vec<SearchResult> = cache
        .documents
        .values()
        .filter(|doc| matches_query(doc, &query_lower))
        .map(|doc| {
            let participants = extract_participants(doc);
            SearchResult {
                id: doc.id.clone(),
                title: doc.title.clone(),
                date: doc.created_at.clone(),
                participants,
                summary: doc.overview.clone(),
                has_transcript: cache.transcripts.contains_key(&doc.id),
                has_notes: doc.notes_plain.is_some() || doc.notes_markdown.is_some(),
            }
        })
        .collect();

    // Sort by date (newest first)
    results.sort_by(|a, b| b.date.cmp(&a.date));

    // Apply limit
    results.truncate(limit);

    let total_matches = results.len();

    Ok(SearchOutput {
        query: query.to_string(),
        total_matches,
        results,
    })
}

fn matches_query(doc: &crate::models::CacheDocument, query_lower: &str) -> bool {
    // Check title
    if doc.title.to_lowercase().contains(query_lower) {
        return true;
    }

    // Check overview
    if let Some(overview) = &doc.overview {
        if overview.to_lowercase().contains(query_lower) {
            return true;
        }
    }

    // Check participants
    if let Some(people) = &doc.people {
        if let Some(attendees) = &people.attendees {
            for attendee in attendees {
                if attendee.name.to_lowercase().contains(query_lower) {
                    return true;
                }
            }
        }
        if let Some(creator) = &people.creator {
            if creator.name.to_lowercase().contains(query_lower) {
                return true;
            }
        }
    }

    false
}

fn extract_participants(doc: &crate::models::CacheDocument) -> Vec<String> {
    let mut participants = Vec::new();

    if let Some(people) = &doc.people {
        if let Some(creator) = &people.creator {
            participants.push(creator.name.clone());
        }
        if let Some(attendees) = &people.attendees {
            for attendee in attendees {
                if !participants.contains(&attendee.name) {
                    participants.push(attendee.name.clone());
                }
            }
        }
    }

    participants
}
```

**Step 3: Wire search into main.rs**

In `src/main.rs`, add at top:

```rust
mod commands;
```

Replace the search placeholder in `run()` with:

```rust
Commands::Search { query, limit } => {
    let output = commands::search_meetings(&query, limit, &cache)?;
    Ok(serde_json::to_string_pretty(&output)?)
}
```

**Step 4: Test compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 5: Test search with real cache (if available)**

Run: `cargo run -- search "test" --limit 5`
Expected: Returns JSON with search results or empty array

**Step 6: Commit search command**

Run:
```bash
git add src/commands/ src/main.rs
git commit -m "feat: implement search command"
```

---

## Task 7: Implement Details Command

**Files:**
- Create: `src/commands/details.rs`
- Modify: `src/commands/mod.rs`
- Modify: `src/main.rs`

**Step 1: Implement details command**

Create `src/commands/details.rs`:

```rust
use crate::error::{GranolaError, Result};
use crate::models::{Cache, MeetingDetails, ParticipantInfo};

pub fn get_meeting_details(meeting_id: &str, cache: &Cache) -> Result<MeetingDetails> {
    let doc = cache
        .documents
        .get(meeting_id)
        .ok_or_else(|| GranolaError::MeetingNotFound(meeting_id.to_string()))?;

    let duration_minutes = calculate_duration(meeting_id, cache);
    let participants = extract_participants(doc);

    Ok(MeetingDetails {
        id: doc.id.clone(),
        title: doc.title.clone(),
        date: doc.created_at.clone(),
        duration_minutes,
        participants,
        meeting_type: doc.doc_type.clone().unwrap_or_else(|| "unknown".to_string()),
        has_transcript: cache.transcripts.contains_key(&doc.id),
        has_notes: doc.notes_plain.is_some() || doc.notes_markdown.is_some(),
        created_at: doc.created_at.clone(),
        updated_at: doc.updated_at.clone(),
    })
}

fn calculate_duration(meeting_id: &str, cache: &Cache) -> Option<i32> {
    if let Some(segments) = cache.transcripts.get(meeting_id) {
        if let Some(last_segment) = segments.last() {
            // Convert seconds to minutes
            return Some((last_segment.timestamp / 60) as i32);
        }
    }
    None
}

fn extract_participants(doc: &crate::models::CacheDocument) -> Vec<ParticipantInfo> {
    let mut participants = Vec::new();

    if let Some(people) = &doc.people {
        if let Some(creator) = &people.creator {
            participants.push(ParticipantInfo {
                name: creator.name.clone(),
                email: creator.email.clone(),
            });
        }
        if let Some(attendees) = &people.attendees {
            for attendee in attendees {
                // Avoid duplicates
                if !participants.iter().any(|p| p.name == attendee.name) {
                    participants.push(ParticipantInfo {
                        name: attendee.name.clone(),
                        email: attendee.email.clone(),
                    });
                }
            }
        }
    }

    participants
}
```

**Step 2: Export details in commands module**

In `src/commands/mod.rs`, add:

```rust
pub mod details;

pub use details::get_meeting_details;
```

**Step 3: Wire details into main.rs**

Replace the details placeholder in `run()` with:

```rust
Commands::Details { meeting_id } => {
    let output = commands::get_meeting_details(&meeting_id, &cache)?;
    Ok(serde_json::to_string_pretty(&output)?)
}
```

**Step 4: Test compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 5: Commit details command**

Run:
```bash
git add src/commands/details.rs src/commands/mod.rs src/main.rs
git commit -m "feat: implement details command"
```

---

## Task 8: Implement Transcript Command

**Files:**
- Create: `src/commands/transcript.rs`
- Modify: `src/commands/mod.rs`
- Modify: `src/main.rs`

**Step 1: Implement transcript command**

Create `src/commands/transcript.rs`:

```rust
use crate::error::{GranolaError, Result};
use crate::models::{Cache, CompactSegment, TranscriptOutput};
use std::collections::HashSet;

pub fn get_transcript(meeting_id: &str, cache: &Cache) -> Result<TranscriptOutput> {
    let doc = cache
        .documents
        .get(meeting_id)
        .ok_or_else(|| GranolaError::MeetingNotFound(meeting_id.to_string()))?;

    let segments = cache
        .transcripts
        .get(meeting_id)
        .ok_or_else(|| GranolaError::TranscriptNotFound(meeting_id.to_string()))?;

    // Extract unique speakers
    let speakers: HashSet<String> = segments.iter().map(|s| s.source.clone()).collect();
    let speakers: Vec<String> = speakers.into_iter().collect();

    // Convert to compact format
    let compact_segments: Vec<CompactSegment> = segments
        .iter()
        .map(|s| CompactSegment {
            s: s.source.clone(),
            t: s.text.clone(),
            ts: s.timestamp,
        })
        .collect();

    let duration_seconds = segments.last().map(|s| s.timestamp);

    Ok(TranscriptOutput {
        id: doc.id.clone(),
        title: doc.title.clone(),
        duration_seconds,
        speakers,
        total_segments: compact_segments.len(),
        segments: compact_segments,
    })
}
```

**Step 2: Export transcript in commands module**

In `src/commands/mod.rs`, add:

```rust
pub mod transcript;

pub use transcript::get_transcript;
```

**Step 3: Wire transcript into main.rs**

Replace the transcript placeholder in `run()` with:

```rust
Commands::Transcript { meeting_id } => {
    let output = commands::get_transcript(&meeting_id, &cache)?;
    Ok(serde_json::to_string_pretty(&output)?)
}
```

**Step 4: Test compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 5: Commit transcript command**

Run:
```bash
git add src/commands/transcript.rs src/commands/mod.rs src/main.rs
git commit -m "feat: implement transcript command"
```

---

## Task 9: Implement Documents Command

**Files:**
- Create: `src/commands/documents.rs`
- Modify: `src/commands/mod.rs`
- Modify: `src/main.rs`

**Step 1: Implement documents command**

Create `src/commands/documents.rs`:

```rust
use crate::error::{GranolaError, Result};
use crate::models::{Cache, Document, DocumentsOutput};

pub fn get_documents(meeting_id: &str, cache: &Cache) -> Result<DocumentsOutput> {
    let doc = cache
        .documents
        .get(meeting_id)
        .ok_or_else(|| GranolaError::MeetingNotFound(meeting_id.to_string()))?;

    let mut documents = Vec::new();

    // Priority: notes_plain > notes_markdown
    if let Some(content) = &doc.notes_plain {
        documents.push(Document {
            id: format!("{}-notes", doc.id),
            title: format!("{} - Notes", doc.title),
            doc_type: "meeting_notes".to_string(),
            format: "plain".to_string(),
            content: content.clone(),
            word_count: content.split_whitespace().count(),
            created_at: doc.created_at.clone(),
        });
    } else if let Some(content) = &doc.notes_markdown {
        documents.push(Document {
            id: format!("{}-notes", doc.id),
            title: format!("{} - Notes", doc.title),
            doc_type: "meeting_notes".to_string(),
            format: "markdown".to_string(),
            content: content.clone(),
            word_count: content.split_whitespace().count(),
            created_at: doc.created_at.clone(),
        });
    }

    // Add overview as separate document if present
    if let Some(overview) = &doc.overview {
        documents.push(Document {
            id: format!("{}-overview", doc.id),
            title: format!("{} - Overview", doc.title),
            doc_type: "overview".to_string(),
            format: "plain".to_string(),
            content: overview.clone(),
            word_count: overview.split_whitespace().count(),
            created_at: doc.created_at.clone(),
        });
    }

    Ok(DocumentsOutput {
        id: doc.id.clone(),
        title: doc.title.clone(),
        total_documents: documents.len(),
        documents,
    })
}
```

**Step 2: Export documents in commands module**

In `src/commands/mod.rs`, add:

```rust
pub mod documents;

pub use documents::get_documents;
```

**Step 3: Wire documents into main.rs**

Replace the documents placeholder in `run()` with:

```rust
Commands::Documents { meeting_id } => {
    let output = commands::get_documents(&meeting_id, &cache)?;
    Ok(serde_json::to_string_pretty(&output)?)
}
```

**Step 4: Test compilation**

Run: `cargo check`
Expected: Compiles successfully

**Step 5: Commit documents command**

Run:
```bash
git add src/commands/documents.rs src/commands/mod.rs src/main.rs
git commit -m "feat: implement documents command"
```

---

## Task 10: Implement Workflow Command

**Files:**
- Create: `src/commands/workflow.rs`
- Modify: `src/commands/mod.rs`
- Modify: `src/main.rs`

**Step 1: Implement workflow command with embedded guide**

Create `src/commands/workflow.rs`:

```rust
pub fn get_workflow_guide() -> String {
    WORKFLOW_GUIDE.to_string()
}

const WORKFLOW_GUIDE: &str = r#"# Granola CLI Workflow Guide

## Purpose
Query local Granola meeting data via CLI. Optimized for LLM consumption with JSON output.

## Commands Overview
- `search <query>` - Find meetings (returns summaries for context)
- `details <id>` - Get meeting metadata
- `transcript <id>` - Get full conversation with speakers
- `documents <id>` - Get notes and overviews

## Common Patterns

### Finding and Reading a Meeting
1. Search: `granola search "moose dx"`
2. Review results, pick relevant meeting ID
3. Get details: `granola details <id>` (optional, for metadata)
4. Get content: `granola transcript <id>` or `granola documents <id>`

### Finding Recent Meetings with Someone
`granola search "Dave" --limit 20`

### Getting Full Context for a Meeting
```bash
# Get all three in sequence
granola details <id>
granola transcript <id>
granola documents <id>
```

## Token Optimization Tips

1. **Use --limit wisely**: Default is 30. Lower it if you just need recent matches.
2. **Search returns summaries**: Use these to filter before fetching transcripts
3. **Transcripts are large**: 25k+ chars typical. Only fetch when needed.
4. **Documents are smaller**: Notes/overviews are more compact than transcripts
5. **Check has_transcript/has_notes**: Avoid fetching what doesn't exist

## Output Schemas

### Search Output
```json
{
  "query": "string",
  "total_matches": 0,
  "results": [{
    "id": "uuid",
    "title": "string",
    "date": "ISO-8601",
    "participants": ["string"],
    "summary": "string | null",
    "has_transcript": boolean,
    "has_notes": boolean
  }]
}
```

### Details Output
```json
{
  "id": "uuid",
  "title": "string",
  "date": "ISO-8601",
  "duration_minutes": number | null,
  "participants": [{"name": "string", "email": "string | null"}],
  "type": "string",
  "has_transcript": boolean,
  "has_notes": boolean,
  "created_at": "ISO-8601",
  "updated_at": "ISO-8601"
}
```

### Transcript Output
```json
{
  "id": "uuid",
  "title": "string",
  "duration_seconds": number | null,
  "speakers": ["string"],
  "total_segments": number,
  "segments": [
    {"s": "speaker", "t": "text", "ts": timestamp_seconds}
  ]
}
```
*Note: segments use short keys (s/t/ts) for token efficiency*

### Documents Output
```json
{
  "id": "uuid",
  "title": "string",
  "total_documents": number,
  "documents": [{
    "id": "uuid",
    "title": "string",
    "type": "meeting_notes | overview",
    "format": "plain | markdown",
    "content": "string",
    "word_count": number,
    "created_at": "ISO-8601"
  }]
}
```

## Error Handling

### Default (stderr + exit codes)
- Success: exit 0, JSON to stdout
- Failure: exit 1-5, error message to stderr

### JSON Error Mode
Use `--json-errors` to get structured errors on stdout:
```json
{
  "error": {
    "code": number,
    "type": "error_type",
    "message": "string",
    "suggestion": "string"
  }
}
```

### Exit Codes
- 0: Success
- 1: General error
- 2: Cache file not found
- 3: Invalid cache format
- 4: Meeting not found
- 5: Invalid arguments

## Configuration

### Cache Path Priority
1. `--cache-path <PATH>` flag
2. `GRANOLA_CACHE_PATH` environment variable
3. Default: `~/Library/Application Support/Granola/cache-v3.json`

## Examples

### Search and fetch transcript
```bash
# Find meetings
granola search "roadmap planning" --limit 5

# Get transcript for first result
granola transcript a36d4c3e-7f02-4685-b0fd-98a2aac435f4
```

### Error handling in scripts
```bash
if granola details <id> 2>/dev/null; then
  echo "Meeting found"
else
  echo "Meeting not found"
fi
```

### Using JSON errors for parsing
```bash
result=$(granola search "foo" --json-errors)
# Parse result with jq, handle both success and error JSON
```
"#;
```

**Step 2: Export workflow in commands module**

In `src/commands/mod.rs`, add:

```rust
pub mod workflow;

pub use workflow::get_workflow_guide;
```

**Step 3: Wire workflow into main.rs**

Replace the workflow placeholder in `run()` with:

```rust
Commands::Workflow => {
    Ok(commands::get_workflow_guide())
}
```

**Step 4: Test workflow output**

Run: `cargo run -- workflow`
Expected: Outputs full markdown guide

**Step 5: Commit workflow command**

Run:
```bash
git add src/commands/workflow.rs src/commands/mod.rs src/main.rs
git commit -m "feat: implement workflow command with guide"
```

---

## Task 11: Fix JSON Error Handling Bug

**Files:**
- Modify: `src/main.rs`

**Step 1: Fix json_errors flag access in main**

The `cli.json_errors` field is not accessible in the error handling path. We need to restructure:

Replace the `main()` function with:

```rust
fn main() {
    let cli = Cli::parse();
    let json_errors = cli.json_errors;

    let result = run(cli);

    match result {
        Ok(output) => {
            println!("{}", output);
            std::process::exit(0);
        }
        Err(e) => {
            if json_errors {
                println!("{}", serde_json::to_string_pretty(&e.to_json()).unwrap());
            } else {
                eprintln!("{}", e);
            }
            std::process::exit(e.exit_code());
        }
    }
}
```

**Step 2: Test error handling**

Run: `cargo run -- details nonexistent-id`
Expected: Stderr error message, exit code 4

Run: `cargo run -- details nonexistent-id --json-errors`
Expected: JSON error to stdout, exit code 4

**Step 3: Commit fix**

Run:
```bash
git add src/main.rs
git commit -m "fix: json-errors flag handling in error path"
```

---

## Task 12: Integration Testing & Verification

**Files:**
- Create: `tests/integration_test.rs`

**Step 1: Create integration test file**

Create `tests/integration_test.rs`:

```rust
use std::process::Command;

#[test]
fn test_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Query Granola meeting data"));
    assert!(stdout.contains("search"));
    assert!(stdout.contains("workflow"));
}

#[test]
fn test_workflow_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "workflow"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Granola CLI Workflow Guide"));
    assert!(stdout.contains("Token Optimization Tips"));
}

#[test]
fn test_cache_not_found_error() {
    let output = Command::new("cargo")
        .args(&["run", "--", "search", "test", "--cache-path", "/nonexistent/path.json"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Cache file not found"));
}

#[test]
fn test_json_errors_flag() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "search",
            "test",
            "--cache-path",
            "/nonexistent/path.json",
            "--json-errors",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"error\""));
    assert!(stdout.contains("\"code\""));
    assert!(stdout.contains("cache_not_found"));
}
```

**Step 2: Run integration tests**

Run: `cargo test`
Expected: All tests pass

**Step 3: Commit integration tests**

Run:
```bash
git add tests/integration_test.rs
git commit -m "test: add integration tests for CLI"
```

---

## Task 13: Build & Manual Testing

**Files:**
- None (testing only)

**Step 1: Build release binary**

Run: `cargo build --release`
Expected: Binary created at `target/release/granola`

**Step 2: Test with real cache (if available)**

Run:
```bash
./target/release/granola search "test" --limit 5
./target/release/granola workflow
```

Expected: Commands work with real cache or show appropriate errors

**Step 3: Test error scenarios**

```bash
# Cache not found
./target/release/granola search "test" --cache-path /nonexistent

# Meeting not found
./target/release/granola details invalid-id

# JSON errors
./target/release/granola details invalid-id --json-errors
```

Expected: Appropriate error messages and exit codes

**Step 4: Document testing results**

Create notes about what worked and what needs adjustment based on real cache testing.

---

## Task 14: Final Documentation & Cleanup

**Files:**
- Modify: `README.md`
- Create: `.gitignore`

**Step 1: Update README with installation instructions**

Replace `README.md` with:

```markdown
# Granola CLI

A command-line tool for querying Granola meeting data. Optimized for AI assistant consumption with JSON output.

## Installation

### From Source

```bash
git clone <repository>
cd granola-cli
cargo build --release
cp target/release/granola /usr/local/bin/  # Or add to PATH
```

### Using Nix

```bash
nix develop  # Enter development shell
cargo build --release
```

## Usage

### Search Meetings

```bash
granola search "moose" --limit 10
```

Returns JSON with matching meetings, including summaries for LLM context.

### Get Meeting Details

```bash
granola details <meeting-id>
```

Returns metadata about a specific meeting.

### Get Transcript

```bash
granola transcript <meeting-id>
```

Returns full conversation with speakers. Note: Transcripts are large (25k+ chars typical).

### Get Documents/Notes

```bash
granola documents <meeting-id>
```

Returns meeting notes and overviews. More compact than transcripts.

### Workflow Guide (For AI Assistants)

```bash
granola workflow
```

Outputs comprehensive markdown guide with usage patterns, schemas, and tips.

## Configuration

### Cache File Location

Priority order:
1. `--cache-path <PATH>` command-line flag
2. `GRANOLA_CACHE_PATH` environment variable
3. Default: `~/Library/Application Support/Granola/cache-v3.json`

Example:
```bash
export GRANOLA_CACHE_PATH=/custom/path/cache.json
granola search "test"
```

### Error Output Format

Default: Human-readable errors to stderr
```bash
granola details invalid-id
# Outputs to stderr, exit code 4
```

JSON errors: Use `--json-errors` flag
```bash
granola details invalid-id --json-errors
# Outputs JSON to stdout, exit code 4
```

## For AI Assistants

This tool is designed for AI assistant consumption. Key features:

- **JSON Output**: All commands return structured JSON
- **Token Optimization**: Compact schemas in high-frequency data
- **Self-Documenting**: Run `granola workflow` for complete usage guide
- **Unix Conventions**: Exit codes, stdout/stderr separation

Run `granola workflow` to get detailed schemas, patterns, and tips.

## Exit Codes

- `0` - Success
- `1` - General error
- `2` - Cache file not found
- `3` - Invalid cache format
- `4` - Meeting/transcript not found
- `5` - Invalid arguments

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Run Locally

```bash
cargo run -- search "test"
```

## Related Documentation

- **Design Document**: `docs/plans/2025-11-15-granola-cli-design.md`
- **Implementation Plan**: `docs/plans/2025-11-15-granola-cli-implementation.md`
- **AI Assistant Context**: `CLAUDE.md`
- **Cache Format Analysis**: `/Users/lucio/Library/Application Support/Granola/CONTEXT.md`
```

**Step 2: Create .gitignore**

Create `.gitignore`:

```
/target
Cargo.lock
.DS_Store
*.swp
*.swo
*~
```

**Step 3: Final build verification**

Run: `cargo build --release && cargo test`
Expected: Clean build and all tests pass

**Step 4: Final commit**

Run:
```bash
git add README.md .gitignore
git commit -m "docs: update README and add gitignore"
```

---

## Completion Checklist

After all tasks are complete, verify:

- [ ] All commands implemented: search, details, transcript, documents, workflow
- [ ] Error handling works with both stderr and `--json-errors`
- [ ] Cache path resolution: CLI flag > env var > default
- [ ] Exit codes correct for all error types
- [ ] Integration tests pass
- [ ] Release binary builds successfully
- [ ] Documentation complete: README, CLAUDE.md, design doc
- [ ] Manual testing with real cache (if available)

**Final Step: Use superpowers:finishing-a-development-branch**

After verification, announce: "I'm using the finishing-a-development-branch skill to complete this work."
