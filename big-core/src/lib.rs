//! big-core: archive parsing and manipulation core library

pub mod archive;
pub mod entry;
pub mod index;
pub mod iterator;
pub mod models;
pub mod parser;
pub mod paths;
pub mod search;

pub mod extract;
pub mod pack;
pub mod preview;
pub mod preview_handlers;
pub mod progress;
pub mod validate;

pub use archive::open as open_archive;
pub use extract::{extract_entry_to_path, stream_entry_to_writer};
pub use iterator::EntryIterator;
pub use parser::parse_archive;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_version() {
        assert!(!version().is_empty());
    }
}
