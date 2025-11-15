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
