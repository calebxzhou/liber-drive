use crate::gallery::{Gallery, GalleryInfo};
use log::info;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::media_item::MediaItem;
use crate::media_scanner::scan_all_galleries;

pub struct MainService {
    pub galleries: Arc<HashMap<u32, Box<Gallery>>>,
    pub medias: Arc<HashMap<u32, MediaItem>>,
    pub galleries_info: Arc<Vec<GalleryInfo>>,
}
impl MainService {
    pub fn new(drive_dir: &PathBuf) -> Self {
        let galleries = scan_all_galleries(drive_dir).expect("扫描相册错误！");
        info!("共{}个相册", galleries.len());
        let medias: HashMap<u32, MediaItem> = galleries
            .iter()
            .flat_map(|gallery| gallery.1.medias.clone())
            .map(|media| (media.id, media))
            .collect();
        let galleries_info = galleries
            .iter()
            .map(|g| GalleryInfo {
                id: g.1.id,
                name: g.1.name.clone(),
                size: g.1.size,
                media_amount: g.1.medias.len() as u32,
                tbnl_media_ids: GalleryInfo::get_tbnl_media_ids(&g.1.medias)
            })
            //去除没有照片的目录
            .filter(|g|g.media_amount!=0)
            .collect();
        let galleries = Arc::new(galleries);
        let medias = Arc::new(medias);
        let galleries_info = Arc::new(galleries_info);
        info!("共{}个文件 载入完成", medias.len());

        Self {
            galleries,
            medias,
            galleries_info,
        }
    }
}
