use std::collections::HashMap;
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU32, Ordering::SeqCst},
    time::UNIX_EPOCH,
};

use log::info;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use walkdir::WalkDir;

use crate::gallery::{self, Gallery};
use crate::{
    media_item::{IdName, MediaItem},
    media_processing::get_image_exif,
};
pub fn scan_all_galleries(drive_dir: &PathBuf) -> Vec<Gallery> {
    let mut all_galleries = Vec::new();

    let mut media_id = 0;
    let mut gallery_id = 0;
    for entry in fs::read_dir(drive_dir).unwrap() {
        let gallery = entry.unwrap();
        //相册是目录 不是文件
        if gallery.file_type().unwrap().is_file() {
            continue;
        }
        let gallery_name = gallery.file_name().into_string().unwrap();
        let mut gallery_size = 0;
        let mut gallery_medias = Vec::new();
        for entry in WalkDir::new(gallery.path()) {
            let media = entry.unwrap();
            //跳过目录
            if media.file_type().is_dir() {
                continue;
            }
            let path = media.path().to_path_buf();
            if let Some(ext) = path.extension() {
                if !(ext == "jpg" || ext == "mp4") {
                    continue;
                }
                let name = media.file_name().to_string_lossy().into_owned();
                let meta = fs::metadata(&path).unwrap();
                let size = meta.len();
                let   time = meta
                    .created()
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let exif = if ext == "jpg" {
                    if let Ok(exif) = get_image_exif(&path) {
                        exif
                    } else {
                        None
                    }
                } else {
                    None
                };
                let info = MediaItem::new(media_id, path, name, time, size, exif);
                gallery_medias.push( info);
                gallery_size += size;
                media_id += 1;
            }
        }
        let gallery = Gallery::new(gallery_id, gallery_name, gallery_size, gallery_medias);
        info!("{}", gallery);
        all_galleries.push( gallery);
        gallery_id += 1;
    }
    all_galleries
} 