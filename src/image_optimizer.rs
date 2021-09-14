use image::codecs::png;
use image::imageops::FilterType;
use image::jpeg::JpegEncoder;
use image::png::PngEncoder;
use image::DynamicImage;
use image::GenericImageView;
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
    pub resize_ratio: f32,
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
            resize_ratio,
        }
    }
    fn extension(&self) -> &str {
        self.nfilename.extension().unwrap().to_str().unwrap()
    }
    pub fn save_jpg_image(&self) -> Result<(), ImageError> {
        let file = File::create(&self.nfilename).unwrap();
        let ref mut buffer = BufWriter::new(file);
        JpegEncoder::new_with_quality(buffer, self.nquality).encode(
            &self.nimage.to_bytes(),
            self.nwidth,
            self.nheight,
            self.nimage.color(),
        )
    }
    pub fn save_png_image(&self) -> Result<(), ImageError> {
        let file = File::create(&self.nfilename).unwrap();
        let ref mut buffer = BufWriter::new(file);
        PngEncoder::new_with_quality(
            buffer,
            png::CompressionType::Default,
            png::FilterType::NoFilter,
        )
        .encode(
            &self.nimage.to_bytes(),
            self.nwidth,
            self.nheight,
            self.nimage.color(),
        )
    }
    pub fn save_webp_image(&self) -> () {
        let file = &self
            .nfilename
            .to_str()
            .to_owned()
            .unwrap()
            .replace(&self.extension(), "webp");
        let filename = Path::new(file);
        println!(
            "Creating WebP image {:?} (w: {:?}, h: {:?}), resize ratio: {:?}, quality: {:?}",
            filename, self.nwidth, self.nheight, self.resize_ratio, self.nquality
        );
        let mut buffer = File::create(filename).unwrap();
        let webp_image = Encoder::new(
            &self.nimage.to_bytes(),
            PixelLayout::Rgb,
            self.nwidth,
            self.nheight,
        )
        .encode(self.nquality as f32);
        buffer.write(&*webp_image).unwrap();
    }
}
