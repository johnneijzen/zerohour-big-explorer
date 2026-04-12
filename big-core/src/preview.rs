use crate::models::Entry;

/// Trait for preview handlers
pub trait PreviewHandler {
    fn can_preview(&self, entry: &Entry) -> bool;
    fn preview(&self, entry: &Entry) -> anyhow::Result<String>; // return HTML/text preview
}

/// Registry (very small) for handlers
pub struct PreviewRegistry {
    handlers: Vec<Box<dyn PreviewHandler + Send + Sync>>,
}

impl PreviewRegistry {
    pub fn new() -> Self { Self { handlers: Vec::new() } }
    pub fn register<H: PreviewHandler + Send + Sync + 'static>(&mut self, h: H) {
        self.handlers.push(Box::new(h));
    }

    pub fn preview(&self, entry: &Entry) -> Option<anyhow::Result<String>> {
        for h in &self.handlers {
            if h.can_preview(entry) {
                return Some(h.preview(entry));
            }
        }
        None
    }
}
