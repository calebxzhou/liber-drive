use std::path::PathBuf;

use walkdir::WalkDir;

use crate::media_item::MediaItem;

pub fn scan_medias(path: PathBuf) -> Vec<MediaItem> {
    let mut id:u32 = 0;
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_ascii_lowercase();
                let name = entry.file_name();
                
                
            }
            
           
        }
    }
    files
}