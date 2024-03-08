use crate::util::{date_str_to_timestamp, filename_to_timestamp, AnyError};
use log::info;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::thread;
use walkdir::WalkDir;

use crate::media_item::{self, is_image, Gallery, GalleryInfo, ImageExif, MediaItem};

pub struct MainService {
    pub galleries: HashMap<u32, Gallery>,
    pub medias: HashMap<u32, MediaItem>,
    pub galleries_info: Vec<GalleryInfo>,
}
impl MainService {
    pub fn new(drive_dir: &PathBuf) -> Self {
        let galleries = Self::scan_all_galleries(drive_dir).expect("扫描相册错误！");
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
                tbnl_media_id: g.1.get_cover_media_id(),
            })
            //去除没有照片的目录
            .filter(|g| g.media_amount != 0)
            .collect();
        info!("共{}个文件 载入完成", medias.len());
        
        Self {
            galleries,
            medias,
            galleries_info,
        }
    }
    fn scan_all_galleries(drive_dir: &PathBuf) -> Result<HashMap<u32, Gallery>, AnyError> {
        let mut all_galleries = HashMap::new();

        let media_id: &mut u32 = &mut 0;
        let mut gallery_id = 0;
        // Using the ? operator to propagate errors
        let entries = fs::read_dir(drive_dir)?
            .filter(|e| e.as_ref().unwrap().file_type().unwrap().is_dir())
            .collect::<Result<Vec<DirEntry>, std::io::Error>>()?;
        //扫描每个相册
        for entry in entries {
            let gallery = Self::scan_single_gallery(&entry, media_id, gallery_id)?;

            info!("{}", gallery);
            all_galleries.insert(gallery_id, gallery);
            gallery_id += 1;
        }
        Ok(all_galleries)
    }
    //扫描单个相册
    fn scan_single_gallery(
        entry: &DirEntry,
        media_id: &mut u32,
        gallery_id: u32,
    ) -> Result<Gallery, AnyError> {
        let gallery_name = entry.file_name().to_string_lossy().into_owned();
        let mut gallery_size = 0;
        let mut gallery_medias = Vec::new();
        for entry in WalkDir::new(entry.path()) {
            let media_entry = entry?;
            let path = media_entry.path().to_path_buf();
            //跳过目录
            if media_entry.file_type().is_dir() {
                continue;
            }
            //既不是图片也不是视频，跳过
            if !(media_item::is_video(&path) 
            || media_item::is_image(&path) || media_item::is_heif_image(&path)
        ) {
                continue;
            }

            let name = media_entry.file_name().to_string_lossy().into_owned();
            let meta = fs::metadata(&path)?;
            let size = meta.len();
            //时间
            let mut time = media_item::get_file_created_time(&path)?;
            let exif = ImageExif::from_media_path(&path);

            if let Ok(exif) = &exif {
                //exif里面成功取时间了 就用exif的
                if let Ok(_time) = date_str_to_timestamp(&exif.shot_time) {
                    time = _time;
                } else if let Ok(_time) = filename_to_timestamp(&name) {
                    //exif里没有 就去文件名里取
                    time = _time;
                }
                //再取不到 就用系统的修改时间
            }
            let exif = if let Ok(exif) = exif {
                Some(exif)
            } else {
                None
            };
            gallery_medias.push(MediaItem::new(*media_id, path, name, time, size, exif));
            //累计相册尺寸
            gallery_size += size;
            *media_id += 1;
        }
        let gallery = Gallery::new(gallery_id, gallery_name, gallery_size, gallery_medias);
        Ok(gallery)
    }
}
