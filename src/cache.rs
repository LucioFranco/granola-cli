use crate::error::{GranolaError, Result};
use crate::models::{Cache, CacheDocument, TranscriptSegment};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn load_cache(cache_path: &PathBuf) -> Result<Cache> {
    // Read file
    let file_content = fs::read_to_string(cache_path)
        .map_err(|_| GranolaError::CacheNotFound(cache_path.display().to_string()))?;

    // First parse: outer JSON
    let raw: Value = serde_json::from_str(&file_content)?;

    // Extract "cache" string field
    let cache_str = raw["cache"]
        .as_str()
        .ok_or_else(|| GranolaError::InvalidCacheFormat("Missing 'cache' field".to_string()))?;

    // Second parse: inner JSON
    let inner: Value = serde_json::from_str(cache_str)?;

    // Extract "state" object
    let state = inner["state"]
        .as_object()
        .ok_or_else(|| GranolaError::InvalidCacheFormat("Missing 'state' field".to_string()))?;

    // Parse documents
    let documents = parse_documents(state)?;

    // Parse transcripts
    let transcripts = parse_transcripts(state)?;

    Ok(Cache {
        documents,
        transcripts,
    })
}

fn parse_documents(
    state: &serde_json::Map<String, Value>,
) -> Result<HashMap<String, CacheDocument>> {
    let mut documents = HashMap::new();

    if let Some(docs_value) = state.get("documents") {
        if let Some(docs_obj) = docs_value.as_object() {
            for (id, doc_value) in docs_obj {
                if let Ok(doc) = serde_json::from_value::<CacheDocument>(doc_value.clone()) {
                    documents.insert(id.clone(), doc);
                }
            }
        }
    }

    Ok(documents)
}

fn parse_transcripts(
    state: &serde_json::Map<String, Value>,
) -> Result<HashMap<String, Vec<TranscriptSegment>>> {
    let mut transcripts = HashMap::new();

    if let Some(trans_value) = state.get("transcripts") {
        if let Some(trans_obj) = trans_value.as_object() {
            for (id, segments_value) in trans_obj {
                if let Some(segments_array) = segments_value.as_array() {
                    let segments: Vec<TranscriptSegment> = segments_array
                        .iter()
                        .filter_map(|v| serde_json::from_value(v.clone()).ok())
                        .collect();
                    if !segments.is_empty() {
                        transcripts.insert(id.clone(), segments);
                    }
                }
            }
        }
    }

    Ok(transcripts)
}

pub fn resolve_cache_path(cli_path: Option<PathBuf>) -> PathBuf {
    // Priority: CLI flag > env var > default
    if let Some(path) = cli_path {
        return path;
    }

    if let Ok(env_path) = std::env::var("GRANOLA_CACHE_PATH") {
        return PathBuf::from(env_path);
    }

    // Default path
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join("Library/Application Support/Granola/cache-v3.json")
}
