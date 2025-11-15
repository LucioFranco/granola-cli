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
