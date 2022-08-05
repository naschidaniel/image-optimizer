use image::codecs::jpeg::JpegEncoder;
use image::codecs::png;
use image::codecs::png::PngEncoder;
use image::imageops::FilterType;
use image::DynamicImage;
use image::ImageEncoder;
use image::ImageError;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use webp::{Encoder, PixelLayout};

pub struct ImageOptimizer {
    pub nimage: DynamicImage,
    pub nfilename: PathBuf,
    pub nwidth: u32,
    pub nheight: u32,
    pub nquality: u8,
}

impl ImageOptimizer {
    pub fn new(
        original_image: DynamicImage,
        nfilename: PathBuf,
        nwidth: u32,
        nquality: u8,
    ) -> ImageOptimizer {
        let resize_ratio = original_image.width() as f32 / nwidth as f32;
        let nheight = (original_image.height() as f32 / resize_ratio) as u32;
        let nimage = original_image.resize_exact(nwidth, nheight, FilterType::Lanczos3);
        ImageOptimizer {
            nimage,
            nfilename,
            nwidth,
            nheight,
            nquality,
        }
    }
    pub fn new_thumbnail(
        original_image: DynamicImage,
        nfilename: PathBuf,
        nwidth: u32,
        nquality: u8,
    ) -> ImageOptimizer {
        let nheight = nwidth;
        let nimage = original_image.resize_to_fill(nwidth, nheight, FilterType::Lanczos3);
        ImageOptimizer {
            nimage,
            nfilename,
            nwidth,
            nheight,
            nquality,
        }
    }
    pub fn extension(&self) -> &str {
        self.nfilename.extension().unwrap().to_str().unwrap()
    }
    pub fn save_jpg_image(&self) -> Result<(), ImageError> {
        let file = File::create(&self.nfilename).unwrap();
        let buffer = &mut BufWriter::new(file);
        JpegEncoder::new_with_quality(buffer, self.nquality).encode(
            self.nimage.as_bytes(),
            self.nwidth,
            self.nheight,
            self.nimage.color(),
        )
    }
    pub fn save_png_image(&self) -> Result<(), ImageError> {
        let file = File::create(&self.nfilename).unwrap();
        let buffer = &mut BufWriter::new(file);
        PngEncoder::new_with_quality(
            buffer,
            png::CompressionType::Default,
            png::FilterType::NoFilter,
        )
        .write_image(
            self.nimage.as_bytes(),
            self.nwidth,
            self.nheight,
            self.nimage.color(),
        )
    }
    pub fn save_webp_image(&self) {
        let file = &self
            .nfilename
            .to_str()
            .to_owned()
            .unwrap()
            .replace(&self.extension(), "webp");
        let filename = Path::new(file);
        println!(
            "Creating WebP image {:?} (w: {:?}, h: {:?}), quality: {:?}",
            filename, self.nwidth, self.nheight, self.nquality
        );
        let mut buffer = File::create(filename).unwrap();
        let webp_image = Encoder::new(
            self.nimage.as_bytes(),
            PixelLayout::Rgb,
            self.nwidth,
            self.nheight,
        )
        .encode(self.nquality as f32);
        buffer.write(&*webp_image).unwrap();
    }
}
