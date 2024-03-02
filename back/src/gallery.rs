use core::fmt;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{media_item::MediaItem, util::human_readable_size};

#[derive(Serialize,Clone)]
pub struct Gallery{
    id: u32,
    name: String,
    size: u64,
    pub medias: HashMap<u32,MediaItem>
}
impl Gallery {
    pub fn new(id: u32, name: String, size: u64, medias: HashMap<u32, MediaItem>) -> Self {
        Self { id, name, size, medias }
    }
}
impl fmt::Display for Gallery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {} {}x {}", self.id, self.name ,self.medias.len(),human_readable_size(self.size))
    }
}
