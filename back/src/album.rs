use core::fmt;
use std::collections::HashMap;

use serde::Serialize;

use crate::media_item::MediaItem;

//影集
#[derive(Serialize, Clone)]
pub struct Album {
    pub name: String,
    pub medias: HashMap<String, MediaItem>,
}
impl Album {
    pub fn new(name: String, medias: HashMap<String, MediaItem>) -> Self {
        Self { name, medias }
    }
}
impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}x ", self.name, self.medias.len())
    }
}
