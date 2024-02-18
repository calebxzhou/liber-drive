#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn it_works() {
        let file = File::open("/Users/calebzhou/Pictures/240215hld/_1021890.jpg").unwrap();
        let mut bufreader = BufReader::new(&file);

        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).unwrap();

        for field in exif.fields() {
            println!("Tag: {} (IFD: {}) - Value: {}", field.tag, field.ifd_num, field.display_value().with_unit(&exif));
        }
    }
}