# Granola CLI Design

**Date:** 2025-11-15
**Status:** Approved

## Overview & Purpose

### What We're Building

A Rust-based CLI tool that replicates the Granola AI MCP server's functionality, optimized for LLM consumption via direct command invocation. The tool reads Granola's local cache file and outputs JSON data that Claude Code (or other AI assistants) can consume directly through Bash commands, eliminating the need for MCP server infrastructure.

### Core Design Principles

1. **LLM-First**: JSON output by default, optimized schemas for token efficiency and clarity
2. **Self-Documenting**: Built-in workflow guide helps LLMs discover and use the tool correctly
3. **Unix Philosophy**: Clear separation of concerns (stdout for data, stderr for errors, exit codes for status)
4. **Zero Config**: Works out-of-the-box with sensible defaults, configurable when needed

### Architecture Overview

```
┌─────────────────┐
│  Claude Code    │
│  (LLM)          │
└────────┬────────┘
         │
         │ Bash: granola search "moose"
         ↓
┌─────────────────┐
│  granola CLI    │
│  (Rust binary)  │
└────────┬────────┘
         │
         │ Read & parse
         ↓
┌─────────────────────────────────┐
│  ~/Library/Application Support/ │
│  Granola/cache-v3.json          │
└─────────────────────────────────┘
```

## Command Structure & Interface

### Commands

Flat, action-based structure with 5 core commands:

```bash
granola search <query> [--limit N]
granola details <meeting-id>
granola transcript <meeting-id>
granola documents <meeting-id>
granola workflow
```

### Global Flags

```bash
--cache-path <PATH>    # Override cache location
--json-errors          # Output errors as JSON to stdout (default: stderr)
--help, -h             # Show help
--version, -v          # Show version
```

### Cache File Resolution Priority

1. CLI flag: `--cache-path /custom/path.json`
2. Environment variable: `GRANOLA_CACHE_PATH`
3. Default: `~/Library/Application Support/Granola/cache-v3.json`

### Help System Design

**`granola --help`** - Human-readable overview with LLM signpost:
```
granola-cli - Query Granola meeting data

USAGE:
    granola <COMMAND>

COMMANDS:
    search      Search meetings by query
    details     Get meeting metadata
    transcript  Get meeting transcript
    documents   Get meeting notes/documents
    workflow    Show usage patterns (recommended for AI assistants)

For AI assistants: Run `granola workflow` for usage patterns,
output schemas, and best practices.
```

**`granola workflow`** - Comprehensive markdown guide including:
- Tool purpose and capabilities
- Common usage patterns (search → details → transcript)
- Output schema documentation for each command
- Token optimization tips (use --limit, chain commands efficiently)
- Error handling guidance (--json-errors flag)
- Real command examples with sample output

## JSON Output Schemas

### Design Philosophy: Context-Aware Naming

- **Top-level fields**: Clear, descriptive names (`query`, `meeting_id`, `relevance_score`)
- **High-frequency arrays**: Compact names to reduce token cost (`s` for speaker, `t` for text, `ts` for timestamp)
- **Self-documenting**: Include metadata that helps LLMs understand what to do next

### `granola search <query> [--limit N]`

**Algorithm**: Simple keyword filter (no scoring):
- Match query against: title, participants, overview/summary
- If match found → include it
- Sort by date (newest first)
- Default limit: 30

**Rationale**: LLMs are better at semantic ranking than crude keyword scoring. We filter to reduce token count, provide rich context (summaries), and let the LLM decide relevance.

```json
{
  "query": "moose",
  "total_matches": 12,
  "results": [
    {
      "id": "uuid",
      "title": "Dave/Oliva/Lucio - moose dx 0.2",
      "date": "2025-11-07T19:40:42.843Z",
      "participants": ["Dave", "Oliva", "Lucio"],
      "summary": "Discussed architecture decisions for moose 0.2...",
      "has_transcript": true,
      "has_notes": true
    }
  ]
}
```

### `granola details <meeting-id>`

```json
{
  "id": "uuid",
  "title": "Meeting title",
  "date": "2025-11-07T19:40:42.843Z",
  "duration_minutes": 45,
  "participants": [
    {
      "name": "Dave",
      "email": "dave@example.com"
    }
  ],
  "type": "scheduled",
  "has_transcript": true,
  "has_notes": true,
  "created_at": "2025-11-07T19:40:42.843Z",
  "updated_at": "2025-11-07T20:31:11.042Z"
}
```

### `granola transcript <meeting-id>`

