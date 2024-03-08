use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fmt, fs, path::PathBuf, time::UNIX_EPOCH};

use exif::{In, Tag};
use image::imageops::FilterType;
use image::DynamicImage; 
use serde::Serialize;
use webp::WebPMemory; 
 
use crate::util::{human_readable_size, AnyError, ResultAnyErr}; 
 
#[derive(Serialize, Clone)]
pub struct MediaItem {
    pub id: u32,
    #[serde(skip_serializing)]
    pub path: PathBuf,
    pub name: String,
    pub time: u64,
    pub size: u64,
    pub exif: Option<ImageExif>,
}

impl MediaItem {
    pub fn new(
        id: u32,
        path: PathBuf,
        name: String,
        time: u64,
        size: u64,
        exif: Option<ImageExif>,
    ) -> Self {
        Self {
            id,
            path,
            name,
            time,
            size,
            exif,
        }
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
    pub fn is_preview_created(&self) -> bool {
        self.get_preview_path(true).exists() && self.get_preview_path(false).exists()
    }
    //获取预览
    pub fn get_preview(&self, thumbnail: bool) -> ResultAnyErr<Vec<u8>> {
        //没预览
        if !self.is_preview_created() {
            self.create_preview()?;
        }
        let mut file = std::fs::File::open(&self.get_preview_path(thumbnail))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    //创建预览
    pub fn create_preview(&self) -> ResultAnyErr<()> {
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
        let webp_mem = compress_image_webp(&image, true)?.to_vec();
        //保存小图缓存
        fs::write(&self.get_preview_path(true), &webp_mem)?;
        let webp_mem = compress_image_webp(&image, false)?.to_vec();
        //保存大图缓存
        fs::write(&self.get_preview_path(false), &webp_mem)?;
        Ok(())
    }
}
impl fmt::Display for MediaItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.id, self.path.display())
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
    matches_extension(path, &["mp4", "mov"])
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

//照片摄影参数
#[derive(Serialize, Clone)]
pub struct ImageExif {
    //相机
    pub make: String,
    //镜头
    pub lens: String,
    //档位
    pub xp_prog: char,
    //焦距
    pub focal_len: String,
    //光圈
    pub av: String,
    //快门
    pub tv: String,
    //ISO
    pub iso: String,
    //拍摄时间
    pub shot_time: String,
}
impl ImageExif {
    pub fn new(
        make: String,
        lens: String,
        xp_prog: char,
        focal_len: String,
        av: String,
        tv: String,
        iso: String,
        shot_time: String,
    ) -> Self {
        Self {
            make,
            lens,
            xp_prog,
            focal_len,
            av,
            tv,
            iso,
            shot_time,
        }
    }

    pub fn get_field_value(exif: &exif::Exif, tag: Tag) -> String {
        match exif.get_field(tag, In::PRIMARY) {
            None => "".to_owned(),
            Some(field) => format!("{}", field.display_value()),
        }
    }

    //exif字段
    fn get_exif_field(exif: &exif::Exif, tag: Tag) -> String {
        ImageExif::get_field_value(exif, tag)
            .replace(['\"', ','], "")
            .trim()
            .to_owned()
    }

    pub fn from_media_path(media_path: &PathBuf) -> ResultAnyErr<ImageExif> {
        let file = File::open(media_path)?;
        let mut bufreader = BufReader::new(&file);

        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        //相机厂商
        let make = Self::get_exif_field(&exif, Tag::Make);
        //相机型号
        let model = Self::get_exif_field(&exif, Tag::Model)
            //型号里包含厂商 则去除
            .replace(&make, "");
        let make = format!("{} {}", make, model);
        //快门时间
        let tv = Self::get_exif_field(&exif, Tag::ExposureTime);
        //档位 只保留第一个字母
        let xp_prog = Self::get_exif_field(&exif, Tag::ExposureProgram)
            .chars()
            .next()
            .unwrap_or(' ')
            .to_ascii_uppercase();
        let av = Self::get_exif_field(&exif, Tag::FNumber);
        //镜头
        let lens_make = Self::get_exif_field(&exif, Tag::LensMake);
        let lens_model = Self::get_exif_field(&exif, Tag::LensModel);
        let lens = if lens_model.contains("| Art") {
            format!("Sigma {} {}", lens_make, lens_model)
        } else {
            format!("{} {}", lens_make, lens_model)
        };

        let shot_time = Self::get_exif_field(&exif, Tag::DateTimeOriginal);
        let focal_len = Self::get_exif_field(&exif, Tag::FocalLength);
        let iso = Self::get_exif_field(&exif, Tag::PhotographicSensitivity);
        let exif: ImageExif =
            ImageExif::new(make, lens, xp_prog, focal_len, av, tv, iso, shot_time);
        Ok(exif)
    }
}

//读取视频第一帧
pub fn get_video_first_frame(video_path: &String) -> Result<DynamicImage, AnyError> {
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

//相册
#[derive(Serialize)]
pub struct Gallery {
    pub id: u32,
    pub name: String,
    pub size: u64,
    pub medias: Vec<MediaItem>,
}
impl Gallery {
    pub fn new(id: u32, name: String, size: u64, medias: Vec<MediaItem>) -> Self {
        Self {
            id,
            name,
            size,
            medias,
        }
    }
    //封面图片ID
    pub fn get_cover_media_id(&self) -> u32 {
        self.medias
            .iter()
            .map(|m| m.id)
            .collect::<Vec<u32>>()
            .get(0)
            .unwrap_or(&0)
            .clone()
    }
}
impl fmt::Display for Gallery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}] {} {}x {}",
            self.id,
            self.name,
            self.medias.len(),
            human_readable_size(self.size)
        )
    }
}
#[derive(Serialize, Clone)]
pub struct GalleryInfo {
    pub id: u32,
    pub name: String,
    pub size: u64,
    pub media_amount: u32,
    //预览图1张
    pub tbnl_media_id: u32,
} 
 