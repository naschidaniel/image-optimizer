use chrono::Local;
use glob::{glob_with, MatchOptions};
use image::DynamicImage;
use image::codecs::png;
use image::imageops::FilterType;
use image::jpeg::JpegEncoder;
use image::png::PngEncoder;
use image::GenericImageView;
use image::ImageError;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use webp::{Encoder, PixelLayout};

fn encode_webp_image(resized: &DynamicImage, filename: &Path, width: u32, height: u32, quality: f32) {
    let mut buffer = File::create(filename).unwrap();
    let webp_image = Encoder::new(&resized.to_bytes(), PixelLayout::Rgb, width, height).encode(quality);
    buffer.write(&*webp_image).unwrap();
}

fn encode_image(
    filename_path: PathBuf,
    filename_new_path: &PathBuf,
    width: &u32,
    quality: &u8,
    webp_image: &bool,
) -> Result<(), ImageError> {
    let extension = filename_path.extension().unwrap().to_str().unwrap();
    let extension_lowercase = extension.to_lowercase();
    let img = image::open(&filename_path).expect("Opening image failed");
    let new_width = width.to_owned();
    let resize_ratio = img.width() as f32 / new_width as f32;
    let new_height_f32 = img.height() as f32 / resize_ratio;
    let new_height = new_height_f32 as u32;

    println!(
        "Converting {:?} (w: {:?}, h: {:?}) to {:?} (w: {:?}, h: {:?}), resize ratio: {:?}",
        filename_path, img.width(), img.height(), filename_new_path, new_width, new_height, resize_ratio
    );

    let resized = img.resize_exact(new_width, new_height, FilterType::Lanczos3);
    let file = File::create(filename_new_path).unwrap();
    let ref mut file_output = BufWriter::new(file);
    
    if !(extension_lowercase == "jpg" || extension_lowercase == "jpeg" || extension_lowercase == "png") {
        panic!("The format is not supported")
    }
    if webp_image == &true {
        let rudi = &filename_new_path.to_str().to_owned().unwrap().replace(&extension, "webp");
        let filename = Path::new(rudi);
        println!("Creating WebP image {:?} (w: {:?}, h: {:?}), resize ratio: {:?}", filename, new_width, new_height, resize_ratio);
        encode_webp_image(&resized, filename, new_width, new_height, *quality as f32);
    }
    
    if extension_lowercase == "jpg" || extension_lowercase == "jpeg" {
        JpegEncoder::new_with_quality(file_output, *quality).encode(
            &resized.to_bytes(),
            new_width,
            new_height,
            img.color(),
        )
    } else {
        PngEncoder::new_with_quality(
          file_output,
            png::CompressionType::Default,
            png::FilterType::NoFilter,
        )
        .encode(&resized.to_bytes(), new_width, new_height, img.color())
    }
}

fn create_filename_new_path(
    filename_path: &PathBuf,
    input_folder: &String,
    output_folder: &String,
    suffix: &String,
) -> PathBuf {
    let input_folder_pattern = input_folder.strip_prefix("./").unwrap();
    let input_sub_folders = filename_path.parent().unwrap().strip_prefix(input_folder_pattern).unwrap();
    let filename = filename_path.file_name().unwrap().to_str().unwrap();
    let file_stem = filename_path.file_stem().unwrap().to_str().unwrap();
    let file_stem_new = format!("{}_{}", file_stem, suffix);
    let filename_new = filename.to_owned().replace(file_stem, &*file_stem_new);
    let output_path = Path::new(output_folder).join(input_sub_folders);
    fs::create_dir_all(&output_path).unwrap();
    output_path.join(filename_new)
}

fn resize_images(
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
            Ok(filename_path) => {
                let filename_new_path =
                    create_filename_new_path(&filename_path, input_folder, output_folder, suffix);
                let handle = encode_image(filename_path, &filename_new_path, width, quality, webp_image);
                match handle {
                    Ok(_) => println!(
                        "The file '{:?}' was converted successfully!",
                        filename_new_path
                    ),
                    Err(_) => {
                        println!("The file '{:?}' could not be converted!", filename_new_path)
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

    resize_images(&args[1], &args[2], &args[3], width, quality, webp_image);
    let end_time = Local::now().time();
    let diff = end_time - start_time;
    println!("Total time {} in Seconds", diff.num_seconds());
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use fs::remove_dir_all;

    #[test]
    fn test_resize_images() {
        let media = String::from("./media");
        let tempdir = String::from(tempdir().unwrap().into_path().to_str().unwrap());
        let suffix = String::from("sm");
        let width = 500;
        let quality = 90;
        let webp_image = true;
        resize_images(&media, &tempdir, &suffix, &width, &quality, &webp_image);
        let img_jpg_ok = image::open("./testdata/test_ok_fly_sm.JPG").expect("Opening './testdata/test_ok_fly_sm.JPG' failed");
        let img_webp_ok = image::open("./testdata/test_ok_fly_sm.webp").expect("Opening './testdata/test_ok_fly_sm.webp' Jpg image failed");

        let mut temp_img_jpg_path = tempdir.to_owned();
        temp_img_jpg_path.push_str("/paradise/fly_sm.JPG");
        let mut temp_img_webp_path = tempdir.to_owned();
        temp_img_webp_path.push_str("/paradise/fly_sm.webp");

        let temp_img = image::open(temp_img_jpg_path).expect("Opening temporary Jpg image failed");
        assert_eq!(img_jpg_ok, temp_img);
        let temp_img = image::open(temp_img_webp_path).expect("Opening temporary WebP image failed");
        assert_eq!(img_webp_ok, temp_img);
        remove_dir_all(tempdir).unwrap();
    }
}