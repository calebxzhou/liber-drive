use crate::album::Album;
use crate::util::ResultAnyErr;
use log::info;
use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::media_item::{self, is_jpg_image, is_video, MediaItem};
#[derive(Clone)]
pub struct MainService {
    pub albums: HashMap<String, Album>,
}
impl MainService {
    pub fn new(drive_dirs: &Vec<PathBuf>) -> Self {
        let albums = Self::scan_all_albums(drive_dirs).expect("扫描相册错误！");

        info!("共{}个相册", albums.len());
        Self { albums }
    }
   

    //单个影集
    fn scan_all_albums(drive_dirs: &Vec<PathBuf>) -> ResultAnyErr<HashMap<String, Album>> {
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
                
                let album = Self::scan_single_album(entry.path()).unwrap();
                if album.medias.len() == 0 && album.sub_albums.len() == 0 {
                    info!("跳过空album: {}", album.name);
                    continue;
                }
                all_albums.insert(album.name.clone(), album);
            }
        }
        Ok(all_albums)
    }

    //单个影集
    fn scan_single_album(album_path: PathBuf) -> ResultAnyErr<Album> {
        let mut album_medias = HashMap::new();
        let mut sub_albums = HashMap::new();
        let mut password = Option::None;
        //遍历所有下属文件
        for media_entry in WalkDir::new(&album_path) {
            let media_entry = media_entry?;
            let path = media_entry.path().to_path_buf();
            if media_entry.file_type().is_dir() && path != album_path {
                // Recursively scan sub-albums
                let sub_album = Self::scan_single_album(path)?;
                if sub_album.medias.len() == 0 && sub_album.sub_albums.len() == 0 {
                    info!("跳过空album: {:?}/{}",album_path, sub_album.name);
                    continue;
                }
                sub_albums.insert(sub_album.name.clone(), sub_album);
                continue;
            }
           

            let name = media_entry.file_name().to_string_lossy().into_owned();
            //密码
            if name == "pwd.txt"{
                let mut contents = String::new();
                File::open(&path)?.read_to_string(&mut contents)?;
                password =  Option::Some(contents)
            };
            //跳过点开头的文件
            if name.starts_with(".") {
                continue;
            }
            //既不是图片也不是视频，跳过
            if !(media_item::is_video(&path) || media_item::is_jpg_image(&path)) {
                continue;
            }
            print!("\r读取中 {}", path.display());
            stdout().flush().unwrap();
            let size = media_item::get_file_size(&path)?;
            let time = media_item::get_file_created_time(&path)?;

            let mut media = MediaItem::new(path, name.clone(), time, size);
            if is_jpg_image(&media.path) {
                if let Err(e) = media.create_exif_cache() {
                    info!("创建exif错误，{:?}", e);
                }
                if let Err(e) = media.read_exif_cache() {
                    info!("读取exif错误，{:?}", e);
                }
            } else if is_video(&media.path) {
                if let Err(e) = media.create_video_duration_cache() {
                    info!("创建video cache错误，{:?}", e);
                };
                if let Err(e) = media.read_video_duration_cache() {
                    info!("读取video cache错误，{:?}", e);
                };
            }
            media.update_media_time();
            album_medias.insert(name.clone(), media);
        }
        Ok(Album::new(
            album_path.file_name().unwrap().to_string_lossy().into_owned(),
            album_medias,
            sub_albums,
            password
        ))
    }
}
