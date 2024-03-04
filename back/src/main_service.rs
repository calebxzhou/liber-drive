use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;  
use log::info;
use serde::Serialize;
use crate::gallery::Gallery;

use crate::media_item::{IdName, MediaItem};
use crate::media_scanner::scan_all_galleries;
 
pub struct MainService{
    pub galleries: Arc<Vec<Gallery>>,
    pub medias: Arc<HashMap<u32,MediaItem>>
}
impl MainService{
    pub fn new(drive_dir: &PathBuf) -> Self {
 
        let galleries = scan_all_galleries(&drive_dir);
        info!("{}个相册",galleries.len());
        let medias: HashMap<u32, MediaItem> = galleries
        .iter()
            .flat_map(|gallery| gallery.medias.clone())
            .map(|media| (media.id, media))
            .collect();

        let galleries = Arc::new(galleries);
        let medias = Arc::new(medias);

    
        Self { galleries, medias }

    }
}