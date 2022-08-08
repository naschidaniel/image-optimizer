use crate::image_optimizer::ImageOptimizer;
use glob::{glob_with, MatchOptions};

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ConvertImage {
    pub dest_file: PathBuf,
    pub dest_parent: PathBuf,
    pub dest_thumbnail: PathBuf,
    pub file_name_src: String,
    pub file_name_thumbnail: String,
    pub file_name_thumbnail_webp: String,
    pub file_name: String,
    pub file_name_webp: String,
    pub file_type: String,
    pub suffix: String,
    pub src_file: PathBuf,
}

impl ConvertImage {
    /// The necessary file structure is created and the modified file name is returned as `PathBuf`.
    fn new(
        file_name_src: &PathBuf,
        source_path: &PathBuf,
        destination: &PathBuf,
        suffix: &String,
    ) -> ConvertImage {
        let source_parent_path = match source_path.is_file() {
            true => fs::canonicalize(file_name_src.parent().unwrap()).unwrap(),
            false => fs::canonicalize(&source_path).unwrap(),
        };

        let file_name_src_absolute = fs::canonicalize(file_name_src).unwrap();
        let source_path_sub_folders = file_name_src_absolute
            .parent()
            .unwrap()
            .strip_prefix(source_parent_path)
            .unwrap();

        let dest_path = Path::new(destination).join(source_path_sub_folders);
        fs::create_dir_all(&dest_path).unwrap();

        let file_stem = file_name_src.file_stem().unwrap().to_str().unwrap();
        let file_extension = file_name_src.extension().unwrap().to_str().unwrap();

        let file_name = format!("{}_{}.{}", &file_stem, suffix, file_extension);
        let file_name_thumbnail = format!("{}_thumbnail_{}.{}", &file_stem, suffix, file_extension);

        Self {
            dest_file: dest_path.join(&file_name),
            dest_parent: dest_path.to_owned(),
            dest_thumbnail: dest_path.join(file_name_thumbnail),
            file_name,
            file_name_src: format!("{}.{}", file_stem, file_extension),
            file_name_thumbnail: format!("{}_thumbnail_{}.{}", file_stem, suffix, file_extension),
            file_name_thumbnail_webp: format!("{}_thumbnail_{}.webp", file_stem, suffix),
            file_name_webp: format!("{}_{}.webp", file_stem, suffix),
            file_type: file_extension.to_lowercase(),
            suffix: suffix.to_string(),
            src_file: file_name_src_absolute,
        }
    }

    pub fn resize_one(&self, nwidth: &u32, nquality: &u8, webpimage: &bool, thumbnail: &bool) {
        let original_image = image::open(&self.src_file).expect("Opening image original failed");
        let optimized_image = match thumbnail {
            true => ImageOptimizer::new_thumbnail(
                original_image.to_owned(),
                self.dest_thumbnail.to_owned(),
                *nwidth,
                *nquality,
            ),
            false => ImageOptimizer::new(
                original_image.to_owned(),
                self.dest_file.to_owned(),
                *nwidth,
                *nquality,
            ),
        };
        println!(
            "Converting {:?} (w: {:?}, h: {:?}) to {:?} (w: {:?}, h: {:?})",
            self.file_name_src,
            original_image.width(),
            original_image.height(),
            optimized_image.nfilename,
            optimized_image.nwidth,
            optimized_image.nheight,
        );

        if webpimage == &true {
            optimized_image.save_webp_image();
        }

        if optimized_image.extension().to_lowercase() == "jpg"
            || optimized_image.extension().to_lowercase() == "jpeg"
        {
            optimized_image.save_jpg_image().unwrap();
        } else {
            optimized_image.save_png_image().unwrap();
        }
    }
}

pub struct ResizeImage {
    pub source: String,
    pub source_path: PathBuf,
    pub destination: String,
    pub destination_path: PathBuf,
    pub suffix: String,
    pub width: u32,
    pub quality: u8,
    pub webpimage: bool,
    pub thumbnail: bool,
}

