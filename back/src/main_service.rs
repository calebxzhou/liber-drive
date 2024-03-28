use crate::image_exif;
use crate::util::{date_str_to_timestamp, filename_to_timestamp, ResultAnyErr};
use log::{debug, info};
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::io::{stdout, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::media_item::{self, Album, AlbumInfo, Gallery, GalleryInfo, MediaItem};

pub struct MainService {
    pub galleries: HashMap<String, Gallery>,
    pub galleries_info: HashMap<String, GalleryInfo>,
}
unsafe impl Send for MainService {}
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
        let mut all_galleries = HashMap::new();

        // Get all gallery entries
        let gallery_entries = fs::read_dir(drive_dir)
            .unwrap()
            .filter(|e| e.as_ref().unwrap().file_type().unwrap().is_dir())
            .map(|e| e.unwrap())
            .collect::<Vec<DirEntry>>();
        for entry in gallery_entries {
            let gallery = Self::scan_single_gallery(entry).unwrap();
            if gallery.albums.len() == 0 {
                debug!("跳过空gallery: {}", gallery);
                continue;
            }
            all_galleries.insert(gallery.name.clone(), gallery);
        }
        Ok(all_galleries)
    }
    //扫描单个相册
    fn scan_single_gallery(entry: DirEntry) -> ResultAnyErr<Gallery> {
        let gallery_name = entry.file_name().to_string_lossy().into_owned();
        let mut gallery_albums = HashMap::new();
        //扫描下属影集
        // Get all album entries
        let album_entries = fs::read_dir(entry.path())
            .unwrap()
            .map(|e| e.unwrap())
            .collect::<Vec<DirEntry>>();
        for album_entry in album_entries {
            // album必须是目录
            if album_entry.file_type().unwrap().is_file() {
                continue;
            }

            let album = Self::scan_single_album(album_entry).unwrap();
            if album.medias.len() == 0 {
                debug!("跳过空album: {}", album);
                continue;
            }
            gallery_albums.insert(album.name.clone(), album.clone());
        }
        let gallery = Gallery::new(gallery_name, gallery_albums);
        Ok(gallery)
    }
    //单个影集
    fn scan_single_album(album_entry: DirEntry) -> ResultAnyErr<Album> {
        let album_path = album_entry.path();
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
            print!("\r读取中 {}", path.display());
            stdout().flush().unwrap();
            let size = media_item::get_file_size(&path)?;

            let time = media_item::get_file_created_time(&path)?;

            let media = MediaItem::new(path, name.clone(), time, size);
            album_medias.insert(name.clone(), media);
        }
        Ok(Album::new(
            album_entry.file_name().to_string_lossy().into_owned(),
            album_medias,
        ))
    }
    //读取拍摄参数等更多信息
    pub fn read_more_media_infos(&mut self) {
        info!("\n读取exif");
        for gallery in self.galleries.iter_mut() {
            for albums in gallery.1.albums.iter_mut() {
                for media in albums.1.medias.iter_mut() {
                    let media_item = media.1;
                    let path = &media_item.path;
                    print!("\rexif {}", path.display());
                    stdout().flush().unwrap();
                    let exif = image_exif::ImageExif::from_media_path(path);
                    let mut time = media_item.time;
                    if let Ok(exif) = &exif {
                        //exif里面成功取时间了 就用exif的
                        if let Ok(_time) = date_str_to_timestamp(&exif.shot_time) {
                            time = _time;
                        } else if let Ok(_time) = filename_to_timestamp(&media_item.name) {
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
                    let duration = if media_item::is_video(&path) {
                        match media_item::get_video_duration(path.to_str().unwrap()) {
                            Ok(o) => Some(o),
                            Err(e) => {
                                info!("读取视频时长错误 {}", e);
                                None
                            }
                        }
                    } else {
                        None
                    };
                    media_item.exif = exif;
                    media_item.duration = duration;
                    media_item.time = time;
                }
            }
        }
        info!("\nexif读取完成");
    }
}
