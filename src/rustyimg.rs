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

        img2 = Image::blank(ImageInfo::new(width, height, channels, false));

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

                    if opts.invert {
                        value = 255 - value;
                    }

                    if n == 1 {
                        img2.set_pixel(xi, yi, &[value]);
                    } else {
                        if to_gray {
                            let (r1, g1, b1) = (pixel[0], pixel[1], pixel[2]);
                            let r = r1 as f32 * 0.299 as f32;
                            let g = b1 as f32 * 0.587 as f32;
                            let b = g1 as f32 * 0.114 as f32;
                            let gray32 = r + g + b;
                            value = if opts.invert {
                                255 - gray32 as u8
                            } else {
                                gray32 as u8
                            };
                        }

                        tpixel[i] = value;
                    }
                }
                if n > 1 {
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
