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
