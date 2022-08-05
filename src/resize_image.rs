use crate::image_optimizer::ImageOptimizer;
use glob::{glob_with, MatchOptions};
use image::ImageError;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ResizeImage {
    pub dest_file: PathBuf,
    pub dest_parent: PathBuf,
    pub dest_thumbnail: PathBuf,
    pub file_name_src: String,
    pub file_name_thumbnail_webp: String,
    pub file_name_thumbnail: String,
    pub file_name_webp: String,
    pub file_name: String,
    pub file_type: String,
    pub suffix: String,
}

impl ResizeImage {
    /// The necessary file structure is created and the modified file name is returned as `PathBuf`.
    fn new(
        file_name_src: &PathBuf,
        source_path: &PathBuf,
        destination: &PathBuf,
        suffix: &String,
    ) -> ResizeImage {
        let source_parent_path = match source_path.is_file() {
            true => fs::canonicalize(file_name_src.parent().unwrap()).unwrap(),
            false => fs::canonicalize(&source_path).unwrap(),
        };

        let rudi = fs::canonicalize(file_name_src).unwrap();
        println!("source:             {:?}", source_path);
        println!("source_parent_path: {:?}", source_parent_path);
        println!("destination:        {:?}", destination);
        println!("file_name_src:      {:?}", rudi);
        println!("-------");
        let source_path_sub_folders = rudi
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

        ResizeImage {
            dest_file: dest_path.join(&file_name),
            dest_parent: dest_path.to_owned(),
            dest_thumbnail: dest_path.join(file_name_thumbnail),
            file_name,
            file_name_src: file_name_src
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            file_name_thumbnail: format!("{}_thumbnail_{}.{}", file_stem, suffix, file_extension),
            file_name_thumbnail_webp: format!("{}_thumbnail_{}.webp", file_stem, suffix),
            file_name_webp: format!("{}_{}.{}", file_stem, suffix, file_extension),
            file_type: file_extension.to_lowercase(),
            suffix: suffix.to_string(),
        }
    }

    fn resize_image(
        file_name_src: &PathBuf,
        file_name_optimized_image: &PathBuf,
        nwidth: &u32,
        nquality: &u8,
        webpimage: &bool,
        thumbnail: &bool,
    ) -> Result<(), ImageError> {
        let original_image = image::open(&file_name_src).expect("Opening image original failed");
        let optimized_image = match thumbnail {
            true => ImageOptimizer::new_thumbnail(
                original_image.to_owned(),
                file_name_optimized_image.to_owned(),
                *nwidth,
                *nquality,
            ),
            false => ImageOptimizer::new(
                original_image.to_owned(),
                file_name_optimized_image.to_owned(),
                *nwidth,
                *nquality,
            ),
        };
        println!(
            "Converting {:?} (w: {:?}, h: {:?}) to {:?} (w: {:?}, h: {:?})",
            file_name_src,
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
            optimized_image.save_jpg_image()
        } else {
            optimized_image.save_png_image()
        }
    }

    pub fn run_resize_images(
        source: &String,
        destination: &String,
        suffix: &String,
        width: &u32,
        quality: &u8,
        webpimage: &bool,
        thumbnail: &bool,
    ) {
        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        let entries = match Path::new(source).is_file() {
            true => glob_with(source, options)
                .unwrap()
                .flatten()
                .collect::<Vec<PathBuf>>(),
            false => {
                let pattern_jpg = format!("{}/**/*.jpg", source);
                let pattern_jpeg = format!("{}/**/*.jpeg", source);
                let pattern_png = format!("{}/**/*.png", source);
                glob_with(&pattern_jpg, options)
                    .unwrap()
                    .chain(glob_with(&pattern_jpeg, options).unwrap())
                    .chain(glob_with(&pattern_png, options).unwrap())
                    .flatten()
                    .collect::<Vec<PathBuf>>()
            }
        };
        println!("------------------------------------------");
        if entries.len() == 1 {
            println!("{} image will be optimized.", entries.len());
        } else {
            println!("{} images are optimized.", entries.len());
        }
        println!("------------------------------------------");
        let source_path = fs::canonicalize(&source).unwrap();
        let destioname_path = fs::canonicalize(&destination).unwrap();
        for file_name_src in entries {
            let image = ResizeImage::new(&file_name_src, &source_path, &destioname_path, suffix);
            println!("{:?}", image);
            ResizeImage::resize_image(
                &file_name_src,
                &image.dest_file,
                width,
                quality,
                webpimage,
                &false,
            )
            .unwrap();
            if thumbnail == &true {
                ResizeImage::resize_image(
                    &file_name_src,
                    &image.dest_thumbnail,
                    width,
                    quality,
                    webpimage,
                    thumbnail,
                )
                .unwrap();
            }
            println!(
                "The file '{:?}' has been converted successfully!",
                &file_name_src
            );
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

    /// The test checks if the file_names for the optimized image and the thumbnail can be generated.
    #[test]
    fn test_new() {
        let tempdir = tempdir().unwrap().into_path();
        let file_name_src = tempdir.join("spool/foo/bar/baz.jpg");
        fs::create_dir_all(&file_name_src.parent().unwrap()).unwrap();
        fs::copy("media/paradise/fly.JPG", &file_name_src).unwrap();
        let source = tempdir.join("spool");
        let destination = tempdir.join("moon");
        let image = ResizeImage::new(&file_name_src, &source, &destination, &String::from("sm"));
        assert_eq!(tempdir.join("moon/foo/bar/baz_sm.jpg"), image.dest_file);
        assert_eq!(
            tempdir.join("moon/foo/bar/baz_thumbnail_sm.jpg"),
            image.dest_thumbnail
        );
    }

    /// The test checks whether the original images in a folder can be optimized.
    #[test]
    fn test_resize_images_in_folder() {
        let tempdir = String::from(tempdir().unwrap().into_path().to_str().unwrap());

        // optimize images
        ResizeImage::run_resize_images(
            &String::from("media"),
            &tempdir,
            &String::from("sm"),
            &500,
            &90,
            &true,
            &true,
        );

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

    /// The test checks if on original image can be optimized.
    #[test]
    fn test_resize_one_image() {
        let tempdir = tempdir().unwrap().into_path().to_str().unwrap().to_string();

        // optimize images
        ResizeImage::run_resize_images(
            &String::from("media/paradise/fly.JPG"),
            &tempdir,
            &String::from("xxs"),
            &250,
            &90,
            &true,
            &false,
        );

        let mut temp_img_jpg_webp_path = tempdir.to_owned();
        temp_img_jpg_webp_path.push_str("/fly_xxs.webp");

        let temp_img_jpg_webp = image::open(temp_img_jpg_webp_path).unwrap();

        let img_jpg_webp_ok = image::open(format!("./testdata/fly_xxs.{PLATFORM}.webp")).unwrap();

        assert_eq!(img_jpg_webp_ok, temp_img_jpg_webp);

        remove_dir_all(tempdir).unwrap();
    }
}
