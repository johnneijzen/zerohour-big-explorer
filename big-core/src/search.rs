use crate::models::Entry;

/// Filter entries by an optional query string (case-insensitive substring match).
pub fn filter_entries(entries: &[Entry], query: Option<&str>) -> Vec<Entry> {
    if let Some(q) = query {
        let ql = q.to_lowercase();
        entries.iter().filter(|e| e.name.to_lowercase().contains(&ql)).cloned().collect()
    } else {
        entries.to_vec()
    }
}
