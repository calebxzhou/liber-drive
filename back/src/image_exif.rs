use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use exif::{In, Tag}; 
use serde::Serialize; 

use crate::util::ResultAnyErr; 
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

    //去除exif字段的杂乱字符
    fn trim_exif_field(exif: &exif::Exif, tag: Tag) -> String {
        ImageExif::get_field_value(exif, tag)
            .replace(['\"', ','], "")
            .trim()
            .to_owned()
    }
    //从路径读exif
    pub fn from_media_path(media_path: &PathBuf) -> ResultAnyErr<ImageExif> {
        let file = File::open(media_path)?;
        let mut bufreader = BufReader::new(&file);

        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        //相机厂商
        let make = Self::trim_exif_field(&exif, Tag::Make);
        //相机型号
        let model = Self::trim_exif_field(&exif, Tag::Model)
            //型号里包含厂商 则去除
            .replace(&make, "");
        let make = format!("{} {}", make, model);
        //快门时间
        let tv = Self::trim_exif_field(&exif, Tag::ExposureTime);
        //档位 只保留第一个字母
        let xp_prog = Self::trim_exif_field(&exif, Tag::ExposureProgram)
            .chars()
            .next()
            .unwrap_or(' ')
            .to_ascii_uppercase();
        let av = Self::trim_exif_field(&exif, Tag::FNumber);
        //镜头
        let lens_make = Self::trim_exif_field(&exif, Tag::LensMake);
        let lens_model = Self::trim_exif_field(&exif, Tag::LensModel);
        let lens = if lens_model.contains("| Art") {
            format!("Sigma {} {}", lens_make, lens_model)
        } else {
            format!("{} {}", lens_make, lens_model)
        };

        let shot_time = Self::trim_exif_field(&exif, Tag::DateTimeOriginal);
        let focal_len = Self::trim_exif_field(&exif, Tag::FocalLength);
        let iso = Self::trim_exif_field(&exif, Tag::PhotographicSensitivity);
        let exif: ImageExif =
            ImageExif::new(make, lens, xp_prog, focal_len, av, tv, iso, shot_time);
        Ok(exif)
    }
}