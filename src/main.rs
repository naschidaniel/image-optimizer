mod command_parser;
mod image_optimizer;
mod resize_image;

use clap::Parser;
use command_parser::Args;
use std::time::Instant;

/// A tool to optimize images for web. Check out the README on https://github.com/naschidaniel/image-optimizer/blob/main/README.md for more details

fn main() {
    let start_time = Instant::now();

    // Command line arguments
    let args = Args::parse();
    let webpimage = args.webpimage.parse::<bool>().unwrap();
    let thumbnail = args.thumbnail.parse::<bool>().unwrap();

    println!(
        "Running image-optimizer version {}",
        env!("CARGO_PKG_VERSION")
    );
    println!("Source: {}", &args.source);
    println!("Destination folder: {}", &args.destination);
    println!("Filename Suffix: {}", &args.suffix);
    println!("Width: {}", &args.width);
    println!("Quality: {}", &args.quality);
    println!("WebP-Image: {}", webpimage);
    println!("Thumbnail: {}", thumbnail);

    resize_image::run_resize_images(
        &args.source,
        &args.destination,
        &args.suffix,
        &args.width,
        &args.quality,
        &webpimage,
        &thumbnail,
    );
    let end_time = start_time.elapsed();
    println!("Duration {} in Seconds", end_time.as_secs());
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
            resize_image::create_filenames(&filename_original, &output_path, &String::from("sm"));
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
            resize_image::create_output_dir(&filename_original, &input_folder, &output_folder);
        let temp_output_path_ok = tempdir.join("moon/foo/bar");
        assert_eq!(temp_output_path_ok, temp_output_path);
        remove_dir_all(tempdir).unwrap();
    }

    /// The test checks whether the original images in a folder can be optimized.
    #[test]
    fn test_resize_images_in_folder() {
        let tempdir = String::from(tempdir().unwrap().into_path().to_str().unwrap());

        // optimize images
        resize_image::run_resize_images(
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
        resize_image::run_resize_images(
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
