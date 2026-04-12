use crate::preview::PreviewHandler;
use crate::models::Entry;

pub struct ImageHandler;

impl PreviewHandler for ImageHandler {
    fn can_preview(&self, entry: &Entry) -> bool {
        entry.r#type.as_deref() == Some("image")
    }

    fn preview(&self, entry: &Entry) -> anyhow::Result<String> {
        Ok(format!("<img alt=\"{}\" src=\"data:image/...\"/>", entry.name))
    }
}
