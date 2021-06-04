use chrono::Local;
use glob::{glob_with, MatchOptions};
use image::codecs::png;
use image::imageops::FilterType;
use image::jpeg::JpegEncoder;
use image::png::PngEncoder;
use image::GenericImageView;
use image::ImageError;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

fn encode_image(
    filename_path: PathBuf,
    filename_new_path: &PathBuf,
    width: &u32,
    quality: &u8,
) -> Result<(), ImageError> {
    let extension = filename_path.extension().unwrap().to_str().unwrap().to_lowercase();
    let img = image::open(&filename_path).expect("Opening image failed");
    let new_width = width.to_owned();
    let ratio = img.width() as f32 / new_width as f32;
    let new_hight_f32 = img.height() as f32 / ratio;
    let new_hight = new_hight_f32 as u32;

    println!(
        "Converting {:?} (w: {:?}, h: {:?}) to {:?} (w: {:?}, h: {:?}), ratio: {:?}",
        filename_path, img.width(), img.height(), filename_new_path, new_width, new_hight, ratio
    );

    let resized = img.resize_exact(new_width, new_hight, FilterType::Lanczos3);
    let file = File::create(filename_new_path).unwrap();
    let ref mut file_output = BufWriter::new(file);

    if extension == "jpg" || extension == "jpeg" {
        JpegEncoder::new_with_quality(file_output, *quality).encode(
            &resized.to_bytes(),
            new_width,
            new_hight,
            img.color(),
        )
    } else if extension == "png" {
        PngEncoder::new_with_quality(
          file_output,
            png::CompressionType::Default,
            png::FilterType::NoFilter,
        )
        .encode(&resized.to_bytes(), new_width, new_hight, img.color())
    } else {
        panic!("The format is not supported")
    }
}

fn create_filename_new_path(
    filename_path: &PathBuf,
    dist_folder: &String,
    suffix: &String,
) -> PathBuf {
    let folder = filename_path.parent().unwrap();
    let filename = filename_path.file_name().unwrap().to_str().unwrap();
    let file_stem = filename_path.file_stem().unwrap().to_str().unwrap();
    let file_stem_new = format!("{}_{}", file_stem, suffix);
    let filename_new = filename.to_owned().replace(file_stem, &*file_stem_new);
    let dist_media_path = Path::new(dist_folder).join(folder);
    fs::create_dir_all(&dist_media_path).unwrap();
    dist_media_path.join(filename_new)
}

fn resize_images(
    media_path: &String,
    dist_folder: &String,
    suffix: &String,
    width: &u32,
    quality: &u8,
) {
  let options = MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};
    let pattern_jpg = format!("{}/**/*.jpg", media_path);
    let pattern_jpeg = format!("{}/**/*.jpeg", media_path);
    let pattern_png = format!("{}/**/*.png", media_path);

    for entry in glob_with(&pattern_jpg, options)
        .unwrap()
        .chain(glob_with(&pattern_jpeg, options).unwrap())
        .chain(glob_with(&pattern_png, options).unwrap())
    {
        match entry {
            Ok(filename_path) => {
                let filename_new_path =
                    create_filename_new_path(&filename_path, dist_folder, suffix);
                let handle = encode_image(filename_path, &filename_new_path, width, quality);
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
    println!("Folder: {}", &args[1]);
    println!("Folder: {}", &args[2]);
    println!("filename_suffix: {}", &args[3]);
    println!("Width {}", width);
    println!("Quality {}", width);

    resize_images(&args[1], &args[2], &args[3], width, quality);
    let end_time = Local::now().time();
    let diff = end_time - start_time;
    println!("Total time {} in Seconds", diff.num_seconds());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_images() {
        let media = String::from("./media");
        let testdata = String::from("./testdata");
        let suffix = String::from("sm");
        let width = 500;
        let quality = 90;
        resize_images(&media, &testdata, &suffix, &width, &quality);
        let img_ok = image::open("./testdata/test_ok_fly_sm.JPG").expect("Opening image failed");
        let img = image::open("./testdata/media/fly_sm.JPG").expect("Opening image failed");
        assert_eq!(img_ok, img);
    }
}