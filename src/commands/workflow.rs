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
1. Search: `granola search "project topic"`
2. Review results, pick relevant meeting ID
3. Get details: `granola details <id>` (optional, for metadata)
4. Get content: `granola transcript <id>` or `granola documents <id>`

### Finding Recent Meetings with Someone
`granola search "person_name" --limit 20`

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
