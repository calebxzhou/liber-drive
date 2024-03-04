use std::{fmt, path::PathBuf};

use serde::Serialize;

use crate::media_processing::ImageExif;

#[derive(Serialize, Clone)]
pub struct MediaItem {
    pub id: u32,
    #[serde(skip_serializing)]
    pub path: PathBuf,
    name: String,
    pub time: u64,
    pub size: u64, 
    pub exif: Option<ImageExif>,
}
impl MediaItem {
    pub fn new(
        id: u32,
        path: PathBuf,
        name: String,
        time: u64,
        size: u64,
        exif: Option<ImageExif>,
    ) -> MediaItem { 
        Self {
            id,
            path,
            name,
            time,
            size,
            exif,
        }
    }
    pub fn is_video(&self) -> bool {
        if let Some(ext) = self.path.extension() {
            let ext = ext.to_ascii_lowercase();
            ext == "mp4" || ext == "mov"
        } else {
            false
        }
    }
    pub fn is_image(&self) -> bool {
        if let Some(ext) = self.path.extension() {
            let ext = ext.to_ascii_lowercase();
            ext == "png" || ext == "jpg"
        } else {
            false
        }
    }
}
impl fmt::Display for MediaItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.id, self.path.display().to_string())
    }
}

#[derive(Serialize, Clone)]
pub struct IdName {
    id: u32,
    name: String,
}
impl IdName {
    pub fn new(id: u32, path: String) -> IdName {
        Self { id, name: path }
    }
}
impl fmt::Display for IdName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.id, self.name)
    }
}
