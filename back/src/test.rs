#[cfg(test)]
mod test {

    use std::fs::{self, File};

    use image::{DynamicImage, ImageBuffer};
    use libheif_rs::{Channel, ColorSpace, HeifContext, ItemId, LibHeif, Result, RgbChroma};

    use crate::{media_item::compress_image_webp, util::ResultAnyErr};

    #[test]
    fn read_and_decode_heic_file() -> ResultAnyErr<()> {
        let lib_heif = LibHeif::new();
        let file_path = "c:\\Users\\calebxzhou\\Pictures\\IMG_20220111_112225.HEIC";
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
}