```json
{
  "id": "uuid",
  "title": "Moose DX Discussion",
  "duration_seconds": 2700,
  "speakers": ["Dave", "Lucio"],
  "total_segments": 42,
  "segments": [
    {
      "s": "Dave",
      "t": "Let's discuss the moose architecture...",
      "ts": 0
    },
    {
      "s": "Lucio",
      "t": "Great idea, I think we should...",
      "ts": 15
    }
  ]
}
```

**Note**: `segments` array uses short keys (`s`, `t`, `ts`) because it repeats 100+ times. Saves ~200 tokens per transcript.

### `granola documents <meeting-id>`

```json
{
  "id": "uuid",
  "title": "Moose DX Discussion",
  "total_documents": 1,
  "documents": [
    {
      "id": "uuid",
      "title": "Meeting Notes",
      "type": "meeting_notes",
      "format": "markdown",
      "content": "# Discussion Points\n- Architecture...",
      "word_count": 450,
      "created_at": "2025-11-07T19:40:42.843Z"
    }
  ]
}
```

## Error Handling

### Default Behavior (Unix Conventions)

**Success:**
- Exit code: `0`
- Stdout: JSON data
- Stderr: empty

**Failure:**
- Exit code: `1` (general error) or specific codes
- Stdout: empty
- Stderr: human-readable error message

### Exit Codes

```
0   - Success
1   - General error
2   - Cache file not found
3   - Invalid cache format
4   - Meeting not found
5   - Invalid arguments
```

### Error Message Format (Stderr)

```
Error: Cache file not found
Path: /Users/lucio/Library/Application Support/Granola/cache-v3.json
Suggestion: Ensure Granola is installed and has been run at least once
```

### JSON Error Mode (`--json-errors`)

When flag is present, errors output to stdout as JSON:

```json
{
  "error": {
    "code": 2,
    "type": "cache_not_found",
    "message": "Cache file not found",
    "path": "/Users/lucio/Library/Application Support/Granola/cache-v3.json",
    "suggestion": "Ensure Granola is installed and has been run at least once"
  }
}
```

Exit codes still reflect the error type even with `--json-errors`.

### Common Error Scenarios

1. **Cache not found** → Exit 2, suggest checking Granola installation
2. **Invalid JSON** → Exit 3, suggest cache might be corrupted
3. **Meeting ID not found** → Exit 4, suggest using search first
4. **Invalid double-nested structure** → Exit 3, version mismatch issue
5. **Missing required fields** → Exit 3, cache format changed

## Cache Parsing & Data Model

### Cache File Structure

Double-nested JSON requiring two parse operations:
```json
{
  "cache": "{\"version\":4,\"state\":{...}}"
}
```

### Parsing Strategy

```rust
// 1. Read file and parse outer JSON
let raw: serde_json::Value = serde_json::from_str(&file_content)?;

// 2. Extract "cache" string field
let cache_str = raw["cache"].as_str()?;

// 3. Parse inner JSON string
let inner: serde_json::Value = serde_json::from_str(cache_str)?;

// 4. Extract "state" object containing all data
let state = inner["state"].as_object()?;
```

**Rationale**: Direct replication of Python MCP server's proven approach. The format is what it is, and this is fast (sub-2 seconds for 100+ meetings).

### Key Data Structures

**Cache State Object (57+ keys, we care about 3):**
- `state.documents` → Meeting metadata and notes
- `state.transcripts` → Meeting transcriptions (arrays of segments)
- `state.people` → Contact information

**Internal Rust Structs:**

```rust
// Raw cache structures (matches Granola format)
struct CacheDocument {
    id: String,
    title: String,
    created_at: String,  // ISO-8601
    updated_at: String,
    notes_plain: Option<String>,
    notes_markdown: Option<String>,
    overview: Option<String>,
    people: Option<DocumentPeople>,
    // ... other fields
}

struct DocumentPeople {
    title: Option<String>,
    creator: Option<Person>,
    attendees: Vec<Person>,
}

struct Person {
    name: String,
    email: Option<String>,
}

struct TranscriptSegment {
    text: String,
    source: String,  // Speaker name
    timestamp: i64,  // Unix timestamp
}
```

**Output Structs (optimized for JSON output):**

