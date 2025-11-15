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
