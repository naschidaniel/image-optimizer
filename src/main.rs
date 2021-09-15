mod image_optimizer;

use chrono::Local;
use glob::{glob_with, MatchOptions};
use image::GenericImageView;
use image::ImageError;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// The necessary file structure is created and the modified file name is returned as `PathBuf`.
fn create_output_dir_filename(
    filename_original: &PathBuf,
    input_folder: &String,
    output_folder: &String,
    suffix: &String,
) -> PathBuf {
    let input_folder_pattern = input_folder.strip_prefix("./").unwrap();
    let input_sub_folders = filename_original
        .parent()
        .unwrap()
        .strip_prefix(input_folder_pattern)
        .unwrap();
    let file_stem = filename_original.file_stem().unwrap().to_str().unwrap();
    let file_stem_optimize_image = format!("{}_{}", file_stem, suffix);
    let filename_optimize_image = filename_original
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .replace(file_stem, &*file_stem_optimize_image);
    let output_path = Path::new(output_folder).join(input_sub_folders);
    fs::create_dir_all(&output_path).unwrap();
    output_path.join(filename_optimize_image)
}

fn encode_image(
    filename_original: PathBuf,
    filename_optimized_image: &PathBuf,
    nwidth: &u32,
    nquality: &u8,
    webp_image: &bool,
) -> Result<(), ImageError> {
    let extension = filename_original.extension().unwrap().to_str().unwrap();
    if !(extension.to_lowercase() == "jpg"
        || extension.to_lowercase() == "jpeg"
        || extension.to_lowercase() == "png")
    {
        panic!("The format '{:?}' is not supported", extension)
    }
    let original_image = image::open(&filename_original).expect("Opening image original failed");
    let optimized_image = image_optimizer::ImageOptimizer::new(
        original_image.to_owned(),
        filename_optimized_image.to_owned(),
        *nwidth,
        *nquality,
    );
    println!(
        "Converting {:?} (w: {:?}, h: {:?}) to {:?} (w: {:?}, h: {:?}), resize ratio: {:?}",
        filename_original,
        original_image.width(),
        original_image.height(),
        filename_optimized_image,
        optimized_image.nwidth,
        optimized_image.nheight,
        optimized_image.resize_ratio
    );

    if webp_image == &true {
        optimized_image.save_webp_image();
    }

    if extension.to_lowercase() == "jpg" || extension.to_lowercase() == "jpeg" {
        optimized_image.save_jpg_image()
    } else {
        optimized_image.save_png_image()
    }
}

