use std::collections::HashMap;
use std::ffi::OsStr;

use std::io::{Cursor, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fmt, fs, path::PathBuf, time::UNIX_EPOCH};

use image::imageops::FilterType;
use image::{error, DynamicImage, ImageBuffer};
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};
use log::info;
use mp4::Mp4Reader;
use serde::{Serialize, Serializer};
use webp::WebPMemory;

use crate::image_exif::{self, ImageExif};
use crate::util::{
    date_str_to_timestamp, filename_to_timestamp, human_readable_size, AnyError, ResultAnyErr,
};

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
    //获取扩展名（小写）
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
            //返回空的
            return Err("缩略图不存在".into());
        }
        let mut file = std::fs::File::open(&self.get_preview_path(thumbnail))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    //创建预览
    pub fn create_preview(&self, thumbnail: bool) -> ResultAnyErr<()> {
        if self.is_preview_created(thumbnail) {
            return Ok(());
        }
        let path = &self.path;
        info!("正在创建缩略图{:?}", path);

        let image = if is_video(path) {
            //是视频 截取第一帧
            get_video_first_frame(&path.display().to_string())?
        } else if is_heif_image(path) {
            decode_heif_image(
                path.to_str()
                    .ok_or(format!("无法解码图片路径{:?}，包含无效字符", path))?,
            )?
        } else if is_image(path) {
            //是图片
            image::open(path)?
        } else {
            return Err(format!("{:?}不是照片视频", path).into());
        };
        //缩略图
        let webp_mem = compress_image_webp(&image, thumbnail)?.to_vec();
        //保存 缓存
        fs::write(&self.get_preview_path(thumbnail), &webp_mem)?;
        Ok(())
    }
    //exif信息缓存路径
    pub fn get_exif_cache_path(&self) -> PathBuf {
        Path::new("cache").join("exif").join(format!(
            "exif_{}_{}.json",
            self.size,
            self.name.to_lowercase()
        ))
    }
    //视频时长信息缓存路径
    pub fn get_video_duration_cache_path(&self) -> PathBuf {
        Path::new("cache").join("video").join(format!(
            "duration_{}_{}.txt",
            self.size,
            self.name.to_lowercase()
        ))
    }
    //创建exif信息缓存
    pub fn create_exif_cache(&self) -> ResultAnyErr<()> {
        info!("正在创建exif缓存 {:?}", self.path);
        if self.get_exif_cache_path().exists() {
            info!("缓存已存在，不需要创建");
            return Ok(());
        }
        let exif = ImageExif::from_media_path(&self.path);
        if let Ok(exif) = exif {
            let json = serde_json::to_string(&exif)?;
            fs::write(&self.get_exif_cache_path(), json)?;
        }
        Ok(())
    }
    //读取exif信息缓存
    pub fn read_exif_cache(&mut self) -> ResultAnyErr<()> {
        let path = self.get_exif_cache_path();
        if !path.exists() {
            info!("{:?} 缓存不存在", path);
            return Ok(());
        }
        let data = fs::read_to_string(path)?;
        let exif: ImageExif = serde_json::from_str(&data)?;
        self.exif = Some(exif);
        Ok(())
    }
    //读取视频时长信息缓存
    pub fn read_video_duration_cache(&mut self) -> ResultAnyErr<()> {
        let path = self.get_video_duration_cache_path();
        if !path.exists() {
            info!("{:?} 缓存不存在", path);
            return Ok(());
        }
        let data = fs::read_to_string(path)?.parse::<u16>()?;
        self.duration = Some(data);
        Ok(())
    }
    //创建视频时长信息缓存
    pub fn create_video_duration_cache(&self) -> ResultAnyErr<()> {
        info!("正在创建视频时长缓存 {:?}", self.path);
        if self.get_exif_cache_path().exists() {
            info!("缓存已存在，不需要创建");
            return Ok(());
        }
        let duration = if is_video(&self.path) {
            get_video_duration(self.path.to_str().unwrap())?
        } else {
            return Err(format!("{}不是视频 无法获取视频时长", self.name).into());
        };
        fs::write(&self.get_video_duration_cache_path(), duration.to_string())?;
        Ok(())
    }
    //更新照片修改时间（从exif/文件名读取）
    pub fn update_media_time(&mut self) {
        let mut time = self.time;
        if let Some(exif) = &self.exif {
            //exif里面成功取时间了 就用exif的
            if let Ok(_time) = date_str_to_timestamp(&exif.shot_time) {
                time = _time;
            } else if let Ok(_time) = filename_to_timestamp(&self.name) {
                //exif里没有 就去文件名里取
                time = _time;
            }
            //再取不到 就用系统的修改时间（什么都不动）
        }
        self.time = time;
    }
}
//解码heif图片为DynamicImage
pub fn decode_heif_image(path: &str) -> ResultAnyErr<DynamicImage> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file(path)?;
    let handle = ctx.primary_image_handle()?;
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    println!("{:?} ", image.color_space());

    let width = image.width();
    let height = image.height();
    let planes = image.planes();
    let interleaved_plane = planes.interleaved.unwrap();
    let img = match ImageBuffer::from_raw(width, height, interleaved_plane.data.to_owned())
        .map(DynamicImage::ImageRgb8)
    {
        Some(a) => a,
        None => {
            return Err(format!("无法解码图片{}", path).into());
        }
    };
    Ok(img)
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
    matches_extension(path, &["png", "jpg"]) || is_heif_image(path)
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
    let output = Command::new("ffprobe")
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
    let duration = duration_str.parse::<f64>()?;
    Ok(duration as u16)
    /* let file = File::open(path)?;
    let size = file.metadata()?.len();
    let reader = BufReader::new(file);
    let mp4 = Mp4Reader::read_header(reader, size)?;

    Ok(mp4.duration().as_secs() as u16) */
}
