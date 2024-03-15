use crate::image_exif;
use crate::util::{date_str_to_timestamp, filename_to_timestamp, ResultAnyErr};
use log::{debug, info};
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
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
    //所有相册
    fn scan_all_galleries(drive_dir: &PathBuf) -> ResultAnyErr<HashMap<String, Gallery>> {
        let mut all_galleries = HashMap::new();
        //扫描每个相册
        for entry in
            fs::read_dir(drive_dir)?.filter(|e| e.as_ref().unwrap().file_type().unwrap().is_dir())
        {
            let entry = entry.expect("扫描目录错误！");
            let gallery = Self::scan_single_gallery(&entry).unwrap();
            if gallery.albums.len() == 0 {
                debug!("跳过空gallery: {}", gallery);
                continue;
            }
            info!("{}", gallery);
            all_galleries.insert(gallery.name.clone(), gallery.clone());
        }
        Ok(all_galleries)
    }
    //扫描单个相册
    fn scan_single_gallery(entry: &DirEntry) -> ResultAnyErr<Gallery> {
        let gallery_name = entry.file_name().to_string_lossy().into_owned();
        let mut gallery_size = 0;
        let mut gallery_albums = HashMap::new();
        //扫描下属影集
        for album_entry in fs::read_dir(entry.path())? {
            let album_entry = album_entry?;
            //album必须是目录
            if album_entry.file_type()?.is_file() {
                continue;
            }
            let album = Self::scan_single_album(&album_entry)?;
            if album.medias.len() == 0 {
                debug!("跳过空album: {}", album);
                continue;
            }
            gallery_albums.insert(album.name.clone(), album.clone());
            gallery_size += album.size;
        }
        let gallery = Gallery::new(gallery_name, gallery_size, gallery_albums);
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

            album_medias.insert(
                name.clone(),
                MediaItem::new(path, name.clone(), time, size, exif, duration),
            );
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
