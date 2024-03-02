#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use log::info;

    use crate::media_scanner::{scan_all_galleries, scan_medias};
    #[test]
    fn it_works() {
        env_logger::init();
        let path = Path::new("D:/相机").to_path_buf();
        let medias = scan_medias(&path);
        info!("{} medias", medias.len());
        let galleries = scan_all_galleries(&path);
        info!("{} galleries", galleries.len());
    }
}
