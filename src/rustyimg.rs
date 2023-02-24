pub mod rustyimg {


    use imgproc_rs::image::{BaseImage, Image, ImageInfo};

    pub use crate::config::*;
    use crate::{debug, log};

    /**
     * Convert 8-bit RGB image to 8-bit grayscale image
     */
    pub fn color_to_grayscale(img: &Image<u8>) -> Image<u8> {
        let (width, height) = img.info().wh();

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

        return img2;
    }

    pub fn auto_contrast_grayscale(img: &Image<u8>) -> Option<Image<u8>> {
        log!("Applying linear contrast to image");
        let gray_channel = get_channel(img, 0);

        let min_gray = *gray_channel.iter().min().unwrap();
        let max_gray = *gray_channel.iter().max().unwrap();

        let range_gray = 255.0/(max_gray - min_gray) as f32;

        debug!(" > Gray range: {} - {}", min_gray, max_gray);

        //let gray_scale = color_to_grayscale(img);

        let (width, height, _channels) = img.info().whc();
        let mut img2 = img.clone();

        for x in 0..width {
            for y in 0..height {
                let pixel = img.get_pixel(x, y);
                // iO      = (iI-minI)*(((maxO-minO)/(maxI-minI))+minO)
                let gray = (pixel[0] - min_gray) as f32 * range_gray + min_gray as f32;
                // convert back to rgb
                img2.set_pixel(x, y, &[gray as u8]);
            }
        }

        return Some(img2);

    }

    pub fn auto_contrast(img: &Image<u8>) -> Option<Image<u8>> {
        log!("Applying linear contrast to image");
        let red_channel = get_channel(img, 0);
        let green_channel = get_channel(img, 1);
        let blue_channel = get_channel(img, 2);

        let min_red = *red_channel.iter().min().unwrap();
        let max_red = *red_channel.iter().max().unwrap();

        let min_green = *green_channel.iter().min().unwrap();
        let max_green = *green_channel.iter().max().unwrap();

        let min_blue = *blue_channel.iter().min().unwrap();
        let max_blue = *blue_channel.iter().max().unwrap();

        let range_red = 255.0/(max_red - min_red) as f32;
        let range_green = 255.0/(max_green - min_green) as f32;
        let range_blue = 255.0/(max_blue - min_blue) as f32;

        debug!(" > R range: {} - {}", min_red, max_red);
        debug!(" > G range: {} - {}", min_green, max_green);
        debug!(" > B range: {} - {}", min_blue, max_blue);

        //let gray_scale = color_to_grayscale(img);

        let (width, height, _channels) = img.info().whc();
        let mut img2 = img.clone();

        for x in 0..width {
            for y in 0..height {
                let pixel = img.get_pixel(x, y);
                // iO      = (iI-minI)*(((maxO-minO)/(maxI-minI))+minO)
                let r = (pixel[0] - min_red) as f32 * range_red + min_red as f32;
                let g = (pixel[1] - min_green) as f32 * range_green + min_green as f32;
                let b = (pixel[2] - min_blue) as f32 * range_blue + min_blue as f32;
                // convert back to rgb
                img2.set_pixel(x, y, &[r as u8, g as u8, b as u8]);
            }
        }
        return Some(img2);
    }

    // gets a single channel from an image
    pub fn get_channel(img: &Image<u8>, channel: usize) -> Vec<u8> {
        let (width, height) = img.info().wh();
        let mut channel_data = Vec::new();
        for x in 0..width {
            for y in 0..height {
                let pixel = img.get_pixel(x, y);
                channel_data.push(pixel[channel]);
            }
        }
        return channel_data;
    }


    // check if the image is grayscale
    pub fn is_grayscale_image(img: &Image<u8>) -> bool {
        let info = img.info();
        return info.channels == 1;
    }

    // check if the image is not grayscale
    pub fn is_color_image(img: &Image<u8>) -> bool {
        let (width, height) = img.info().wh();
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
        return is_color;
    }


}

pub use rustyimg::*;
