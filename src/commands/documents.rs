use crate::error::{GranolaError, Result};
use crate::models::{Cache, Document, DocumentsOutput};

pub fn get_documents(meeting_id: &str, cache: &Cache) -> Result<DocumentsOutput> {
    let doc = cache
        .documents
        .get(meeting_id)
        .ok_or_else(|| GranolaError::MeetingNotFound(meeting_id.to_string()))?;

    let mut documents = Vec::new();

    // Priority: notes_plain > notes_markdown
    if let Some(content) = &doc.notes_plain {
        documents.push(Document {
            id: format!("{}-notes", doc.id),
            title: format!("{} - Notes", doc.title),
            doc_type: "meeting_notes".to_string(),
            format: "plain".to_string(),
            content: content.clone(),
            word_count: content.split_whitespace().count(),
            created_at: doc.created_at.clone(),
        });
    } else if let Some(content) = &doc.notes_markdown {
        documents.push(Document {
            id: format!("{}-notes", doc.id),
            title: format!("{} - Notes", doc.title),
            doc_type: "meeting_notes".to_string(),
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
            doc_type: "overview".to_string(),
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
