use crate::image_optimizer::ImageOptimizer;
use glob::{glob_with, MatchOptions};
use image::ImageError;
use std::fs;
use std::path::{Path, PathBuf};

/// The necessary file structure is created and the modified file name is returned as `PathBuf`.
pub fn create_filenames(
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
pub fn create_output_dir(
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

pub fn resize_image(
    filename_original: &PathBuf,
    filename_optimized_image: &PathBuf,
    nwidth: &u32,
    nquality: &u8,
    webpimage: &bool,
    thumbnail: &bool,
) -> Result<(), ImageError> {
    let original_image = image::open(&filename_original).expect("Opening image original failed");
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
        let output_path = create_output_dir(&filename_original, source, destination_folder);
        let filenames_optimize_image = create_filenames(&filename_original, &output_path, suffix);
        resize_image(
            &filename_original,
            &filenames_optimize_image[0],
            width,
            quality,
            webpimage,
            &false,
        )
        .unwrap();
        if thumbnail == &true {
            resize_image(
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
