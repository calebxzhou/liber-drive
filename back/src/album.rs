use core::fmt;
use std::collections::HashMap;
use std::ops::Deref;
use log::{error, info};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{collect_media_from_sub_albums, get_selected_media_items};
use crate::media_item::MediaItem;

//影集
#[derive(Serialize, Clone)]
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
    pub name: String,
    pub tbnl_ids: Vec<String>,
}
impl Album {
    pub fn new(name: String) -> Self {
        Self { name, medias:HashMap::new(), sub_albums:HashMap::new(), pwd : Option::None}
    }
    pub fn into_info(self) -> AlbumInfo {
        let medias = self.medias.values().cloned().collect();
        let mut sub_albums = Vec::new();
        for (key, album) in self.sub_albums {
            let mut media_items:Vec<String> = album.medias.values().map(|i|i.get_media_id().clone()).collect();
            //不够4个 去子相册里拿
            if media_items.len() < 4 {
                collect_media_from_sub_albums(&album, &mut media_items);
            }

            let selected_items = get_selected_media_items(media_items);
            sub_albums.push(AlbumTbnl { name: album.name.clone(), tbnl_ids: selected_items });
        }
        AlbumInfo {
            name: self.name.clone(),
            medias,
            sub_albums,
        }
    }
    pub fn build_tbnls(&self)  {
        info!("开始建立缩略图: {}", self.name);
        self.medias.par_iter().for_each(|(media_name, media_item)| {
            if let Err(e) = media_item.create_preview(true) {
                error!("创建小图错误：{:?}", e);
            }
            if let Err(e) = media_item.create_preview(false) {
                error!("创建大图错误：{:?}", e);
            }
        });
        self.sub_albums.iter().for_each(|(sub_album_name, sub_album)| {
            info!("开始建立子相册缩略图: {}", sub_album_name);
            sub_album.build_tbnls();
        });
    }
}
impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}x ", self.name, self.medias.len())
    }
}
