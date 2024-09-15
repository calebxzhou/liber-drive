use core::fmt;
use std::collections::HashMap;

use serde::Serialize;

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
impl Album {
    pub fn new(name: String, medias: HashMap<String, MediaItem>, sub_albums: HashMap<String, Album>, pwd: Option<String>) -> Self {
        Self { name, medias, sub_albums, pwd }
    }
}
impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}x ", self.name, self.medias.len())
    }
}