impl ResizeImage {
    pub fn new(
        source: String,
        destination: String,
        suffix: String,
        width: u32,
        quality: u8,
        webpimage: bool,
        thumbnail: bool,
    ) -> ResizeImage {
        Self {
            source_path: fs::canonicalize(&source).unwrap(),
            source,
            destination_path: fs::canonicalize(&destination).unwrap(),
            destination,
            suffix,
            width,
            quality,
            webpimage,
            thumbnail,
        }
    }
    pub fn run_resize_images(&self) {
        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        let entries = match Path::new(&self.source).is_file() {
            true => glob_with(&self.source, options)
                .unwrap()
                .flatten()
                .collect::<Vec<PathBuf>>(),
            false => {
                let pattern_jpg = format!("{}/**/*.jpg", &self.source);
                let pattern_jpeg = format!("{}/**/*.jpeg", &self.source);
                let pattern_png = format!("{}/**/*.png", &self.source);
                glob_with(&pattern_jpg, options)
                    .unwrap()
                    .chain(glob_with(&pattern_jpeg, options).unwrap())
                    .chain(glob_with(&pattern_png, options).unwrap())
                    .flatten()
                    .collect::<Vec<PathBuf>>()
            }
        };
        println!("------------------------------------------");
        println!("{} images will be optimized.", entries.len());
        println!("------------------------------------------");

        for file_name_src in entries {
            let image = ConvertImage::new(
                &file_name_src,
                &self.source_path,
                &self.destination_path,
                &self.suffix,
            );
            image.resize_one(&self.width, &self.quality, &self.webpimage, &false);
            if self.thumbnail {
                image.resize_one(&self.width, &self.quality, &self.webpimage, &self.thumbnail);
            }
            println!("The file '{:?}' has been converted!", file_name_src);
            println!("------------------------------------------");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;

    use tempfile::tempdir;

    // Determine operating system
    const PLATFORM: &str = match cfg!(windows) {
        true => "windows",
        false => "unix",
    };

    /// The test checks if the metadata can be generated.
    #[test]
    fn test_new() {
        let tempdir = tempdir().unwrap().into_path();
        let file_name_src = tempdir.join("spool/foo/bar/baz.JPG");
        fs::create_dir_all(&file_name_src.parent().unwrap()).unwrap();
        fs::copy("media/paradise/fly.JPG", &file_name_src).unwrap();
        let source = tempdir.join("spool");
        let destination = tempdir.join("moon");
        let image = ConvertImage::new(&file_name_src, &source, &destination, &String::from("sm"));
        assert_eq!(tempdir.join("moon/foo/bar/baz_sm.JPG"), image.dest_file);
        assert_eq!(tempdir.join("moon/foo/bar"), image.dest_parent);
        assert_eq!(
            tempdir.join("moon/foo/bar/baz_thumbnail_sm.JPG"),
            image.dest_thumbnail
        );
        assert_eq!("baz.JPG", image.file_name_src);
        assert_eq!("baz_sm.JPG", image.file_name);
        assert_eq!("baz_sm.webp", image.file_name_webp);
        assert_eq!("baz_thumbnail_sm.JPG", image.file_name_thumbnail);
        assert_eq!("baz_thumbnail_sm.webp", image.file_name_thumbnail_webp);
        assert_eq!("jpg", image.file_type);
        assert_eq!("sm", image.suffix);
        assert_eq!(
            tempdir
                .join("spool/foo/bar/baz.JPG")
                .canonicalize()
                .unwrap(),
            image.src_file
        );
        remove_dir_all(tempdir).unwrap();
    }

    /// The test checks if multiple files in multiple sub folders can be optimized.
    #[test]
    fn test_resize_images_in_folder() {
        let tempdir = String::from(tempdir().unwrap().into_path().to_str().unwrap());

        ResizeImage::new(
            String::from("media"),
            tempdir.clone(),
            String::from("sm"),
            500,
            90,
            true,
            true,
        )
        .run_resize_images();

        let mut temp_img_jpg_path = tempdir.to_owned();
        temp_img_jpg_path.push_str("/paradise/fly_sm.JPG");
        let mut temp_img_jpg_webp_path = tempdir.to_owned();
        temp_img_jpg_webp_path.push_str("/paradise/fly_sm.webp");
        let mut temp_img_png_webp_path = tempdir.to_owned();
        temp_img_png_webp_path.push_str("/paradise/paragliding_sm.webp");

        let temp_img_jpg = image::open(temp_img_jpg_path).unwrap();
        let temp_img_jpg_webp = image::open(temp_img_jpg_webp_path).unwrap();
        let temp_img_png_webp = image::open(temp_img_png_webp_path).unwrap();

        let mut temp_img_jpg_thumbnail_path = tempdir.to_owned();
        temp_img_jpg_thumbnail_path.push_str("/paradise/fly_thumbnail_sm.JPG");
        let mut temp_img_jpg_webp_thumbnail_path = tempdir.to_owned();
        temp_img_jpg_webp_thumbnail_path.push_str("/paradise/fly_thumbnail_sm.webp");
        let mut temp_img_png_webp_thumbnail_path = tempdir.to_owned();
        temp_img_png_webp_thumbnail_path.push_str("/paradise/paragliding_thumbnail_sm.webp");

        let temp_img_jpg_thumbnail = image::open(temp_img_jpg_thumbnail_path).unwrap();
        let temp_img_jpg_webp_thumbnail = image::open(temp_img_jpg_webp_thumbnail_path).unwrap();
        let temp_img_png_webp_thumbnail = image::open(temp_img_png_webp_thumbnail_path).unwrap();

        // valid testdata
        let img_jpg_ok = image::open(format!("./testdata/fly_sm.{PLATFORM}.JPG")).unwrap();
        let img_jpg_webp_ok = image::open(format!("./testdata/fly_sm.{PLATFORM}.webp")).unwrap();
        let img_png_webp_ok =
            image::open(format!("./testdata/paragliding_sm.{PLATFORM}.webp")).unwrap();

        assert_eq!(img_jpg_ok, temp_img_jpg);
        assert_eq!(img_jpg_webp_ok, temp_img_jpg_webp);
        assert_eq!(img_png_webp_ok, temp_img_png_webp);

        // valid testdata thumbnails
        let img_jpg_thumbnail_ok =
            image::open(format!("./testdata/fly_thumbnail_sm.{PLATFORM}.JPG")).unwrap();
        let img_jpg_webp_thumbnail_ok =
            image::open(format!("./testdata/fly_thumbnail_sm.{PLATFORM}.webp")).unwrap();
        let img_png_webp_thumbnail_ok = image::open(format!(
            "./testdata/paragliding_thumbnail_sm.{PLATFORM}.webp"
        ))
        .unwrap();

        assert_eq!(img_jpg_thumbnail_ok, temp_img_jpg_thumbnail);
        assert_eq!(img_jpg_webp_thumbnail_ok, temp_img_jpg_webp_thumbnail);
        assert_eq!(img_png_webp_thumbnail_ok, temp_img_png_webp_thumbnail);
        remove_dir_all(tempdir).unwrap();
    }

    /// The test checks if one file can be optimized.
    #[test]
    fn test_resize_one_file() {
        let tempdir = tempdir().unwrap().into_path().to_str().unwrap().to_string();

        ResizeImage::new(
            String::from("media/paradise/fly.JPG"),
            tempdir.clone(),
            String::from("xxs"),
            250,
            90,
            true,
            false,
        )
        .run_resize_images();

        let mut temp_img_jpg_webp_path = tempdir.to_owned();
        temp_img_jpg_webp_path.push_str("/fly_xxs.webp");

        let temp_img_jpg_webp = image::open(temp_img_jpg_webp_path).unwrap();

        let img_jpg_webp_ok = image::open(format!("./testdata/fly_xxs.{PLATFORM}.webp")).unwrap();

        assert_eq!(img_jpg_webp_ok, temp_img_jpg_webp);

        remove_dir_all(tempdir).unwrap();
    }
}
