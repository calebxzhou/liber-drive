use std::cell::RefCell;
use crate::album::{Album, AlbumInfo};
use crate::util::ResultAnyErr;
use log::{debug, error, info};
use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use rayon::prelude::*;
use tokio::sync::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use tokio::time::{self, Duration};
use crate::media_item::{self, is_jpg_image, is_video, MediaItem};
pub type SharedService = Arc<RwLock<MainService>>;
#[derive(Clone,Serialize)]
pub struct MainService {
    pub albums: HashMap<String, Album>,
    // id to media
    pub medias: HashMap<String, MediaItem>
}
impl MainService {
    pub fn new(drive_dirs: &Vec<PathBuf>) -> SharedService {
        let mut medias: HashMap<String, MediaItem> = HashMap::new();
        let albums = Self::scan_all_albums(drive_dirs, &mut medias).expect("扫描相册错误！");
        let slf = Self { albums , medias};
        slf.build_tbnls();
        let service = Arc::new(RwLock::new(slf));

        // Clone drive_dirs to move it into the async block
        let drive_dirs_clone = drive_dirs.clone();
        let service_clone = Arc::clone(&service);
        // 10min更新一次
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(600));
            interval.tick().await; // Initial delay
            loop {
                interval.tick().await;
                let drive_dirs_clone = drive_dirs_clone.clone();
                let service_clone = Arc::clone(&service_clone);
                tokio::spawn(async move {
                    let mut service = service_clone.write().await;
                    service.update_albums(&drive_dirs_clone).await;
                });
            }
        });

        service
    }
    async fn update_albums(&mut self, drive_dirs: &Vec<PathBuf>) {
        let mut medias: HashMap<String, MediaItem> = HashMap::new();
        match Self::scan_all_albums(drive_dirs, &mut medias) {
            Ok(new_albums) => {
                self.albums = new_albums;
                self.medias = medias;
                self.build_tbnls();
                info!("Albums updated successfully.");
            }
            Err(e) => {
                error!("Failed to update albums: {:?}", e);
            }
        }
    }
    fn build_tbnls(&self) {
        // 多线程建立缩略图
        let albums = &self.albums;
        rayon::join(
            || {
                albums.par_iter().for_each(|(album_name, album)| {

                    album.build_tbnls();

                });
            },
            || {
                // This empty closure ensures that the join function waits for the first closure to complete
            },
        );
        info!("缩略图OK");
    }
    //单个影集
    fn scan_all_albums(drive_dirs: &Vec<PathBuf>,all_medias: &mut HashMap<String, MediaItem>) -> ResultAnyErr<HashMap<String, Album>> {
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

                let album = Self::scan_single_album(entry.path(),all_medias).unwrap();
                if album.medias.len() == 0 && album.sub_albums.len() == 0 {
                    continue;
                }
                all_albums.insert(album.name.clone(), album);
            }
        }
        Ok(all_albums)
    }

    //单个影集
    fn scan_single_album(album_path: PathBuf, all_medias: &mut HashMap<String, MediaItem>) -> ResultAnyErr<Album> {
        let mut album = Album::new(
            album_path.file_name().unwrap().to_string_lossy().into_owned());

        // Scan for album's password first
        for media_entry in fs::read_dir(&album_path)? {
            let media_entry = media_entry?;
            let path = media_entry.path().to_path_buf();
            let name = media_entry.file_name().to_string_lossy().into_owned();

            if name == "pwd.txt" {
                let mut contents = String::new();
                File::open(&path)?.read_to_string(&mut contents)?;
                album.pwd = Some(contents);
                break;
            }
        }

        // Traverse all files and sub-albums
        for media_entry in fs::read_dir(&album_path)? {
            let media_entry = media_entry?;
            let path = media_entry.path().to_path_buf();
            if media_entry.file_type()?.is_dir() && path != album_path {
                // Recursively scan sub-albums
                let sub_album = Self::scan_single_album(path, all_medias)?;
                if sub_album.medias.len() == 0 && sub_album.sub_albums.len() == 0 {
                    //debug!("Skipping empty album: {:?}/{}", album_path, sub_album.name);
                    continue;
                }
                album.sub_albums.insert(sub_album.name.clone(), sub_album);
                continue;
            }

            let name = media_entry.file_name().to_string_lossy().into_owned();
            // Skip dot files
            if name.starts_with(".") {
                continue;
            }
            // Skip non-image and non-video files
            if !(media_item::is_video(&path) || media_item::is_jpg_image(&path)) {
                continue;
            }
            print!("\rReading {}", path.display());
            stdout().flush().unwrap();
            let size = media_item::get_file_size(&path)?;
            let time = media_item::get_file_created_time(&path)?;

            let mut media = MediaItem::new(path, name.clone(), time, size);
            media.pwd = album.pwd.clone();
            if is_jpg_image(&media.path) {
                if let Err(e) = media.create_exif_cache() {
                    error!("Error creating exif cache, {:?}", e);
                }
                if let Err(e) = media.read_exif_cache() {
                    error!("Error reading exif cache, {:?}", e);
                }
            } else if is_video(&media.path) {
                if let Err(e) = media.create_video_duration_cache() {
                    error!("Error creating video cache, {:?}", e);
                }
                if let Err(e) = media.read_video_duration_cache() {
                    error!("Error reading video cache, {:?}", e);
                }
            }
            media.update_media_time();
            all_medias.insert(media.get_media_id(), media.clone());
            album.medias.insert(name.clone(), media);
        }
        Ok(album)
    }

}