```rust
// These serialize to our compact JSON schemas
struct SearchResult {
    id: String,
    title: String,
    date: String,
    participants: Vec<String>,
    summary: Option<String>,
    has_transcript: bool,
    has_notes: bool,
}

struct MeetingDetails {
    id: String,
    title: String,
    date: String,
    duration_minutes: Option<i32>,
    participants: Vec<ParticipantInfo>,
    #[serde(rename = "type")]
    meeting_type: String,
    has_transcript: bool,
    has_notes: bool,
    created_at: String,
    updated_at: String,
}

// Transcript uses custom serialization for compact output
struct TranscriptOutput {
    id: String,
    title: String,
    duration_seconds: Option<i64>,
    speakers: Vec<String>,
    total_segments: usize,
    segments: Vec<CompactSegment>,
}

struct CompactSegment {
    s: String,  // speaker
    t: String,  // text
    ts: i64,    // timestamp
}
```

## Command Implementations

### Search Command

**`granola search <query> [--limit 30]`**

**Algorithm:**
```rust
fn search_meetings(query: &str, limit: usize, cache: &Cache) -> Vec<SearchResult> {
    let query_lower = query.to_lowercase();

    cache.documents
        .iter()
        .filter(|doc| {
            // Simple presence check - no scoring
            doc.title.to_lowercase().contains(&query_lower)
            || doc.overview.as_ref().map_or(false, |o| o.to_lowercase().contains(&query_lower))
            || participants_contain(&doc.people, &query_lower)
        })
        .sorted_by(|a, b| b.created_at.cmp(&a.created_at))  // Newest first
        .take(limit)
        .map(|doc| SearchResult {
            id: doc.id.clone(),
            title: doc.title.clone(),
            date: doc.created_at.clone(),
            participants: extract_participant_names(&doc.people),
            summary: doc.overview.clone(),  // Key addition for LLM context
            has_transcript: cache.transcripts.contains_key(&doc.id),
            has_notes: doc.notes_plain.is_some() || doc.notes_markdown.is_some(),
        })
        .collect()
}
```

### Details Command

**`granola details <meeting-id>`**

**Algorithm:**
```rust
fn get_meeting_details(meeting_id: &str, cache: &Cache) -> Result<MeetingDetails> {
    let doc = cache.documents.get(meeting_id)
        .ok_or(Error::MeetingNotFound)?;

    Ok(MeetingDetails {
        id: doc.id.clone(),
        title: doc.title.clone(),
        date: doc.created_at.clone(),
        duration_minutes: calculate_duration(&doc, &cache.transcripts),
        participants: extract_participants(&doc.people),
        meeting_type: doc.type.clone(),
        has_transcript: cache.transcripts.contains_key(&doc.id),
        has_notes: doc.notes_plain.is_some() || doc.notes_markdown.is_some(),
        created_at: doc.created_at.clone(),
        updated_at: doc.updated_at.clone(),
    })
}
```

### Transcript Command

**`granola transcript <meeting-id>`**

**Algorithm:**
```rust
fn get_transcript(meeting_id: &str, cache: &Cache) -> Result<TranscriptOutput> {
    let doc = cache.documents.get(meeting_id)
        .ok_or(Error::MeetingNotFound)?;

    let segments = cache.transcripts.get(meeting_id)
        .ok_or(Error::TranscriptNotFound)?;

    let speakers: HashSet<String> = segments.iter()
        .map(|s| s.source.clone())
        .collect();

    Ok(TranscriptOutput {
        id: doc.id.clone(),
        title: doc.title.clone(),
        duration_seconds: segments.last().map(|s| s.timestamp),
        speakers: speakers.into_iter().collect(),
        total_segments: segments.len(),
        segments: segments.iter().map(|s| CompactSegment {
            s: s.source.clone(),
            t: s.text.clone(),
            ts: s.timestamp,
        }).collect(),
    })
}
```

### Documents Command

**`granola documents <meeting-id>`**

