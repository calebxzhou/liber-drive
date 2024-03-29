use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fmt, fs, path::PathBuf, time::UNIX_EPOCH};

use image::imageops::FilterType;
use image::DynamicImage;
use mp4::Mp4Reader;
use serde::{Serialize, Serializer};
use webp::WebPMemory;

use crate::image_exif::ImageExif;
use crate::util::{human_readable_size, AnyError, ResultAnyErr};

//图片/视频
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct MediaItem {
    pub name: String,
    #[serde(skip_serializing)]
    pub path: PathBuf,
    pub time: u64,
    pub size: u64,
    pub exif: Option<ImageExif>,
    pub duration: Option<u16>,
}

impl MediaItem {
    pub fn new(path: PathBuf, name: String, time: u64, size: u64) -> Self {
        Self {
            path,
            name,
            time,
            size,
            exif: None,
            duration: None,
        }
    }
    pub fn get_extension(&self) -> String {
        self.path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_ascii_lowercase()
            .to_string_lossy()
            .into_owned()
    }
    //预览路径
    pub fn get_preview_path(&self, thumbnail: bool) -> PathBuf {
        Path::new("cache").join(format!(
            "tb_{}_{}_{}.webp",
            if thumbnail { 2 } else { 0 },
            self.size,
            self.name.to_lowercase()
        ))
    }
    //是否已创建预览 小图+大图
    pub fn is_preview_created(&self, thumbnail: bool) -> bool {
        self.get_preview_path(thumbnail).exists()
    }
    //获取预览
    pub fn get_preview(&self, thumbnail: bool) -> ResultAnyErr<Vec<u8>> {
        //没预览
        if !self.is_preview_created(thumbnail) {
            self.create_preview(thumbnail)?;
        }
        let mut file = std::fs::File::open(&self.get_preview_path(thumbnail))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    //创建预览
    pub fn create_preview(&self, thumbnail: bool) -> ResultAnyErr<()> {
        let path = &self.path;

        let image = if is_video(path) {
            //是视频 截取第一帧
            get_video_first_frame(&path.display().to_string())?
        } else if is_image(path) {
            //是图片
            image::open(path)?
        } else {
            return Err("nor img/video!".into());
        };
        //缩略图
        let webp_mem = compress_image_webp(&image, thumbnail)?.to_vec();
        //保存 缓存
        fs::write(&self.get_preview_path(thumbnail), &webp_mem)?;
        Ok(())
    }
}
//压缩图片为webp格式
pub fn compress_image_webp(image: &DynamicImage, thumbnail: bool) -> ResultAnyErr<WebPMemory> {
    let width = image.width();
    let webp_mem = if thumbnail {
        //缩略图
        let image = image.resize(width, 256, FilterType::Nearest);
        webp::Encoder::from_image(&image)?.encode(80f32)
    } else {
        //无高度（L0） 压原图
        webp::Encoder::from_image(&image)?.encode(40f32)
    };

    Ok(webp_mem)
}

//匹配文件扩展名 是否在extensions里
fn matches_extension(path: &PathBuf, extensions: &[&str]) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_ascii_lowercase();
        let ext = &ext.to_string_lossy();
        extensions.iter().any(|&e| ext == e)
    } else {
        false
    }
}
//是否视频
pub fn is_video(path: &PathBuf) -> bool {
    matches_extension(path, &["mp4"])
}
//是否图片
pub fn is_image(path: &PathBuf) -> bool {
    matches_extension(path, &["png", "jpg"])
}
pub fn is_heif_image(path: &PathBuf) -> bool {
    matches_extension(path, &["heif", "heic"])
}
//文件创建时间timestamp
pub fn get_file_created_time(path: &PathBuf) -> std::result::Result<u64, AnyError> {
    let meta = fs::metadata(&path)?;
    let time = meta
        .created()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    Ok(time)
}
pub fn get_file_size(path: &PathBuf) -> ResultAnyErr<u64> {
    let meta = fs::metadata(&path)?;
    Ok(meta.len())
}
//读取视频第一帧
pub fn get_video_first_frame(video_path: &String) -> ResultAnyErr<DynamicImage> {
    let mut ffmpeg_output = Command::new("ffmpeg")
        .arg("-i") // input file
        .arg(video_path) // replace with your file
        .arg("-vframes") // number of video frames to output
        .arg("1") // we only want the first frame
        .arg("-f") // force format
        .arg("image2pipe") // pipe image data to stdout
        .arg("-") // output to stdout
        .stdout(Stdio::piped()) // capture stdout
        .spawn()?
        .stdout
        .ok_or("can not capture stdout")?;

    let mut buffer = Vec::new();
    ffmpeg_output.read_to_end(&mut buffer)?;

    let cursor = Cursor::new(buffer);

    let img = image::io::Reader::new(cursor)
        .with_guessed_format()?
        .decode()?;
    Ok(img)
}
//读取视频时长
pub fn get_video_duration(path: &str) -> ResultAnyErr<u16> {
    /* let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output()?;

    let duration_str = std::str::from_utf8(&output.stdout)?.trim();
    let duration = duration_str.parse::<f64>()?; */
    let file = File::open(path)?;
    let size = file.metadata()?.len();
    let reader = BufReader::new(file);
    let mp4 = Mp4Reader::read_header(reader, size)?;

    Ok(mp4.duration().as_secs() as u16)
}
//相册
#[derive(Serialize, Clone)]
pub struct Gallery {
    pub name: String,
    pub albums: HashMap<String, Album>,
}
#[derive(Serialize, Clone)]
pub struct GalleryInfo {
    pub name: String,
    pub albums: Vec<AlbumInfo>,
}
impl Gallery {
    pub fn new(name: String, albums: HashMap<String, Album>) -> Self {
        Self { name, albums }
    }
}
impl GalleryInfo {
    pub fn from_gallery(gallery: &Gallery) -> Self {
        let albums: Vec<AlbumInfo> = gallery
            .albums
            .values()
            .map(|a| AlbumInfo::from_album(a))
            .collect();
        GalleryInfo {
            name: gallery.name.clone(),
            albums,
        }
    }
}
impl fmt::Display for Gallery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}x", self.name, self.albums.len())
    }
}

//影集
#[derive(Serialize, Clone)]
pub struct Album {
    pub name: String,
    pub medias: HashMap<String, MediaItem>,
}
#[derive(Serialize, Clone)]
pub struct AlbumInfo {
    pub name: String,
}
impl AlbumInfo {
    pub fn from_album(album: &Album) -> Self {
        AlbumInfo {
            name: album.name.clone(),
        }
    }
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
