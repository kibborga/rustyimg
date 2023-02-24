pub mod imageaction {

    use crate::debug;
    use crate::log;

    pub use crate::config::*;
    pub use crate::rustyexif::*;
    pub use crate::rustyimg::*;

    pub use config::*;

    use std::ffi::OsStr;
    use std::fs;
    use std::path::Path;

    use filetime::{set_file_mtime, FileTime};
    use imgproc_rs::image::{Image, ImageInfo};
    use imgproc_rs::io;
    use libheif_rs::{Channel, ColorSpace, HeifContext, RgbChroma};

    pub use std::fs::*;

    pub fn convert_heic(src_file: &str, opts: &ConfigOptions) -> u8 {
        match get_dest_name(&src_file, &opts) {
            Some(result) => {
                let (dst_file, _base_name, _dir_name, ext) = result;
                if ext.to_lowercase() != "heic" {
                    // skip non-heic files
                    return 0;
                }
                let img = match read_image(src_file, &ext) {
                    Some(i) => i,
                    None => {
                        println!("Error reading image {}", opts.src_file);
                        return 0;
                    }
                };

                if save_image(&img, src_file, &dst_file) == 1 {
                    log!(" > Image converted succesfully");
                    if opts.overwrite == true {
                        match fs::remove_file(src_file) {
                            Ok(_) => {
                                log!(" > Original file removed");
                            }
                            Err(e) => {
                                println!("Error removing original file: {}", e);
                            }
                        }
                    }
                    return 1;
                }
                return 0;
            }
            None => {
                return 0;
            }
        }
    }

    pub fn auto_contrast(src_file: &str, opts: &ConfigOptions) -> u8 {
        match get_dest_name(&src_file, &opts) {
            Some(result) => {
                let (dst_file, _base_name, _dir_name, ext) = result;

                let img = match read_image(src_file, &ext) {
                    Some(i) => i,
                    None => {
                        println!("Error reading image {}", opts.src_file);
                        return 0;
                    }
                };
                // adjust the image contrast
                let adjusted_img;
                if rustyimg::is_grayscale_image(&img) {
                    log!(" > Image is grayscale");
                    adjusted_img = match rustyimg::auto_contrast_grayscale(&img) {
                        Some(i) => i,
                        None => {
                            log!(" > Image is at max contrast");
                            return 0;
                        }
                    };
                } else {
                    adjusted_img = match rustyimg::auto_contrast(&img) {
                        Some(i) => i,
                        None => {
                            log!(" > Image is at max contrast");
                            return 0;
                        }
                    };
                }
                // write to disk
                if save_image(&adjusted_img, src_file, &dst_file) == 1 {
                    log!(" > Adjusted contrast");
                    return 1;
                }
                return 0;
            }
            None => {
                return 0;
            }
        }
    }

    /**
     * Convert an image to grayscale and save it to a file
     */
    pub fn convert_to_grayscale(src_file: &str, opts: &ConfigOptions) -> u8 {
        match get_dest_name(&src_file, &opts) {
            Some(result) => {
                let (dst_file, _base_name, _dir_name, ext) = result;
                let img = match read_image(src_file, &ext) {
                    Some(i) => i,
                    None => {
                        println!("Error reading image {}", opts.src_file);
                        return 0;
                    }
                };

                if rustyimg::is_grayscale_image(&img) {
                    log!(" > Image is already grayscale");
                    return 0;
                }

                // detect if the image is color
                if !opts.force {
                    if rustyimg::is_color_image(&img) {
                        log!(" > Skipping color image");
                        return 0;
                    }
                }

                let grayscale_img = rustyimg::color_to_grayscale(&img);
                // write as <filename>_bw.<ext>
                if save_image(&grayscale_img, src_file, &dst_file) == 1 {
                    log!(" > Converted to grayscale");
                    return 1;
                }

                return 0;
            }
            None => {
                return 0;
            }
        }
    }

    // * EXIF functions //

    /**
     * Print the EXIF data of an image
     */
    pub fn print_exif_data(src_file: &str) -> u8 {
        let fields = rustyexif::read_exif_from_file(src_file);
        for field in fields {
            println!("{}: {}", field.name, field.value);
        }
        return 1;
    }

    /**
     * Set the EXIF Artist & Copyright tag of an image
     */

    pub fn set_artist_name(src_file: &str) -> u8 {
        let exif = rustyexif::read_exif_from_file(src_file);

        // get the exif date
        let tags = [
            "DateTimeOriginal",
            "DateTimeDigitized",
            "DateTime",
            "CreateDate",
            "SubSecCreateDate",
            "GPSDateTime",
        ];

        let mut year = String::from("");

        // check if exif contains any of the tags
        let mut is_tag_found = false;
        for tag in tags.iter() {
            let field = exif.iter().find(|&f| f.name == *tag);
            match field {
                Some(f) => {
                    year = f.value.clone()[0..4].to_string();
                    is_tag_found = true;
                    break;
                }
                None => {
                    continue;
                }
            }
        }

        if !is_tag_found {
            // get the file date instead
            let metadata = fs::metadata(src_file).unwrap();
            let modified = metadata.modified().unwrap();
            let date = chrono::DateTime::<chrono::Local>::from(modified);
            year = date.format("%Y").to_string();
        }

        let artist = config::option("artist", "");
        if artist == "" {
            panic!("No artist specified.");
        }

        debug!("Artist passed: {}", artist);
        let copyright = format!("© {} {}", year, artist);
        let mut fields: Vec<ExifField> = Vec::new();

        fields.push(ExifField {
            name: "Artist".to_string(),
            value: artist.clone(),
        });
        fields.push(ExifField {
            name: "Copyright".to_string(),
            value: copyright.clone(),
        });

        return rustyexif::write_exif_to_file(src_file, fields);
    }

    /**
     * Set the EXIF date of an image
     */
    pub fn set_exif_date(src_file: &str) -> u8 {
        let sdate = config::option("date", "");
        debug!("Date passed: {}", sdate);
        if sdate == "" {
            panic!("No date specified");
        }

        // let exif = rustyexif::read_exif_from_file(src_file);

        let date = format!("{} 12:00:00", sdate.replace("-", ":"));

        let mut fields: Vec<ExifField> = Vec::new();

        let tags = [
            "DateTimeOriginal",
            "DateTimeDigitized",
            "DateTime",
            "CreateDate",
            "ModifyDate",
            "SubSecCreateDate",
            "GPSDateTime",
        ];

        for tag in tags.iter() {
            // if exif.iter().find(|&f| f.name == *tag).is_none() {
            //     continue;
            // }
            fields.push(ExifField {
                name: tag.to_string(),
                value: date.clone(),
            });
        }

        rustyexif::write_exif_to_file(src_file, fields);

        let mtime = FileTime::from_unix_time(
            chrono::NaiveDateTime::parse_from_str(&date, "%Y:%m:%d %H:%M:%S")
                .unwrap()
                .timestamp(),
            0,
        );

        // set the modified date too
        match set_file_mtime(src_file, mtime) {
            Ok(_) => {
                log!(" Date set succesfully to {}", date);
                return 1;
            }
            Err(e) => {
                println!("Error setting file modified date: {}", e);
                return 0;
            }
        }
    }

    pub fn rename_jpeg_file(src_file: &str) -> u8 {
        let base_name = Path::new(&src_file).file_stem().unwrap().to_str().unwrap();
        let dir_name = Path::new(&src_file).parent().unwrap().to_str().unwrap();
        let dst_file = format!("{}/{}.{}", dir_name, base_name, "jpg");
        // log the action
        log!("Renaming {} to {}", src_file, dst_file);

        // rename the file
        rename_file(src_file, &dst_file);

        return 1;
    }

    pub fn read_image(src_file: &str, ext: &str) -> Option<Image<u8>> {
        if ext.to_lowercase() == "heic" {
            let ctx = match HeifContext::read_from_file(src_file) {
                Ok(c) => c,
                Err(e) => {
                    println!("Error reading heic file: {}", e);
                    return None;
                }
            };
            let handle = ctx.primary_image_handle().unwrap();
            match handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), None) {
                Ok(heif_img) => {
                    let w = heif_img.width(Channel::Interleaved).unwrap();
                    let h = heif_img.height(Channel::Interleaved).unwrap();
                    let mut image = Image::blank(ImageInfo::new(w, h, 3, false));
                    let planes = heif_img.planes();
                    let interleaved_plane = planes.interleaved.unwrap();
                    let stride = interleaved_plane.stride;
                    let data = interleaved_plane.data;

                    // copy data to image
                    for y in 0..h {
                        for x in 0..w {
                            let offset = (y * stride as u32 + x * 3) as usize;
                            let pixel = &[data[offset], data[offset + 1], data[offset + 2]];
                            image.set_pixel(x, y, pixel);
                        }
                    }

                    return Some(image);
                }
                Err(_e) => None,
            }
        } else {
            match io::read(src_file) {
                Ok(i) => Some(i),
                Err(_e) => None,
            }
        }
    }

    pub fn save_image(img: &Image<u8>, src_file: &str, dst_file: &str) -> u8 {
        let exif_fields = rustyexif::read_exif_from_file(src_file);
        let target = dst_file.replace(".heic", ".jpg");
        // write as <filename>_bw.<ext>
        match io::write(&img, &target) {
            Ok(_) => {
                // copy exif data
                return rustyexif::write_exif_to_file(&target, exif_fields);
            }
            Err(e) => {
                println!("Error writing image {}: {:?}", target, e);
                return 0;
            }
        }
    }

    /**
     * Returns the extension of a file name
     */
    pub fn get_filename_extension(filename: &str) -> Option<&str> {
        Path::new(filename).extension().and_then(OsStr::to_str)
    }

    pub fn get_files_in_folder(folder: &str, filter: Vec<String>) -> Vec<String> {
        let mut files = Vec::new();
        for entry in fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_path = path.to_str().unwrap().to_string();
                let ext = get_filename_extension(&file_path).unwrap_or("").to_string();
                if filter.contains(&ext.to_lowercase()) {
                    files.push(file_path);
                }
            }
        }
        files
    }

    pub fn rename_file(src_file: &str, dst_file: &str) {
        fs::rename(src_file, dst_file).unwrap();
    }

    // pub fn  remove_file(file: &str) {
    //     fs::remove_file(file).unwrap();
    // }

    pub fn get_dest_name(
        src_file: &str,
        opts: &ConfigOptions,
    ) -> Option<(String, String, String, String)> {
        let base_name = Path::new(&src_file).file_stem().unwrap().to_str().unwrap();
        let dir_name = Path::new(&src_file).parent().unwrap().to_str().unwrap();
        let ext = get_filename_extension(&src_file).unwrap_or("");

        let mut dst_file = String::from("");
        if !dir_name.is_empty() {
            dst_file.push_str(dir_name);
            dst_file.push_str("/");
        }

        dst_file.push_str(base_name);
        if !opts.suffix.is_empty() {
            dst_file.push_str("_");
            dst_file.push_str(&opts.suffix);
        }
        dst_file.push_str(".");

        // cannot save to HEIC format yet
        if ext == "heic" {
            dst_file.push_str("jpg");
        } else {
            dst_file.push_str(ext);
        }

        if src_file == dst_file && !opts.overwrite {
            log!(" > Skipping file {} ", src_file);
            return None;
        }

        Some((
            dst_file.to_string(),
            base_name.to_string(),
            dir_name.to_string(),
            ext.to_string(),
        ))
    }
}

pub use imageaction::*;
