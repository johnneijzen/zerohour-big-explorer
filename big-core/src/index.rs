use crate::models::Entry;
use std::io::{Read, Seek, SeekFrom};

pub struct IndexReader {}

impl IndexReader {
    pub fn new() -> Self {
        Self {}
    }

    /// Read entries from given offset and count. Expects cursor positioned anywhere; will seek to offset.
    pub fn read_entries_from<R: Read + Seek>(
        &self,
        r: &mut R,
        offset: u64,
        count: u32,
    ) -> anyhow::Result<Vec<Entry>> {
        r.seek(SeekFrom::Start(offset))?;
        let mut entries = Vec::with_capacity(count as usize);

        for _ in 0..count {
            // name length (u16 LE)
            let mut buf = [0u8; 2];
            r.read_exact(&mut buf)?;
            let name_len = u16::from_le_bytes(buf) as usize;

            let mut name_buf = vec![0u8; name_len];
            r.read_exact(&mut name_buf)?;
            let name = String::from_utf8_lossy(&name_buf).into_owned();

            // offset (u64 LE)
            let mut buf8 = [0u8; 8];
            r.read_exact(&mut buf8)?;
            let offset = u64::from_le_bytes(buf8);

            // length (u64 LE)
            r.read_exact(&mut buf8)?;
            let length = u64::from_le_bytes(buf8);

            // compressed flag (u8)
            let mut f = [0u8; 1];
            r.read_exact(&mut f)?;
            let compressed = f[0] != 0;

            // type length + type bytes (u8 length)
            let mut tlen = [0u8; 1];
            r.read_exact(&mut tlen)?;
            let tlen = tlen[0] as usize;
            let mut tbuf = vec![0u8; tlen];
            r.read_exact(&mut tbuf)?;
            let rtype =
                if tlen == 0 { None } else { Some(String::from_utf8_lossy(&tbuf).into_owned()) };

            entries.push(Entry { name, offset, length, compressed, r#type: rtype });
        }

        Ok(entries)
    }
}

impl Default for IndexReader {
    fn default() -> Self {
        Self::new()
    }
}
