use imgproc_rs::image::{BaseImage, Image, ImageInfo};
use imgproc_rs::io;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() {
    println!("RustyBW v.0.1.0");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rustybw <filename> [<suffix>|overwrite] [verbose]");
        return;
    }

    let src_file = &args[1];
    let suffix = if args.len() > 2 { &args[2] } else { "bw" };
    let verbose = if args.len() > 3 { &args[3] } else { "" };

    let ext = get_filename_extension(src_file);
    let mut count = 0;
    if ext.is_none() {

        let files = get_files_in_folder(src_file);

        if verbose == "verbose" {
            println!("Found {} files", files.len());
        }

        for file in files {
            count = count + convert_image(&file, suffix, verbose);
        }

    } else {
        count = count + convert_image(src_file, suffix, verbose);
    }
    println!("Converted {} images", count);

}

fn get_filename_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn get_files_in_folder(folder: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(folder).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    files
}

fn convert_image(src_file: &str, suffix: &str, verbose: &str) -> u8 {
    // get the file name, directory and extension
    let base_name = Path::new(src_file).file_stem().unwrap();
    let dir_name = Path::new(src_file).parent().unwrap();
    let ext = get_filename_extension(src_file);
    if ext != Some("jpg") && ext != Some("jpeg") {
        if verbose == "verbose" {
            println!(" > Skipping file {}", base_name.to_str().unwrap());
        }
        return 0;
    }

    // read the image
    let img = io::read(src_file).unwrap();
    let (width, height) = img.info().wh();
    if verbose == "verbose" {
        println!("Read {}, size: {}x{}", base_name.to_str().unwrap(), width, height);
    }
    // check if the image is grayscale
    let info = img.info();
    if info.channels == 1 {
        if verbose == "verbose" {
            println!(" > Image is already grayscale");
        }
        return 0;
    }

    // detect if the image is color
    let mut is_color = false;
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let (r1, g1, b1) = (pixel[0], pixel[1], pixel[2]);
            if r1 != g1 || r1 != b1 {
                is_color = true;
                break;
            }
        }
    }
    if is_color && suffix == "overwrite" {
        if verbose == "verbose" {
            println!(" > Skipping color image");
        }
        return 0;
    }

    // copy image data to new image
    let mut img2 = Image::blank(ImageInfo::new(width, height, 1, false));
    // convert to grayscale
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let (r1, g1, b1) = (pixel[0], pixel[1], pixel[2]);
            let r = r1 as f32 * 0.299 as f32;
            let g = b1 as f32 * 0.587 as f32;
            let b = g1 as f32 * 0.114 as f32;
            let gray32 = r + g + b;
            let gray = gray32 as u8;
            img2.set_pixel(x, y, &[gray]);
        }
    }

    // write as <filename>_bw.<ext>
    let dst_file;
    if suffix == "overwrite" {
        dst_file = src_file.to_string();
    } else {
        dst_file = format!("{}/{}_{}.{}", dir_name.to_str().unwrap(), base_name.to_str().unwrap(), suffix, ext.unwrap());
    }

    io::write(&img2, &dst_file).expect("Failed to write image");
    println!(" > Saved as {}", dst_file);
    return 1;
}
