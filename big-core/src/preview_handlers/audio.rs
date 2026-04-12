use crate::models::Entry;
use crate::preview::PreviewHandler;

pub struct AudioHandler;

impl PreviewHandler for AudioHandler {
    fn can_preview(&self, entry: &Entry) -> bool {
        entry.r#type.as_deref() == Some("audio")
    }

    fn preview(&self, entry: &Entry) -> anyhow::Result<String> {
        Ok(format!("<audio controls src=\"data:audio/...\">{}</audio>", entry.name))
    }
}
