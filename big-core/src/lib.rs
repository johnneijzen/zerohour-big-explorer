//! big-core: archive parsing and manipulation core library

pub mod models;
pub mod archive;
pub mod index;
pub mod entry;
pub mod parser;
pub mod iterator;

pub mod extract;

pub use archive::open as open_archive;
pub use parser::parse_archive;
pub use iterator::EntryIterator;
pub use extract::{extract_entry_to_path, stream_entry_to_writer};

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
