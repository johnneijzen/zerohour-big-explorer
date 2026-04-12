use serde::Serialize;

/// Core data model types for BIG archives
#[derive(Debug, Clone, Serialize)]
pub struct Archive {
    pub path: String,
    pub size: u64,
    pub format_version: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Index {
    pub entries_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Entry {
    pub name: String,
    pub offset: u64,
    pub length: u64,
    pub compressed: bool,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RepackJob {
    pub source_dir: String,
    pub compression: Option<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    /// Structured validation issues with optional machine-readable code and severity
    pub issues: Vec<ValidationIssue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationIssue {
    pub code: Option<String>,
    pub message: String,
    pub severity: String,
}
