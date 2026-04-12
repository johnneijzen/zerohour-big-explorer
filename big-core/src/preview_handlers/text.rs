use crate::preview::PreviewHandler;
use crate::models::Entry;

pub struct TextHandler;

impl PreviewHandler for TextHandler {
    fn can_preview(&self, entry: &Entry) -> bool {
        entry.r#type.as_deref() == Some("text")
    }

    fn preview(&self, entry: &Entry) -> anyhow::Result<String> {
        Ok(format!("<pre>Preview of {}</pre>", entry.name))
    }
}
