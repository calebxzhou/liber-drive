use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use exif::{In, Tag};
use serde::{Deserialize, Serialize};

use crate::util::ResultAnyErr;
//照片摄影参数
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct ImageExif {
    //相机
    pub make: String,
    //镜头
    pub lens: String,
    //档位
    pub xp_prog: String,
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
    //测光模式
    pub meter_mode: String,
    //曝光补偿
    pub exp_cp: String,
    //闪光灯
    pub flash: String,
    //位置
    pub loca: Option<GPSLocation>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct GPSLocation {
    //高度
    pub alt: String,
    //经度
    pub lng: String,
    //纬度
    pub lat: String,
}
impl ImageExif {
    pub fn get_field_value(exif: &exif::Exif, tag: Tag) -> String {
        match exif.get_field(tag, In::PRIMARY) {
            None => "".to_string(),
            Some(field) => format!("{}", field.display_value()),
        }
    }

    //去除exif字段的杂乱字符
    fn trim_exif_field(exif: &exif::Exif, tag: Tag) -> String {
        Self::get_field_value(exif, tag)
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
        /* make = match make.to_lowercase().as_str() {
            "panasonic" => "松下",
            "canon" => "佳能",
            "nikon" => "尼康",
            "sony" => "索尼",
        }
        .to_string(); */
        let model = Self::trim_exif_field(&exif, Tag::Model);
        //型号里包含厂商 则去除
        let model = model.replace(&make, "");
        let make = format!("{} {}", make, model);
        //快门时间
        let tv = Self::trim_exif_field(&exif, Tag::ExposureTime);
        //档位 只保留第一个字母
        let xp_prog = Self::trim_exif_field(&exif, Tag::ExposureProgram);

        let av = Self::trim_exif_field(&exif, Tag::FNumber);
        //镜头
        let lens_make = Self::trim_exif_field(&exif, Tag::LensMake);
        let lens_model = Self::trim_exif_field(&exif, Tag::LensModel);
        let mut lens = lens_model.clone();
        if !lens_make.is_empty() {
            lens = format!("{}-{}", lens_make, lens_model);
        }
        //摄影时间
        let shot_time = Self::trim_exif_field(&exif, Tag::DateTimeOriginal);
        //焦距
        let mut focal_len = Self::trim_exif_field(&exif, Tag::FocalLengthIn35mmFilm);
        if focal_len.is_empty() {
            focal_len = Self::trim_exif_field(&exif, Tag::FocalLength);
        }
        //iso
        let iso = Self::trim_exif_field(&exif, Tag::PhotographicSensitivity);
        //测光模式
        let meter_mode = Self::trim_exif_field(&exif, Tag::MeteringMode);
        //曝光补偿
        let exp_cp = Self::trim_exif_field(&exif, Tag::ExposureBiasValue);
        //闪光灯
        let flash = Self::trim_exif_field(&exif, Tag::Flash);
        //GPS 高度
        let alt = Self::trim_exif_field(&exif, Tag::GPSAltitude);
        //纬度
        let lat = Self::trim_exif_field(&exif, Tag::GPSLatitude);
        //经度
        let lng = Self::trim_exif_field(&exif, Tag::GPSLongitude); 
        //todo 读取xmp:Rating获取图片星级
        let exif: ImageExif = ImageExif {
            make,
            lens,
            xp_prog,
            focal_len,
            av,
            tv,
            iso,
            shot_time,
            meter_mode,
            exp_cp,
            flash,
            loca: Some(GPSLocation { alt, lat, lng }),
        };
        Ok(exif)
    }
}