fn run_resize_images(
    input_folder: &String,
    output_folder: &String,
    suffix: &String,
    width: &u32,
    quality: &u8,
    webp_image: &bool,
) {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    let pattern_jpg = format!("{}/**/*.jpg", input_folder);
    let pattern_jpeg = format!("{}/**/*.jpeg", input_folder);
    let pattern_png = format!("{}/**/*.png", input_folder);

    for entry in glob_with(&pattern_jpg, options)
        .unwrap()
        .chain(glob_with(&pattern_jpeg, options).unwrap())
        .chain(glob_with(&pattern_png, options).unwrap())
    {
        match entry {
            Ok(filename_original) => {
                let filename_optimize_image = create_output_dir_filename(
                    &filename_original,
                    input_folder,
                    output_folder,
                    suffix,
                );
                let handle = encode_image(
                    filename_original,
                    &filename_optimize_image,
                    width,
                    quality,
                    webp_image,
                );
                match handle {
                    Ok(_) => println!(
                        "The file '{:?}' was converted successfully!",
                        filename_optimize_image
                    ),
                    Err(_) => {
                        println!(
                            "The file '{:?}' could not be converted!",
                            filename_optimize_image
                        )
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn main() {
    let start_time = Local::now().time();
    let args: Vec<String> = env::args().collect();

    let width = &args[4].parse().unwrap();
    let quality = &args[5].parse().unwrap();
    let webp_image = &args[6].parse().unwrap();
    println!("Input Folder: {}", &args[1]);
    println!("Output Folder: {}", &args[2]);
    println!("Filename Suffix: {}", &args[3]);
    println!("Width: {}", width);
    println!("Quality: {}", quality);
    println!("WebP Image: {}", webp_image);

    run_resize_images(&args[1], &args[2], &args[3], width, quality, webp_image);
    let end_time = Local::now().time();
    let diff = end_time - start_time;
    println!("Duration {} in Seconds", diff.num_seconds());
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::remove_dir_all;
    use tempfile::tempdir;

    #[test]
    /// The test checks whether the original images can be optimized.
    fn test_resize_images() {
        // command line arguments
        let media = String::from("./media");
        let tempdir = String::from(tempdir().unwrap().into_path().to_str().unwrap());
        let suffix = String::from("sm");
        let width = 500;
        let quality = 90;
        let webp_image = true;

        // optimize images
        run_resize_images(&media, &tempdir, &suffix, &width, &quality, &webp_image);

        let mut temp_img_jpg_path = tempdir.to_owned();
        temp_img_jpg_path.push_str("/paradise/fly_sm.JPG");
        let mut temp_img_png_path = tempdir.to_owned();
        temp_img_png_path.push_str("/paradise/paragliding_sm.png");
        let mut temp_img_jpg_webp_path = tempdir.to_owned();
        temp_img_jpg_webp_path.push_str("/paradise/fly_sm.webp");
        let mut temp_img_png_webp_path = tempdir.to_owned();
        temp_img_png_webp_path.push_str("/paradise/paragliding_sm.webp");

        let temp_img_jpg = image::open(temp_img_jpg_path).unwrap();
        let temp_img_png = image::open(temp_img_png_path).unwrap();
        let temp_img_jpg_webp = image::open(temp_img_jpg_webp_path).unwrap();
        let temp_img_png_webp = image::open(temp_img_png_webp_path).unwrap();

        let mut temp_img_jpg_thumbnail_path = tempdir.to_owned();
        temp_img_jpg_thumbnail_path.push_str("/paradise/fly_sm_thumbnail.JPG");
        let mut temp_img_png_thumbnail_path = tempdir.to_owned();
        temp_img_png_thumbnail_path.push_str("/paradise/paragliding_sm_thumbnail.png");
        let mut temp_img_jpg_webp_thumbnail_path = tempdir.to_owned();
        temp_img_jpg_webp_thumbnail_path.push_str("/paradise/fly_sm_thumbnail.webp");
        let mut temp_img_png_webp_thumbnail_path = tempdir.to_owned();
        temp_img_png_webp_thumbnail_path.push_str("/paradise/paragliding_sm_thumbnail.webp");

        let temp_img_jpg_thumbnail = image::open(temp_img_jpg_thumbnail_path).unwrap();
        let temp_img_png_thumbnail = image::open(temp_img_png_thumbnail_path).unwrap();
        let temp_img_jpg_webp_thumbnail = image::open(temp_img_jpg_webp_thumbnail_path).unwrap();
        let temp_img_png_webp_thumbnail = image::open(temp_img_png_webp_thumbnail_path).unwrap();

        // valid testdata
        let img_jpg_ok = image::open("./testdata/test_ok_fly_sm.JPG").unwrap();
        let img_jpg_webp_ok = image::open("./testdata/test_ok_fly_sm.webp").unwrap();
        let img_png_ok = image::open("./testdata/test_ok_paragliding_sm.png").unwrap();
        let img_png_webp_ok = image::open("./testdata/test_ok_paragliding_sm.webp").unwrap();

        assert_eq!(img_jpg_ok, temp_img_jpg);
        assert_eq!(img_jpg_webp_ok, temp_img_jpg_webp);
        assert_eq!(img_png_ok, temp_img_png);
        assert_eq!(img_png_webp_ok, temp_img_png_webp);

        // valid testdata thumbnails
        let img_jpg_thumbnail_ok = image::open("./testdata/test_ok_fly_sm_thumbnail.JPG").unwrap();
        let img_jpg_webp_thumbnail_ok = image::open("./testdata/test_ok_fly_sm_thumbnail.webp").unwrap();
        let img_png_thumbnail_ok = image::open("./testdata/test_ok_paragliding_sm_thumbnail.png").unwrap();
        let img_png_webp_thumbnail_ok = image::open("./testdata/test_ok_paragliding_sm_thumbnail.webp").unwrap();

        assert_eq!(img_jpg_thumbnail_ok, temp_img_jpg_thumbnail);
        assert_eq!(img_jpg_webp_thumbnail_ok, temp_img_jpg_webp_thumbnail);
        assert_eq!(img_png_thumbnail_ok, temp_img_png_thumbnail);
        assert_eq!(img_png_webp_thumbnail_ok, temp_img_png_webp_thumbnail);
        remove_dir_all(tempdir).unwrap();
    }
}
