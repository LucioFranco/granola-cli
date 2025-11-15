use std::fmt;

#[derive(Debug)]
pub enum GranolaError {
    CacheNotFound(String),
    InvalidCacheFormat(String),
    MeetingNotFound(String),
    TranscriptNotFound(String),
    #[allow(dead_code)]
    InvalidArguments(String),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for GranolaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GranolaError::CacheNotFound(path) => {
                write!(f, "Error: Cache file not found\nPath: {}\nSuggestion: Ensure Granola is installed and has been run at least once", path)
            }
            GranolaError::InvalidCacheFormat(msg) => {
                write!(f, "Error: Invalid cache format\nDetails: {}\nSuggestion: Cache might be corrupted or version mismatch", msg)
            }
            GranolaError::MeetingNotFound(id) => {
                write!(f, "Error: Meeting not found\nID: {}\nSuggestion: Use 'granola search' to find valid meeting IDs", id)
            }
            GranolaError::TranscriptNotFound(id) => {
                write!(f, "Error: Transcript not found for meeting\nID: {}\nSuggestion: This meeting may not have been transcribed", id)
            }
            GranolaError::InvalidArguments(msg) => {
                write!(f, "Error: Invalid arguments\nDetails: {}", msg)
            }
            GranolaError::IoError(e) => write!(f, "Error: IO error\nDetails: {}", e),
            GranolaError::JsonError(e) => write!(f, "Error: JSON parsing error\nDetails: {}", e),
        }
    }
}

impl std::error::Error for GranolaError {}

impl From<std::io::Error> for GranolaError {
    fn from(err: std::io::Error) -> Self {
        GranolaError::IoError(err)
    }
}

impl From<serde_json::Error> for GranolaError {
    fn from(err: serde_json::Error) -> Self {
        GranolaError::JsonError(err)
    }
}

impl GranolaError {
    pub fn exit_code(&self) -> i32 {
        match self {
            GranolaError::CacheNotFound(_) => 2,
            GranolaError::InvalidCacheFormat(_) => 3,
            GranolaError::MeetingNotFound(_) => 4,
            GranolaError::TranscriptNotFound(_) => 4,
            GranolaError::InvalidArguments(_) => 5,
            GranolaError::IoError(_) => 1,
            GranolaError::JsonError(_) => 3,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "error": {
                "code": self.exit_code(),
                "type": self.error_type(),
                "message": self.error_message(),
                "suggestion": self.suggestion(),
            }
        })
    }

    fn error_type(&self) -> &str {
        match self {
            GranolaError::CacheNotFound(_) => "cache_not_found",
            GranolaError::InvalidCacheFormat(_) => "invalid_cache_format",
            GranolaError::MeetingNotFound(_) => "meeting_not_found",
            GranolaError::TranscriptNotFound(_) => "transcript_not_found",
            GranolaError::InvalidArguments(_) => "invalid_arguments",
            GranolaError::IoError(_) => "io_error",
            GranolaError::JsonError(_) => "json_error",
        }
    }

    fn error_message(&self) -> String {
        match self {
            GranolaError::CacheNotFound(path) => format!("Cache file not found: {}", path),
            GranolaError::InvalidCacheFormat(msg) => format!("Invalid cache format: {}", msg),
            GranolaError::MeetingNotFound(id) => format!("Meeting not found: {}", id),
            GranolaError::TranscriptNotFound(id) => {
                format!("Transcript not found for meeting: {}", id)
            }
            GranolaError::InvalidArguments(msg) => format!("Invalid arguments: {}", msg),
            GranolaError::IoError(e) => format!("IO error: {}", e),
            GranolaError::JsonError(e) => format!("JSON parsing error: {}", e),
        }
    }

    fn suggestion(&self) -> Option<String> {
        match self {
            GranolaError::CacheNotFound(_) => {
                Some("Ensure Granola is installed and has been run at least once".to_string())
            }
            GranolaError::InvalidCacheFormat(_) => {
                Some("Cache might be corrupted or version mismatch".to_string())
            }
            GranolaError::MeetingNotFound(_) => {
                Some("Use 'granola search' to find valid meeting IDs".to_string())
            }
            GranolaError::TranscriptNotFound(_) => {
                Some("This meeting may not have been transcribed".to_string())
            }
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, GranolaError>;
