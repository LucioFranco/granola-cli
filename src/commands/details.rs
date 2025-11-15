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
        meeting_type: doc
            .doc_type
            .clone()
            .unwrap_or_else(|| "unknown".to_string()),
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
