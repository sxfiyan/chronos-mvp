use anyhow::{Context, Result};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

pub struct DiskImage {
    pub data: Mmap,
    pub path: String,
}

impl DiskImage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)
            .context("Failed to open disk image file")?;
        
        let data = unsafe { Mmap::map(&file) }
            .context("Failed to memory map disk image")?;
        
        Ok(DiskImage {
            data,
            path: path.to_string_lossy().to_string(),
        })
    }
    
    pub fn get_slice(&self, offset: usize, length: usize) -> Result<&[u8]> {
        if offset + length > self.data.len() {
            anyhow::bail!("Attempted to read beyond disk image bounds");
        }
        Ok(&self.data[offset..offset + length])
    }
    
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    pub fn is_e01_format(&self) -> bool {
        self.path.to_lowercase().ends_with(".e01")
    }
    
    pub fn is_dd_format(&self) -> bool {
        self.path.to_lowercase().ends_with(".dd")
    }
} 