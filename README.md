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
