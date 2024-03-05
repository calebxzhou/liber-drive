 
use std::collections::HashMap;
use std::error::Error;
use std::fs::DirEntry;
use std::{
    fs,
    path::PathBuf, 
    time::UNIX_EPOCH,
};

use log::info;
use walkdir::WalkDir;

use crate::gallery::Gallery;
use crate::util::{date_str_to_timestamp, filename_to_timestamp};
use crate::{
    media_item::MediaItem,
    media_processing::get_image_exif,
};
pub fn scan_all_galleries(drive_dir: &PathBuf) -> Result<HashMap<u32,Box<Gallery>>, Box<dyn Error>> {
    let mut all_galleries = HashMap::new();

    let media_id: &mut u32 = &mut 0;
    let mut gallery_id=0;
    // Using the ? operator to propagate errors
    let entries = fs::read_dir(drive_dir)?
        .filter(|e| e.as_ref().unwrap().file_type().unwrap().is_dir())
        .collect::<Result<Vec<DirEntry>, std::io::Error>>()?;
    //扫描每个相册
    for entry in entries {
        let gallery = scan_single_gallery(&entry, media_id, gallery_id)?;
        
        info!("{}", gallery);
        all_galleries.insert(gallery_id,gallery);
        gallery_id += 1;
    }
    Ok(all_galleries)
}
fn scan_single_gallery(entry: &DirEntry, media_id: &mut u32, gallery_id: u32) -> Result<Box<Gallery>,Box<dyn Error>> {
  
    let gallery_name = entry.file_name().into_string().unwrap();
    let mut gallery_size = 0;
    let mut gallery_medias = Vec::new();
    for entry in WalkDir::new(entry.path()) {
        let media = entry.unwrap();
        //跳过目录
        if media.file_type().is_dir() {
            continue;
        }
        let path = media.path().to_path_buf();
        if let Some(ext) = path.extension() {
            let ext = ext.to_ascii_lowercase();
            if !(ext == "jpg" || ext == "mp4") {
                continue;
            }
            let name = media.file_name().to_string_lossy().into_owned();
            let meta = fs::metadata(&path).unwrap();
            let size = meta.len();
            let exif = if ext == "jpg" {
                if let Ok(exif) = get_image_exif(&path) {
                    exif
                } else {
                    None
                }
            } else {
                None
            };
            //时间
            let mut time = meta
                .created()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if let Some(exif)=&exif{
                //exif里面成功取时间了 就用exif的
                if let Ok(_time)=date_str_to_timestamp(&exif.shot_time){
                    time=_time;
                }else if let Ok(_time)=filename_to_timestamp(&name) {
                    //exif里没有 就去文件名里取
                    time=_time;
                }
                //再取不到 就用系统的修改时间
            }
            let info = MediaItem::new(*media_id, path, name, time, size, exif);
            gallery_medias.push(info);
            gallery_size += size;
            *media_id += 1;
        }
    }
    let gallery = Gallery::new(gallery_id, gallery_name, gallery_size, gallery_medias);
    Ok(Box::new(gallery))
}
