use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::{collect_media_from_sub_albums, get_selected_media_items};
use crate::media_item::MediaItem;

//影集
#[derive(Serialize, Deserialize, Clone)]
pub struct Album {
    pub name: String,
    pub medias: HashMap<String, MediaItem>,
    pub sub_albums: HashMap<String, Album>,
    #[serde(skip_serializing)]
    pub pwd: Option<String>,
}
#[derive(Serialize, Clone)]
pub struct AlbumInfo {
    pub name: String,
    pub medias: Vec<MediaItem>,
    //相册名带4个缩图
    pub sub_albums: Vec<AlbumTbnl>,
}
#[derive(Serialize, Clone)]
pub struct AlbumTbnl {
    pub parent: String,
    pub name: String,
    pub tbnl_names: Vec<String>,
}
impl Album {
    pub fn new(name: String, medias: HashMap<String, MediaItem>, sub_albums: HashMap<String, Album>, pwd: Option<String>) -> Self {
        Self { name, medias, sub_albums, pwd }
    }
    pub fn into_info(self) -> AlbumInfo {
        let medias = self.medias.values().cloned().collect();
        let mut sub_albums = Vec::new();
        for (key, album) in self.sub_albums {
            let mut media_items: Vec<&MediaItem> = album.medias.values().collect();
            //不够4个 去子相册里拿
            if media_items.len() < 4 {
                collect_media_from_sub_albums(&album, &mut media_items);
            }

            let selected_items = get_selected_media_items(media_items);
            sub_albums.push(AlbumTbnl { parent: self.name.clone(),name: album.name.clone(), tbnl_names: selected_items });
        }
        AlbumInfo {
            name: self.name.clone(),
            medias,
            sub_albums,
        }
    }
}
impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}x ", self.name, self.medias.len())
    }
}