**Algorithm:**
```rust
fn get_documents(meeting_id: &str, cache: &Cache) -> Result<DocumentsOutput> {
    let doc = cache.documents.get(meeting_id)
        .ok_or(Error::MeetingNotFound)?;

    let mut documents = Vec::new();

    // Priority order: notes_plain > notes_markdown > overview
    if let Some(content) = &doc.notes_plain {
        documents.push(Document {
            id: format!("{}-notes", doc.id),
            title: format!("{} - Notes", doc.title),
            type: "meeting_notes".to_string(),
            format: "plain".to_string(),
            content: content.clone(),
            word_count: content.split_whitespace().count(),
            created_at: doc.created_at.clone(),
        });
    } else if let Some(content) = &doc.notes_markdown {
        documents.push(Document {
            id: format!("{}-notes", doc.id),
            title: format!("{} - Notes", doc.title),
            type: "meeting_notes".to_string(),
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
            type: "overview".to_string(),
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

## Workflow Guide Content

The `granola workflow` command outputs comprehensive markdown to help LLMs use the tool efficiently. Content includes:

### Sections

1. **Purpose** - What the tool does
2. **Commands Overview** - Quick reference
3. **Common Patterns** - Typical usage sequences
4. **Token Optimization Tips** - How to minimize token usage
5. **Output Schemas** - Detailed schema for each command
6. **Error Handling** - How errors work
7. **Configuration** - Cache path resolution
8. **Examples** - Real command sequences

### Key Content

**Common Patterns:**
- Finding and reading a meeting (search → details → transcript)
- Finding recent meetings with someone
- Getting full context for a meeting

**Token Optimization Tips:**
- Use `--limit` wisely (default is 30, lower if needed)
- Search returns summaries - use these to filter before fetching transcripts
- Transcripts are large (25k+ chars) - only fetch when needed
- Documents are smaller - notes/overviews more compact than transcripts
- Check `has_transcript`/`has_notes` flags - avoid fetching what doesn't exist

**Output Schemas:**
- Complete JSON schema for each command
- Field descriptions and types
- Note about compact keys in transcript segments

**Error Handling:**
- Default stderr behavior vs `--json-errors`
- Exit code meanings
- Example error JSON structure

**Examples:**
- Search and fetch transcript
- Error handling in scripts
- Using JSON errors for parsing

## Project Structure

```
granola-cli/
├── Cargo.toml
├── flake.nix
├── flake.lock
├── src/
│   ├── main.rs           # CLI entry point, arg parsing
│   ├── cache.rs          # Cache loading & double-parse logic
│   ├── models.rs         # Rust structs for cache & output
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── search.rs     # Search implementation
│   │   ├── details.rs    # Details command
│   │   ├── transcript.rs # Transcript command
│   │   ├── documents.rs  # Documents command
│   │   └── workflow.rs   # Workflow guide (embedded string)
│   ├── error.rs          # Error types & formatting
│   └── output.rs         # JSON serialization helpers
├── docs/
│   └── plans/
│       └── 2025-11-15-granola-cli-design.md  # This document
├── CLAUDE.md             # AI assistant context
└── README.md             # Basic usage for humans
```

## Dependencies

```toml
[package]
name = "granola-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
chrono = { version = "0.4", features = ["serde"] }
```

**Rationale**: Standard, battle-tested crates. Reliability and clear behavior matter more than compile time for this LLM-focused tool.

## Performance Targets

- **Cache load time**: <2 seconds for 1000 meetings
- **Search response**: <100ms after cache loaded
- **Binary size**: <5MB (reasonable for a CLI)
- **Memory usage**: <50MB for typical cache (100 meetings)

## Future Considerations

Not in v1, but documented for later:

- **Watch mode**: `granola watch` for cache updates
- **Export formats**: `--format csv` or `--format markdown`
- **Advanced filters**: `--from DATE --to DATE --participants "Dave"`
- **Analytics**: Implement the `analyze` command from MCP server
- **Streaming**: Large transcripts could stream to stdout
- **Caching**: In-memory cache for repeated queries in same session

## Design Decisions & Rationale

### Why JSON-only output?
- Tool is LLM-first, not human-first
- Simpler implementation (no formatting logic)
- Consistent parsing for AI consumers
- Humans can pipe to `jq` if needed

### Why no relevance scoring in search?
- LLMs are better at semantic understanding than keyword scoring
- Basic filtering (1000 → 30 meetings) provides value
- Rich context (summaries) lets LLM decide relevance
- Simpler implementation, fewer assumptions

### Why compact keys in transcript segments?
- Segments repeat 100+ times per transcript
- `timestamp_seconds` → `ts` saves ~200 tokens per transcript
- Transcripts are already large (25k+ chars)
- Top-level fields stay clear for context

### Why double-parse the cache?
- Proven approach from Python MCP server
- Cache format is what it is
- Performance is acceptable (sub-2s for 100+ meetings)
- No benefit to being clever here

### Why Rust over Python/Go?
- Fast JSON parsing (important for large caches)
- Single binary distribution (easy for users)
- Strong typing prevents runtime errors
- Good tooling ecosystem (cargo, clippy, rustfmt)

## Related Documentation

- **Cache Format Analysis**: `/Users/lucio/Library/Application Support/Granola/CONTEXT.md`
- **Python MCP Server**: `github.com/proofgeist/granola-ai-mcp-server`
- **MCP Protocol**: Model Context Protocol specification
- **Granola App**: granola.so
