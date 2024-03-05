use core::fmt; 
use rayon::vec;
use serde::{Deserialize, Serialize};

use crate::{media_item::MediaItem, util::human_readable_size};

#[derive(Serialize,Clone)]
pub struct Gallery{
    pub id: u32,
    pub name: String,
    pub size: u64,
    pub medias: Vec<MediaItem>
}
impl Gallery {
    pub fn new(id: u32, name: String, size: u64, medias: Vec<MediaItem>) -> Self {
        Self { id, name, size, medias }
    }
}
impl fmt::Display for Gallery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {} {}x {}", self.id, self.name ,self.medias.len(),human_readable_size(self.size))
    }
}
#[derive(Serialize,Clone)]
pub struct GalleryInfo{
    pub id: u32,
    pub name: String,
    pub size: u64,
    pub media_amount: u32,
    //预览图4张
    pub tbnl_media_ids: Vec<u32>


}
impl GalleryInfo{
    
    pub fn get_tbnl_media_ids(medias:&Vec<MediaItem>)->Vec<u32>{
        let mut medias = medias.iter().map(|m|m.id).collect::<Vec<u32>>();
        let len = medias.len();
        if medias.len() < 4 {
            // clone the last element and repeat it until four
            if let None = medias.last() {
                return vec![];
            }
            let last = medias.last().cloned().unwrap();
            medias.extend(std::iter::repeat(last).take(4 -  medias.len()));
        }
        let medias = medias.iter().filter(|&&x| {
            // keep the first, last, 1/3, and 2/3 elements
            x == medias[0] || x == medias[len - 1] || x == medias[len / 3] || x == medias[2 * len / 3]
        }).cloned().collect::<Vec<u32>>();
        medias
    }
}