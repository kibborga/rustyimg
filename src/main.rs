#[macro_use]
extern crate lazy_static;


pub mod config;
pub mod rustyimg;
pub mod imageaction;
pub mod rustyexif;

use imageaction::*;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        if config::option("verbose", "false") == "true" || config::option("debug", "false") == "true" {
            println!($($arg)*);
        }
    })
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        if config::option("debug", "false") == "true" {
            println!($($arg)*);
        }
    })
}

fn main() {
    // parse command line arguments and get the options
    let opts = config::get();

    if opts.error == true {
        return;
    }

    let ext = get_filename_extension(&opts.src_file);
    let mut count = 0;
    let filter: Vec<String>;

    if opts.action == "convert-heic" {
        // remove jpeg files from the filter
        filter = Vec::from([String::from("heic")]);
    } else {
        filter = Vec::from([
            String::from("jpg"),
            String::from("jpeg"),
            String::from("heic"),
        ]);
    }

    if ext.is_none() {
        let files = get_files_in_folder(&opts.src_file, filter);
        let total = files.len();
        let mut current = 0;
        println!("Processing {} files...", total);
        for file in files {
            log!("Processing {} ({} to go)", file, total - current);
            count = count + process_image(&file, &opts);
            current = current + 1;
        }
    } else {
        count = count + process_image(&opts.src_file, &opts);
    }
    println!("=====================");
    println!("Processed {} images", count);
}

/**
 * Processes a single image according to the action
 */
fn process_image(src_file: &String, opts: &ConfigOptions) -> u8 {
    // skip non-jpeg files
    let ext = get_filename_extension(&src_file).unwrap_or("");
    if ext.to_lowercase() != "jpg" && ext.to_lowercase() != "jpeg" && ext.to_lowercase() != "heic" {
        return 0;
    }

    let mut result: u8 = 0;
    // execute the requested action
    if opts.action == "grayscale" {
        result = convert_to_grayscale(&src_file, &opts);
    } else if opts.action == "fix-jpeg-ext" && ext.to_lowercase() != "jpeg" {
        result = rename_jpeg_file(&src_file);
    } else if opts.action == "auto-contrast" {
        result = auto_contrast(&src_file, &opts);
    } else if opts.action == "print-exif" {
        result = print_exif_data(&src_file);
    } else if opts.action == "convert-heic" {
        result = convert_heic(&src_file, &opts);
    } else if opts.action == "set-date" {
        result = set_exif_date(&src_file);
    } else if opts.action == "set-artist" {
        result = set_artist_name(&src_file);
    } else {
        panic!("Unknown action {}", opts.action);
    }
    return result;
}
