use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use crate::errors::Errors;

#[derive(Debug)]
pub struct LogStore {
    pub file_path: PathBuf,
    pub mmap: Mmap,
    pub line_offsets: Vec<(usize, usize)>,
}

impl LogStore {
    pub fn new(path: &str) -> Result<Arc<Self>, Errors> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let file_path = PathBuf::from_str(path).map_err(|e| Errors::PathBufError(e.to_string()))?;

        let line_offsets = mmap
            .as_parallel_slice()
            .par_split(|&byte| byte == b'\n')
            .map(|slice| {
                let start = unsafe { slice.as_ptr().offset_from(mmap.as_ptr()) as usize };
                (start, start + slice.len())
            })
            .collect();

        Ok(Arc::new(Self {
            file_path,
            mmap,
            line_offsets,
        }))
    }

    pub fn get_line(&self, index: usize) -> Option<&str> {
        let (start, end) = self.line_offsets.get(index)?;
        std::str::from_utf8(&self.mmap[*start..*end]).ok()
    }

    pub fn len(&self) -> usize {
        self.line_offsets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
