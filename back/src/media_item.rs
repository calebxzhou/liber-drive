use serde::Serialize;

#[derive(Serialize)]
pub struct MediaItem {
    id: u32,
    #[serde(skip_serializing)]
    path: String,
    name: String,
    time: u64,
    size: u64,
}
impl MediaItem {
    pub fn new(id: u32, path: String, name: String, time: u64, size: u64) -> MediaItem {
        Self {
            id,
            path,
            name,
            time,
            size,
        }
    }
}
