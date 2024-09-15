#[cfg(test)]
mod test {

    use std::{
        fs::{self},
        path::{Path, PathBuf},
        str::FromStr,
    };
    use std::fs::File;
    use std::io::BufReader;
    use image::{DynamicImage, ImageBuffer};
    use image::io::Reader;
    use libheif_rs::{ColorSpace, HeifContext, ItemId, LibHeif, RgbChroma};
    use tracing_test::traced_test;
    //use libheif_rs::{ColorSpace, HeifContext, ItemId, LibHeif, RgbChroma};

    use crate::{
        media_item::{compress_image_webp, MediaItem},
        util::ResultAnyErr,
    };
    use crate::media_item::{decode_heif_image, get_video_first_frame};
      #[test]
        fn read_and_decode_heic_file() -> ResultAnyErr<()> {
            let lib_heif = LibHeif::new();
            let file_path = "test/image2.heic";
            let ctx = HeifContext::read_from_file(file_path)?;
            let handle = ctx.primary_image_handle()?;
            println!("{}", handle.width());
            println!("{}", handle.height());
    
            // Get Exif
            let mut meta_ids: Vec<ItemId> = vec![0; 1];
            let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
            println!("{}", count);
            let exif: Vec<u8> = handle.metadata(meta_ids[0])?;
    
            // Decode the image
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
                    print!("no");
                    return Ok(());
                }
            };
            let webp_mem = compress_image_webp(&img, false)?.to_vec();
            fs::write("1.webp", &webp_mem)?;
    
            Ok(())
        }
   /* #[traced_test]
    #[test]
    fn read_and_decode_avif() -> ResultAnyErr<()> {
        let path = Path::new("C:\\Users\\liberatorch\\Downloads\\hato.profile0.10bpc.yuv420.no-cdef.no-restoration.avif");
       // let path = Path::new("E:\\camera\\situMT24\\0903\\Untitled Export\\20240903-_1049864.avif");
        //let image = MediaItem::decode_avif_image(path)?;/*
        let webp_mem = compress_image_webp(&image, false)?.to_vec();
        fs::write("1.webp", &webp_mem)?;*/

        Ok(())
    }*/
    fn _tbnl_maker(tbnl: bool)-> ResultAnyErr<()>{
        let image = decode_heif_image("test/image2.heic")?;
        let webp_mem = compress_image_webp(&image, tbnl)?.to_vec();
        fs::write(format!("test/output/image2_{}.webp",tbnl), &webp_mem)?;

        let mut img_reader = Reader::new(BufReader::new(File::open("test/image1.jpg")?)).with_guessed_format()?;
        img_reader.no_limits();
        let image = img_reader.decode()?;
        let webp_mem = compress_image_webp(&image, tbnl)?.to_vec();
        fs::write(format!("test/output/image1_{}.webp",tbnl), &webp_mem)?;

        let image = get_video_first_frame("test/video-h.mp4".to_string())?;
        let webp_mem = compress_image_webp(&image, tbnl)?.to_vec();
        fs::write(format!("test/output/image3_{}.webp",tbnl), &webp_mem)?;
        
        let image = get_video_first_frame("test/video-v.mp4".to_string())?;
        let webp_mem = compress_image_webp(&image, tbnl)?.to_vec();
        fs::write(format!("test/output/image4_{}.webp",tbnl), &webp_mem)?;
        Ok(())
    }
    #[test]
    fn tbnl_maker() -> ResultAnyErr<()>{
        _tbnl_maker(true)?;
        _tbnl_maker(false)?;
        Ok(())
        
    }
}
