use std::path::{Path, PathBuf};
use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileContext {
    pub path: PathBuf,
    pub content: String,
    pub line_count: usize,
}

#[allow(dead_code)]
impl FileContext {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let line_count = content.lines().count();
        
        Ok(Self {
            path: path.to_path_buf(),
            content,
            line_count,
        })
    }
    
    pub fn format_for_context(&self) -> String {
        format!(
            "File: {}\nLines: {}\n\n{}",
            self.path.display(),
            self.line_count,
            self.content
        )
    }
}
