use std::path::Path;
use std::fs::{OpenOptions, File};
use std::io::{Seek, SeekFrom, Read, Write};
use bincode;
use anyhow::Result;
use crate::Block;

/// Small append-only length-prefixed file store.
/// Format: [u64 BE len][block bytes]...
pub struct BlockStore {
    file: File,
    path: String,
}

impl BlockStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path_ref = path.as_ref();
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path_ref)?;
        Ok(Self { file, path: path_ref.to_string_lossy().to_string() })
    }

    pub fn append_block(&mut self, block: &Block) -> Result<u64> {
        let payload = bincode::serialize(block)?;
        let len = payload.len() as u64;
        // write len (big endian) then payload
        self.file.write_all(&len.to_be_bytes())?;
        self.file.write_all(&payload)?;
        self.file.flush()?;
        // return new file size as offset (approx). we can return current file position:
        let pos = self.file.seek(SeekFrom::End(0))?;
        Ok(pos)
    }

    pub fn iter_blocks(&mut self) -> Result<Vec<Block>> {
        // reopen for read-only to reset cursor
        let mut f = OpenOptions::new().read(true).open(&self.path)?;
        let mut blocks = Vec::new();
        loop {
            let mut len_buf = [0u8; 8];
            if f.read_exact(&mut len_buf).is_err() {
                break;
            }
            let len = u64::from_be_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];
            f.read_exact(&mut buf)?;
            let blk: Block = bincode::deserialize(&buf)?;
            blocks.push(blk);
        }
        Ok(blocks)
    }
}
