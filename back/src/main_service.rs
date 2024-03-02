use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf; 
use axum::http::{header, Response, StatusCode};
use axum::response::IntoResponse;
use axum::{handler::Handler, routing::get, Json};
use log::info;
use serde::Serialize;
use crate::gallery::Gallery;

use crate::media_item::{IdName, MediaItem};
use crate::media_scanner::scan_all_galleries;
 
#[derive(Serialize,Clone)]
pub struct MainService{
    pub galleries: HashMap<u32,Gallery>,
    pub medias: HashMap<u32,MediaItem>
}
impl MainService{
      

    
    pub fn new(drive_dir: &PathBuf) -> Self {
 
        let galleries = scan_all_galleries(&drive_dir);
        info!("{}个相册",galleries.len());
        let medias: HashMap<u32, MediaItem> = galleries.values()
        .flat_map(|gallery| gallery.medias.clone().into_iter())
        .collect();
    
        Self { galleries, medias }

    }
}