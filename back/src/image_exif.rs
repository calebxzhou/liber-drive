use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use exif::{In, Tag}; 
use serde::{Deserialize, Serialize};

use crate::util::ResultAnyErr;

// 照片摄影参数
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct ImageExif {
    pub make: String,
    pub lens: String,
    pub xp_prog: String,
    pub focal_len: String,
    pub av: String,
    pub tv: String,
    pub iso: String,
    pub shot_time: String,
    pub meter_mode: String,
    pub exp_cp: String,
    pub flash: String,
    pub loca: Option<GPSLocation>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GPSLocation {
    pub alt: String,
    pub lng: String,
    pub lat: String,
}

impl ImageExif {
    fn get_field_value(exif: &exif::Exif, tag: Tag) -> String {
        exif.get_field(tag, In::PRIMARY)
            .map_or_else(|| "".to_string(), |field| field.display_value().to_string())
    }

    fn trim_exif_field(exif: &exif::Exif, tag: Tag) -> String {
        Self::get_field_value(exif, tag)
            .replace(['\"', ','], "")
            .trim()
            .to_owned()
    }

    pub fn from_media_path(media_path: &PathBuf) -> ResultAnyErr<ImageExif> {
        let file = File::open(media_path)?;
        let mut bufreader = BufReader::new(&file);

        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        let make = Self::trim_exif_field(&exif, Tag::Make);
        let model = Self::trim_exif_field(&exif, Tag::Model).replace(&make, "");
        let make = format!("{} {}", make, model);

        let lens_make = Self::trim_exif_field(&exif, Tag::LensMake);
        let lens_model = Self::trim_exif_field(&exif, Tag::LensModel);
        let lens = if lens_make.is_empty() {
            lens_model.clone()
        } else {
            format!("{}-{}", lens_make, lens_model)
        };

        let focal_len = Self::trim_exif_field(&exif, Tag::FocalLengthIn35mmFilm);
        let focal_len = if focal_len.is_empty() {
            Self::trim_exif_field(&exif, Tag::FocalLength)
        } else {
            focal_len
        };
        let exif = ImageExif {
            make,
            lens,
            xp_prog: Self::trim_exif_field(&exif, Tag::ExposureProgram),
            focal_len,
            av: Self::trim_exif_field(&exif, Tag::FNumber),
            tv: Self::trim_exif_field(&exif, Tag::ExposureTime),
            iso: Self::trim_exif_field(&exif, Tag::PhotographicSensitivity),
            shot_time: Self::trim_exif_field(&exif, Tag::DateTimeOriginal),
            meter_mode: Self::trim_exif_field(&exif, Tag::MeteringMode),
            exp_cp: Self::trim_exif_field(&exif, Tag::ExposureBiasValue),
            flash: Self::trim_exif_field(&exif, Tag::Flash),
            loca: Some(GPSLocation {
                alt: Self::trim_exif_field(&exif, Tag::GPSAltitude),
                lat: Self::trim_exif_field(&exif, Tag::GPSLatitude),
                lng: Self::trim_exif_field(&exif, Tag::GPSLongitude),
            }),
        };

        Ok(exif)
    }
}
