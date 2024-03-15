use crate::image_exif;
use crate::util::{date_str_to_timestamp, filename_to_timestamp, ResultAnyErr};
use log::{debug, info};
use rayon::iter::*;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

use crate::media_item::{self, Album, AlbumInfo, Gallery, GalleryInfo, MediaItem};

pub struct MainService {
    pub galleries: HashMap<String, Gallery>,
    pub galleries_info: HashMap<String, GalleryInfo>,
}
impl MainService {
    pub fn new(drive_dir: &PathBuf) -> Self {
        let galleries = Self::scan_all_galleries(drive_dir).expect("扫描相册错误！");
        info!("共{}个相册", galleries.len());
        let galleries_info: HashMap<String, GalleryInfo> = galleries
            .iter()
            .map(|(key, gallery)| {
                let album_infos: Vec<AlbumInfo> = gallery
                    .albums
                    .iter()
                    .map(|(_, album)| AlbumInfo::from_album(album))
                    .collect();
                (
                    key.clone(),
                    GalleryInfo {
                        name: gallery.name.clone(),
                        size: gallery.size,
                        albums: album_infos,
                    },
                )
            })
            .collect();
        Self {
            galleries,
            galleries_info,
        }
    }
    fn scan_all_galleries(drive_dir: &PathBuf) -> ResultAnyErr<HashMap<String, Gallery>> {
        let all_galleries = Arc::new(Mutex::new(HashMap::new()));

        // Get all gallery entries
        let gallery_entries: Vec<_> = fs::read_dir(drive_dir)?
            .filter(|e| e.as_ref().unwrap().file_type().unwrap().is_dir())
            .collect::<Result<_, _>>()?;

        // Use par_iter (parallel iterator) provided by rayon
        gallery_entries.par_iter().for_each(|entry| {
            let gallery = Self::scan_single_gallery(entry).unwrap();
            if gallery.albums.len() == 0 {
                debug!("跳过空gallery: {}", gallery);
                return;
            }

            // Use a lock to safely update the HashMap
            let mut galleries_lock = all_galleries.lock().unwrap();
            galleries_lock.insert(gallery.name.clone(), gallery.clone());
        });
        let x = all_galleries.lock().unwrap().clone();
        Ok(x)
    }
    //扫描单个相册
    fn scan_single_gallery(entry: &DirEntry) -> ResultAnyErr<Gallery> {
        let gallery_name = entry.file_name().to_string_lossy().into_owned();
        let gallery_size = Arc::new(Mutex::new(0));
        let gallery_albums = Arc::new(Mutex::new(HashMap::new()));
        //扫描下属影集
        // Get all album entries
        let album_entries: Vec<_> = fs::read_dir(entry.path())?.collect::<Result<_, _>>()?;

        // Use par_iter (parallel iterator) provided by rayon
        album_entries.par_iter().for_each(|album_entry| {
            // album必须是目录
            if album_entry.file_type().unwrap().is_file() {
                return;
            }

            let album = Self::scan_single_album(album_entry).unwrap();
            if album.medias.len() == 0 {
                debug!("跳过空album: {}", album);
                return;
            }

            // Use a lock to safely update the HashMap and size counter
            let mut albums_lock = gallery_albums.lock().unwrap();
            albums_lock.insert(album.name.clone(), album.clone());

            let mut size_lock = gallery_size.lock().unwrap();
            *size_lock += album.size;
        });
        let gallery = Gallery::new(
            gallery_name,
            *gallery_size.lock().unwrap(),
            gallery_albums.lock().unwrap().clone(),
        );
        Ok(gallery)
    }
    //单个影集
    fn scan_single_album(album_entry: &DirEntry) -> ResultAnyErr<Album> {
        let album_path = album_entry.path();
        let mut album_size = 0;
        let mut album_medias = HashMap::new();
        for media_entry in WalkDir::new(album_path) {
            let media_entry = media_entry?;
            let path = media_entry.path().to_path_buf();
            //媒体文件不是目录
            if media_entry.file_type().is_dir() {
                continue;
            }

            //既不是图片也不是视频，跳过
            if !(media_item::is_video(&path) || media_item::is_image(&path)) {
                continue;
            }

            let name = media_entry.file_name().to_string_lossy().into_owned();
            //跳过点开头的文件
            if name.starts_with(".") {
                continue;
            }
            let size = media_item::get_file_size(&path)?;

            let mut time = media_item::get_file_created_time(&path)?;
            let exif = image_exif::ImageExif::from_media_path(&path);

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

            //视频时长
            let duration = media_item::is_video(&path)
                .then(|| media_item::get_video_duration(path.to_str().unwrap()).unwrap());
            let media = MediaItem::new(path, name.clone(), time, size, exif, duration);
            info!("{}", media.path.display());
            album_medias.insert(name.clone(), media);
            //累计相册尺寸
            album_size += size;
        }
        Ok(Album::new(
            album_entry.file_name().to_string_lossy().into_owned(),
            album_size,
            album_medias,
        ))
    }
}
