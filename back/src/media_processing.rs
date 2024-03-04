use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use exif::{In, Tag};
use image::imageops::FilterType; 
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    //1=512x缩图 2=256x缩图 3=128缩图 4=64缩图 5=32缩图
    static ref PREVIEW_PIXELS : Vec<u32> = [512,256,128,64,32].iter().cloned().collect();
    static ref PREVIEW_LEVEL_PIXELS :HashMap<u8,u32> = [(1,512),(2,256),(3,128),(4,64),(5,32)].iter().cloned().collect();
}
#[derive(Serialize, Clone)]
pub struct ImageExif {
    //相机
    make: String,
    //镜头
    lens: String,
    //档位
    xp_prog: char,
    //焦距
    focal_len: String,
    //光圈
    aperture: String,
    //快门
    shutter: String,
    //ISO
    iso: String,
    //拍摄时间
    shot_time: String,
}
impl ImageExif {
    pub fn new(
        make: String,
        lens: String,
        xp_prog: char,
        focal_len: String,
        aperture: String,
        shutter: String,
        iso: String,
        shot_time: String,
    ) -> Self {
        Self {
            make,
            lens,
            xp_prog,
            focal_len,
            aperture,
            shutter,
            iso,
            shot_time,
        }
    }
    pub fn get_field_value(exif: &exif::Exif, tag: Tag) -> String {
        match exif.get_field(tag, In::PRIMARY) {
            None => "".to_string(),
            Some(field) => format!("{}", field.display_value().with_unit(exif)),
        }
    }
}
//创建媒体文件预览（缩略图）//预览级别 0=webp压图 1=512x缩图 2=256x缩图 3=128缩图 4=64缩图 5=32缩图
pub fn get_media_preview(level: &u8, media_path: &PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
    let height = PREVIEW_LEVEL_PIXELS.get(level);
    let file_size = fs::metadata(media_path)?.len();
    let file_name = media_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase();

    let cache_path =
        Path::new("cache").join(&format!("tb_{}_{}_{}.webp", level, file_size, file_name));
    //缓存存在
    if cache_path.exists() {
        let mut file = std::fs::File::open(&cache_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    //缓存不存在
    else {
        if file_name.ends_with(".mp4") || file_name.ends_with(".mov") {
            //是视频 截取第一帧
            let mut ffmpeg_output = Command::new("ffmpeg")
                .arg("-i") // input file
                .arg(media_path.display().to_string()) // replace with your file
                .arg("-vframes") // number of video frames to output
                .arg("1") // we only want the first frame
                .arg("-f") // force format
                .arg("image2pipe") // pipe image data to stdout
                .arg("-") // output to stdout
                .stdout(Stdio::piped()) // capture stdout
                .spawn()
                .expect("Failed to execute FFmpeg command")
                .stdout
                .expect("Failed to capture stdout");

            let mut buffer = Vec::new();
            ffmpeg_output
                .read_to_end(&mut buffer)
                .expect("Failed to read FFmpeg output");

            let cursor = Cursor::new(buffer);

            let img = image::io::Reader::new(cursor)
                .with_guessed_format()
                .expect("Failed to guess image format")
                .decode()
                .expect("Failed to decode image");
            let width = img.width();
            let webp_mem = if let Some(height) = height {
                let image = img.resize(width, *height, FilterType::Nearest);
                webp::Encoder::from_image(&image)?.encode(80f32)
            } else {
                webp::Encoder::from_image(&img)?.encode(40f32)
            };
            fs::write(&cache_path, &*webp_mem)?;
            Ok(webp_mem.to_vec())
        } else if file_name.ends_with(".jpg") || file_name.ends_with(".png") {
            //是图片
            let image = image::open(media_path)?;
            let width = image.width();
            
            //缩略图
            let webp_mem = if let Some(height) = height {
                let image = image.resize(width, *height, FilterType::Nearest);
                webp::Encoder::from_image(&image)?.encode(80f32)
            } else {
                webp::Encoder::from_image(&image)?.encode(40f32)
            };
            fs::write(&cache_path, &*webp_mem)?;
            Ok(webp_mem.to_vec())
        } else {
            Ok(vec![])
        }
    }
}
fn get_exif_field(exif: &exif::Exif, tag: Tag) -> String {
    ImageExif::get_field_value(exif, tag)
        .replace("\"", "")
        .replace(",", "")
        .trim()
        .to_string()
}
pub fn get_image_exif(media_path: &PathBuf) -> Result<Option<ImageExif>, Box<dyn Error>> {
    let file = File::open(media_path)?;
    let mut bufreader = BufReader::new(&file);

    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    let make = get_exif_field(&exif, Tag::Make);
    if make.is_empty() {
        return Ok(None);
    }
    let model = get_exif_field(&exif, Tag::Model);
    let exposure_time = get_exif_field(&exif, Tag::ExposureTime);
    let exposure_program = get_exif_field(&exif, Tag::ExposureProgram)
        .chars()
        .next()
        .unwrap_or(' ');
    let f_number = get_exif_field(&exif, Tag::FNumber);
    let lens_make = get_exif_field(&exif, Tag::LensMake);
    let lens_model = get_exif_field(&exif, Tag::LensModel);
    let date_time = get_exif_field(&exif, Tag::DateTimeOriginal);
    let focal_length = get_exif_field(&exif, Tag::FocalLength);
    let iso = get_exif_field(&exif, Tag::PhotographicSensitivity);
    let exif = ImageExif::new(
        format!("{} {}", make, model),
        format!("{} {}", lens_make, lens_model),
        exposure_program.to_ascii_uppercase(),
        focal_length,
        f_number,
        exposure_time,
        iso,
        date_time,
    );
    Ok(Some(exif))
}
