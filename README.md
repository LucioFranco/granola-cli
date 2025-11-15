# Granola CLI

A command-line tool for querying Granola meeting data. Optimized for AI assistant consumption with JSON output.

## Installation

### From GitHub Releases (Recommended)

Download pre-built binaries from the [Releases](https://github.com/luciofranco/granola-cli/releases) page:

```bash
# macOS (Apple Silicon)
curl -L https://github.com/luciofranco/granola-cli/releases/latest/download/granola-cli-aarch64-apple-darwin.tar.gz | tar xz
sudo mv granola-cli /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/luciofranco/granola-cli/releases/latest/download/granola-cli-x86_64-apple-darwin.tar.gz | tar xz
sudo mv granola-cli /usr/local/bin/

# Linux (x86_64)
curl -L https://github.com/luciofranco/granola-cli/releases/latest/download/granola-cli-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv granola-cli /usr/local/bin/
```

### From Source

```bash
git clone https://github.com/luciofranco/granola-cli.git
cd granola-cli
cargo build --release
cp target/release/granola-cli /usr/local/bin/
```

### Using Nix

```bash
nix develop  # Enter development shell
cargo build --release
```

### Claude Code Skill (Optional)

To enable Claude Code to automatically use this tool:

```bash
# Install the skill
cp .claude/skills/using-granola-cli.md ~/.claude/skills/

# Or symlink for auto-updates
ln -s $(pwd)/.claude/skills/using-granola-cli.md ~/.claude/skills/
```

Now Claude Code can use `granola-cli` when you ask about your Granola meetings!

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
