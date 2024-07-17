use crate::album::Album;
use crate::image_exif;
use crate::util::{date_str_to_timestamp, filename_to_timestamp, ResultAnyErr};
use log::{debug, info};
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::io::{stdout, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::media_item::{self, MediaItem};
pub type AlbumList = HashMap<String, Album>;
#[derive(Clone)]
pub struct MainService {
    pub albums: AlbumList,
}
unsafe impl Send for MainService {}
impl MainService {
    pub fn new(drive_dirs: &Vec<PathBuf>) -> Self {
        let albums = Self::scan_all_albums(drive_dirs).expect("扫描相册错误！");
        info!("共{}个相册", albums.len());
        Self { albums }
    }
    fn scan_all_albums(drive_dirs: &Vec<PathBuf>) -> ResultAnyErr<AlbumList> {
        let mut all_albums = HashMap::new();
        for drive_dir in drive_dirs {
            info!("开始扫描{:?}", drive_dir);
            // Get all gallery entries
            let dir_entries = fs::read_dir(drive_dir)
                .unwrap()
                .filter(|e| e.as_ref().unwrap().file_type().unwrap().is_dir())
                .map(|e| e.unwrap())
                .collect::<Vec<DirEntry>>();
            for entry in dir_entries {
                let album = Self::scan_single_album(entry).unwrap();
                if album.medias.len() == 0 {
                    info!("跳过空album: {}", album);
                    continue;
                }
                all_albums.insert(album.name.clone(), album);
            }
        }
        Ok(all_albums)
    }

    //单个影集
    fn scan_single_album(album_entry: DirEntry) -> ResultAnyErr<Album> {
        let album_path = album_entry.path();
        let mut album_medias = HashMap::new();
        //遍历所有下属文件
        for media_entry in WalkDir::new(album_path) {
            let media_entry = media_entry?;
            let path = media_entry.path().to_path_buf();
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
}
