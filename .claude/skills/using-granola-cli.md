---
name: using-granola-cli
description: Use when you need to search, retrieve, or analyze Granola meeting data - provides direct CLI access to meeting transcripts, notes, and metadata for LLM consumption
---

# Using Granola CLI

## Overview

The Granola CLI provides direct access to Granola meeting data through JSON output optimized for LLM consumption. Use this skill whenever you need to search meetings, retrieve transcripts, or access meeting notes.

## When to Use This Skill

Use this skill when you need to:
- Search for meetings by keyword, participant, or topic
- Retrieve meeting transcripts or notes
- Get meeting metadata (participants, duration, date)
- Analyze meeting content or patterns
- Access historical meeting data

## Available Commands

### 1. Workflow Guide (Start Here)

**Always start by checking the workflow guide:**
```bash
granola-cli workflow
```

This outputs comprehensive markdown with:
- Command overview and usage patterns
- Token optimization tips
- Complete output schemas
- Error handling guide
- Configuration options

**When to use:** First time using the CLI or when unsure about command usage.

### 2. Search Meetings

```bash
granola-cli search "<query>" [--limit N]
```

**Returns:** JSON with matching meetings, including summaries for context.

**Default limit:** 30 results
**Sorted by:** Newest first

**Output includes:**
- Meeting ID (for use with other commands)
- Title
- Date
- Participants
- Summary (if available)
- Flags: `has_transcript`, `has_notes`

**Use cases:**
- "Find meetings about the roadmap"
- "Search for meetings with Dave"
- "Find recent moose discussions"

**Token optimization tip:** Use `--limit 5` if you only need recent matches.

### 3. Get Meeting Details

```bash
granola-cli details <meeting-id>
```

**Returns:** Complete meeting metadata.

**Output includes:**
- Duration (minutes)
- Participants with emails
- Meeting type
- Created/updated timestamps
- Availability flags

**Use cases:**
- Get meeting duration before fetching transcript
- Check if transcript/notes are available
- Get participant list

### 4. Get Transcript

```bash
granola-cli transcript <meeting-id>
```

**Returns:** Full conversation with speakers.

**Output includes:**
- Speakers list
- Total segments count
- Duration (seconds)
- Segments with compact format (`s`, `t`, `ts` keys)

**⚠️ Token Warning:** Transcripts are large (typically 25k+ chars). Only fetch when needed.

**Use cases:**
- Analyze what was discussed in detail
- Quote specific conversation parts
- Understand context deeply

### 5. Get Documents/Notes

```bash
granola-cli documents <meeting-id>
```

**Returns:** Meeting notes and overviews.

**Output includes:**
- Meeting notes (plain or markdown)
- AI-generated overview (if available)
- Word counts
- Document types

**✅ Token Efficient:** More compact than transcripts, good for summaries.

**Use cases:**
- Get meeting summary quickly
- Access structured notes
- Review action items or key points

## Common Workflows

### Finding and Reading a Meeting

**Pattern: Search → Review → Fetch Details/Content**

```bash
# 1. Search for meetings
granola-cli search "moose architecture" --limit 5

# 2. Review results, pick relevant meeting ID
# (LLM: Look at titles, summaries, participants)

# 3. Get detailed content
granola-cli transcript <meeting-id>
# OR for faster/lighter:
granola-cli documents <meeting-id>
```

### Finding Recent Meetings with Someone

```bash
granola-cli search "Dave" --limit 10
```

The search matches participant names, so this returns meetings where Dave participated.

### Getting Full Context for a Meeting

```bash
# Get everything about a meeting
granola-cli details <meeting-id>
granola-cli transcript <meeting-id>
granola-cli documents <meeting-id>
```

**When to do this:** When you need complete understanding of a specific meeting.

## Token Optimization Strategy

### 1. Start Small
- Use `search` with low `--limit` first
- Check `has_transcript` and `has_notes` flags
- Decide what you actually need

