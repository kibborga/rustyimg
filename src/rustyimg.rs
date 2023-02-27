pub mod rustyimg {

    use imgproc_rs::image::{BaseImage, Image, ImageInfo};
    // use imgproc_rs::enums::White;

    pub use crate::config::*;
    // use crate::{debug, log};

    pub fn process_image(img: &Image<u8>, opts: &ConfigOptions) -> Image<u8> {
        let (width, height, channels) = img.info().whc();
        let mut img2;

        let is_grayscale = channels == 1;
        // should we convert to grayscale?
        let to_gray = opts.grayscale && !is_grayscale && opts.force;

        // get the min and max values for each channel
        let mut min = [0, 0, 0];
        let mut max = [255, 255, 255];
        if opts.autocontrast {
            for index in 0..channels {
                let i = index as usize;
                let (min1, max1) = get_channel_ranges(img, i);
                min[i] = min1;
                max[i] = max1;
            }
        }

        img2 = Image::blank(ImageInfo::new(
            width,
            height,
            if to_gray { 1 } else { channels },
            false,
        ));

        for x in 0..width {
            for y in 0..height {
                let mut xi = x;
                let mut yi = y;
                // flip image horizontally
                if opts.fliph {
                    xi = width - x - 1;
                }
                // flip image vertically
                if opts.flipv {
                    yi = height - y - 1;
                }

                let pixel = img.get_pixel(x, y);
                let n = pixel.len();
                let mut tpixel: [u8; 3] = [0, 0, 0];
                for i in 0..n {
                    let mut value = pixel[i];
                    if opts.autocontrast {
                        // ((pixel – min) / (max – min))*255 [+ min?]
                        let adjusted: f32 =
                            255.0 * (value - min[i]) as f32 / (max[i] - min[i]) as f32;
                        if adjusted > 255.0 {
                            value = 255;
                        } else if adjusted < 0.0 {
                            value = 0;
                        } else {
                            value = adjusted as u8;
                        }
                    }

                    if n == 1 {
                        if opts.invert {
                            value = 255 - value;
                        }
                        img2.set_pixel(xi, yi, &[value]);
                    } else {
                        if to_gray {
                            let (r1, g1, b1) = (pixel[0], pixel[1], pixel[2]);
                            let r = r1 as f32 * 0.299 as f32;
                            let g = b1 as f32 * 0.587 as f32;
                            let b = g1 as f32 * 0.114 as f32;
                            value = if opts.invert {
                                255 - (r + g + b) as u8
                            } else {
                                (r + g + b) as u8
                            };
                            img2.set_pixel(xi, yi, &[value]);
                        } else {
                            tpixel[i] = value;
                        }
                    }
                }
                if !to_gray {
                    img2.set_pixel(xi, yi, &tpixel);
                }
            }
        }
        return img2;
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

    fn get_channel_ranges(img: &Image<u8>, channel: usize) -> (u8, u8) {
        let channel_data = get_channel(img, channel);
        let min = channel_data.iter().min().unwrap();
        let max = channel_data.iter().max().unwrap();
        return (*min, *max);
    }

    pub fn is_grayscale_image(img: &Image<u8>) -> bool {
        let (_width, _height, channels) = img.info().whc();
        return channels == 1;
    }

    // check if the image is not grayscale
    pub fn is_color_grayscale(img: &Image<u8>) -> bool {
        let pixels = img.data();
        let prc1 = pixels.len() as f32 / 200.0 as f32;
        let mut cnt = prc1 as u8;
        for index in 0..pixels.len() - 3 {
            let i = index * 3;
            // println!("{}: {} {} {}", i, pixels[i], pixels[i + 1], pixels[i + 2]);
            if pixels[i] != pixels[i + 1]
                || pixels[i] != pixels[i + 2]
                || pixels[i + 1] != pixels[i + 2]
            {
                return false;
            }

            cnt = cnt - 1;
            if cnt == 0 {
                break;
            }
        }
        return true;
    }
}

pub use rustyimg::*;
