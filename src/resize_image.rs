use crate::image_optimizer::ImageOptimizer;
use glob::{glob_with, MatchOptions};
use image::ImageError;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ResizeImage {}

impl ResizeImage {
    /// The necessary file structure is created and the modified file name is returned as `PathBuf`.
    fn create_filenames(
        filename_original: &Path,
        output_path: &Path,
        suffix: &String,
    ) -> [PathBuf; 2] {
        let file_stem = filename_original.file_stem().unwrap().to_str().unwrap();
        let file_extension = filename_original.extension().unwrap().to_str().unwrap();

        let filename_optimize_image = format!("{}_{}.{}", file_stem, suffix, file_extension);
        let filename_thumbnail_optimize_image =
            format!("{}_thumbnail_{}.{}", file_stem, suffix, file_extension);
        [
            output_path.join(filename_optimize_image),
            output_path.join(filename_thumbnail_optimize_image),
        ]
    }

    /// The directory for the output is created an a `PathBuf` with the name of the folder is returned.
    fn create_output_dir(
        filename_original: &Path,
        source: &String,
        destination_folder: &String,
    ) -> PathBuf {
        let source_path = Path::new(source);
        let parent = match source_path.is_file() {
            true => source_path.parent().unwrap(),
            false => source_path,
        };

        let source_pattern = parent.strip_prefix("./").unwrap();
        let input_sub_folders = filename_original
            .parent()
            .unwrap()
            .strip_prefix(source_pattern)
            .unwrap();

        let output_path = Path::new(destination_folder).join(input_sub_folders);
        fs::create_dir_all(&output_path).unwrap();
        output_path
    }

    fn resize_image(
        filename_original: &PathBuf,
        filename_optimized_image: &PathBuf,
        nwidth: &u32,
        nquality: &u8,
        webpimage: &bool,
        thumbnail: &bool,
    ) -> Result<(), ImageError> {
        let original_image =
            image::open(&filename_original).expect("Opening image original failed");
        let optimized_image = match thumbnail {
            true => ImageOptimizer::new_thumbnail(
                original_image.to_owned(),
                filename_optimized_image.to_owned(),
                *nwidth,
                *nquality,
            ),
            false => ImageOptimizer::new(
                original_image.to_owned(),
                filename_optimized_image.to_owned(),
                *nwidth,
                *nquality,
            ),
        };
        println!(
            "Converting {:?} (w: {:?}, h: {:?}) to {:?} (w: {:?}, h: {:?})",
            filename_original,
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
        destination_folder: &String,
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
        for filename_original in entries {
            let output_path =
                ResizeImage::create_output_dir(&filename_original, source, destination_folder);
            let filenames_optimize_image =
                ResizeImage::create_filenames(&filename_original, &output_path, suffix);
            ResizeImage::resize_image(
                &filename_original,
                &filenames_optimize_image[0],
                width,
                quality,
                webpimage,
                &false,
            )
            .unwrap();
            if thumbnail == &true {
                ResizeImage::resize_image(
                    &filename_original,
                    &filenames_optimize_image[1],
                    width,
                    quality,
                    webpimage,
                    thumbnail,
                )
                .unwrap();
            }
            println!(
                "The file '{:?}' has been converted successfully!",
                &filename_original
            );
            println!("------------------------------------------");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;
    use std::path::PathBuf;
    use tempfile::tempdir;

    // Determine operating system
    const PLATFORM: &str = match cfg!(windows) {
        true => "windows",
        false => "unix",
    };

    /// The test checks if the filenames for the optimized image and the thumbnail can be generated.
    #[test]
    fn test_create_filenames() {
        let tempdir = tempdir().unwrap().into_path();
        let mut filename_original = PathBuf::new();
        filename_original.push("./foo/bar/baz.jpg");
        let output_path = tempdir.join("./moon/foo/bar/");
        let temp_filenames =
            ResizeImage::create_filenames(&filename_original, &output_path, &String::from("sm"));
        let temp_filenames_ok = [
            tempdir.join("./moon/foo/bar/baz_sm.jpg"),
            tempdir.join("./moon/foo/bar/baz_thumbnail_sm.jpg"),
        ];
        assert_eq!(temp_filenames_ok, temp_filenames);
        remove_dir_all(tempdir).unwrap();
    }

    /// The test checks if the destination folder structure can be created.
    #[test]
    fn test_create_output_dir() {
        let tempdir = tempdir().unwrap().into_path();
        let mut filename_original = PathBuf::new();
        filename_original.push("media/foo/bar/baz.jpg");
        let input_folder = String::from("./media");
        let output_folder = tempdir.join("./moon").to_str().unwrap().to_string();
        let temp_output_path =
            ResizeImage::create_output_dir(&filename_original, &input_folder, &output_folder);
        let temp_output_path_ok = tempdir.join("moon/foo/bar");
        assert_eq!(temp_output_path_ok, temp_output_path);
        remove_dir_all(tempdir).unwrap();
    }

    /// The test checks whether the original images in a folder can be optimized.
    #[test]
    fn test_resize_images_in_folder() {
        let tempdir = String::from(tempdir().unwrap().into_path().to_str().unwrap());

        // optimize images
        ResizeImage::run_resize_images(
            &String::from("./media"),
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
    fn test_resize_image() {
        let tempdir = String::from(tempdir().unwrap().into_path().to_str().unwrap());

        // optimize images
        ResizeImage::run_resize_images(
            &String::from("./media/paradise/fly.JPG"),
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