### 2. Use Documents Over Transcripts
- Documents/notes are typically 1-5k chars
- Transcripts are typically 25k+ chars
- If summary is enough, use documents

### 3. Progressive Loading
```bash
# Step 1: Search (lightweight)
granola-cli search "topic" --limit 5

# Step 2: Details if needed (medium)
granola-cli details <id>

# Step 3: Content only if necessary
granola-cli documents <id>  # or transcript if details needed
```

## Error Handling

### Cache Not Found

**Error:** Exit code 2, "Cache file not found"

**Solutions:**
1. Check Granola is installed and has been run
2. Verify cache location: `~/Library/Application Support/Granola/cache-v3.json`
3. Set custom path: `--cache-path /custom/path.json`
4. Or use env var: `export GRANOLA_CACHE_PATH=/custom/path.json`

### Meeting Not Found

**Error:** Exit code 4, "Meeting not found"

**Solutions:**
1. Verify the meeting ID is correct (run search again)
2. Meeting might have been deleted from Granola
3. Try searching for the meeting to get updated ID

### Transcript Not Found

**Error:** Exit code 4, "Transcript not found for meeting"

**Reason:** Meeting wasn't transcribed (check `has_transcript: false`)

**Solution:** Use `documents` command instead for notes/overview.

## Configuration

### Cache Path Priority

1. CLI flag: `--cache-path <PATH>`
2. Environment variable: `GRANOLA_CACHE_PATH`
3. Default: `~/Library/Application Support/Granola/cache-v3.json`

### JSON Error Mode

Add `--json-errors` flag to get structured error output:

```bash
granola-cli search "test" --json-errors
```

Useful when you need to parse error details programmatically.

## Output Schema Reference

### Search Output
```json
{
  "query": "string",
  "total_matches": number,
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

**Note:** Segments use abbreviated keys (`s`, `t`, `ts`) for token efficiency.

## Best Practices

### 1. Check Workflow Guide First
Run `granola-cli workflow` if unsure about usage.

### 2. Use Search Summaries
The search command returns summaries - use these to decide if you need the full transcript.

### 3. Prefer Documents for Summaries
If you just need "what happened", use documents not transcript.

### 4. Batch Your Queries
If you need multiple meetings, search once with appropriate limit rather than multiple searches.

### 5. Check Availability Flags
Before fetching transcript/documents, check `has_transcript` and `has_notes` flags from search or details.

## Examples

### "What did we discuss about the moose project?"

```bash
# 1. Find moose meetings
granola-cli search "moose" --limit 5

# 2. Review summaries in search results
# 3. If summary is enough, done! If not:
granola-cli documents <most-relevant-id>

# 4. If still need more detail:
granola-cli transcript <most-relevant-id>
```

### "Who attended the meeting on November 11th?"

```bash
# 1. Search by date or topic from that day
granola-cli search "november 11" --limit 10

# 2. Get details for the right meeting
granola-cli details <meeting-id>

# Output includes participants list with emails
```

### "Give me a summary of all meetings this week"

```bash
# 1. Search broadly or use recent meetings
granola-cli search "" --limit 20

# 2. Review titles, dates, summaries from search results
# 3. For specific meetings, fetch documents for fuller summaries
```

## Troubleshooting

### "No results found"
- Try broader search terms
- Check the cache has data: `ls -lh ~/Library/Application\ Support/Granola/cache-v3.json`
- Verify Granola has recorded meetings

### "Transcript is empty"
- Some meetings don't have transcripts (check `has_transcript` flag)
- Use `documents` command for notes instead

### "Output is too large"
- Use `--limit` to reduce search results
- Use `documents` instead of `transcript`
- For transcripts, consider if you really need the full text or if a summary would work

## Remember

- **Always check availability flags** (`has_transcript`, `has_notes`) before fetching content
- **Start with search** - don't assume you know the meeting ID
- **Use documents for summaries** - save tokens when full transcript isn't needed
- **Leverage search summaries** - often enough context without fetching full content
- **Check workflow guide** when unsure: `granola-cli workflow`
