use crate::image_optimizer::ImageOptimizer;

use glob::{glob_with, MatchOptions};
use serde_derive::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConvertImage {
    #[serde(skip_serializing)]
    pub dest_file: PathBuf,
    #[serde(skip_serializing)]
    pub dest_parent: PathBuf,
    #[serde(skip_serializing)]
    pub dest_thumbnail: PathBuf,
    pub file_name_src: String,
    pub file_name_thumbnail: String,
    pub file_name_thumbnail_webp: String,
    pub file_name: String,
    pub file_name_webp: String,
    pub file_type: String,
    pub width: u32,
    #[serde(skip_serializing)]
    pub src_file: PathBuf,
}

impl ConvertImage {
    /// The necessary file structure is created and the modified file name is returned as `PathBuf`.
    fn new(
        file_name_src: &PathBuf,
        source_path: &PathBuf,
        destination: &PathBuf,
        prefix: &String,
        width: &u32,
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
            .unwrap()
            .to_str()
            .unwrap()
            .replace('\\', "/");

        let dest_path = Path::new(destination).join(&source_path_sub_folders);
        fs::create_dir_all(&dest_path).unwrap();

        let file_stem = file_name_src.file_stem().unwrap().to_str().unwrap();
        let file_extension = file_name_src.extension().unwrap().to_str().unwrap();

        let metadata_prefix = format!("{}{}", prefix, source_path_sub_folders.as_str());
        let file_name = format!("{}_{}.{}", &file_stem, width, file_extension);
        let file_name_thumbnail = format!("{}_thumbnail_{}.{}", &file_stem, width, file_extension);

        Self {
            dest_file: dest_path.join(&file_name),
            dest_parent: dest_path.to_owned(),
            dest_thumbnail: dest_path.join(file_name_thumbnail),
            file_name: format!("{}/{}", metadata_prefix, &file_name).replace("//", "/"),
            file_name_src: format!(
                "{}/{}",
                metadata_prefix,
                file_name_src_absolute
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            )
            .replace("//", "/"),
            file_name_thumbnail: format!(
                "{}/{}_thumbnail_{}.{}",
                metadata_prefix, file_stem, width, file_extension
            )
            .replace("//", "/"),
            file_name_thumbnail_webp: format!(
                "{}/{}_thumbnail_{}.webp",
                metadata_prefix, file_stem, width
            )
            .replace("//", "/"),
            file_name_webp: format!("{}/{}_{}.webp", metadata_prefix, file_stem, width)
                .replace("//", "/"),
            file_type: file_extension.to_lowercase(),
            src_file: file_name_src_absolute,
            width: width.to_owned(),
        }
    }

    pub fn resize_one(&self, nwidth: &u32, nquality: &u8, is_thumbnail: &bool) {
        let original_image = image::open(&self.src_file).expect("Opening image original failed");
        let optimized_image = match is_thumbnail {
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

        optimized_image.save_webp_image();

        if optimized_image.extension().to_lowercase() == "jpg"
            || optimized_image.extension().to_lowercase() == "jpeg"
        {
            optimized_image.save_jpg_image().unwrap();
        } else {
            optimized_image.save_png_image().unwrap();
        }
    }
}

pub struct ResizeImages {
    pub destination_path: PathBuf,
    pub json: Vec<String>,
    pub quality: u8,
    pub source_path: PathBuf,
    pub source: String,
    pub prefix: String,
    pub width: u32,
}

impl ResizeImages {
    pub fn new(
        source: String,
        destination: String,
        prefix: String,
        width: u32,
        quality: u8,
    ) -> ResizeImages {
        Self {
            source_path: fs::canonicalize(&source).unwrap(),
            source,
            destination_path: fs::canonicalize(&destination).unwrap(),
            width,
            prefix,
            quality,
            json: Vec::new(),
        }
    }
    pub fn run_resize_images(&mut self) {
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
                &self.prefix,
                &self.width,
            );
            // create a Thumbnail
            image.resize_one(&self.width, &self.quality, &true);
            image.resize_one(&self.width, &self.quality, &false);
            println!("The file '{:?}' has been converted!", file_name_src);
            println!("------------------------------------------");
            let data = serde_json::to_string_pretty(&image).unwrap();
            self.json.push(data);
        }
    }
    pub fn get_metadata_json(&self) -> String {
        self.json.join(", ")
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
        let image = ConvertImage::new(
            &file_name_src,
            &source,
            &destination,
            &String::from("/www/moon/"),
            &500,
        );
        assert_eq!(tempdir.join("moon/foo/bar/baz_500.JPG"), image.dest_file);
        assert_eq!(tempdir.join("moon/foo/bar"), image.dest_parent);
        assert_eq!(
            tempdir.join("moon/foo/bar/baz_thumbnail_500.JPG"),
            image.dest_thumbnail
        );
        assert_eq!("/www/moon/foo/bar/baz.JPG", image.file_name_src);
        assert_eq!("/www/moon/foo/bar/baz_500.JPG", image.file_name);
        assert_eq!("/www/moon/foo/bar/baz_500.webp", image.file_name_webp);
        assert_eq!(
            tempdir
                .join("spool/foo/bar/baz.JPG")
                .canonicalize()
                .unwrap(),
            image.src_file
        );
        assert_eq!(
            "/www/moon/foo/bar/baz_thumbnail_500.webp",
            image.file_name_thumbnail_webp
        );
        assert_eq!("jpg", image.file_type);
        remove_dir_all(tempdir).unwrap();
    }

    /// The test checks if multiple files in multiple sub folders can be optimized.
    #[test]
    fn test_resize_images_in_folder() {
        let tempdir = String::from(tempdir().unwrap().into_path().to_str().unwrap());

        ResizeImages::new(
            String::from("media"),
            tempdir.clone(),
            String::from("/www/moon/"),
            500,
            90,
        )
        .run_resize_images();

        let mut temp_img_jpg_path = tempdir.to_owned();
        temp_img_jpg_path.push_str("/paradise/fly_500.JPG");
        let mut temp_img_jpg_webp_path = tempdir.to_owned();
        temp_img_jpg_webp_path.push_str("/paradise/fly_500.webp");
        let mut temp_img_png_webp_path = tempdir.to_owned();
        temp_img_png_webp_path.push_str("/paradise/paragliding_500.webp");

        let temp_img_jpg = image::open(temp_img_jpg_path).unwrap();
        let temp_img_jpg_webp = image::open(temp_img_jpg_webp_path).unwrap();
        let temp_img_png_webp = image::open(temp_img_png_webp_path).unwrap();

        let mut temp_img_jpg_thumbnail_path = tempdir.to_owned();
        temp_img_jpg_thumbnail_path.push_str("/paradise/fly_thumbnail_500.JPG");
        let mut temp_img_jpg_webp_thumbnail_path = tempdir.to_owned();
        temp_img_jpg_webp_thumbnail_path.push_str("/paradise/fly_thumbnail_500.webp");
        let mut temp_img_png_webp_thumbnail_path = tempdir.to_owned();
        temp_img_png_webp_thumbnail_path.push_str("/paradise/paragliding_thumbnail_500.webp");

        let temp_img_jpg_thumbnail = image::open(temp_img_jpg_thumbnail_path).unwrap();
        let temp_img_jpg_webp_thumbnail = image::open(temp_img_jpg_webp_thumbnail_path).unwrap();
        let temp_img_png_webp_thumbnail = image::open(temp_img_png_webp_thumbnail_path).unwrap();

        // valid testdata
        let img_jpg_ok = image::open(format!("./testdata/fly_500.{PLATFORM}.JPG")).unwrap();
        let img_jpg_webp_ok = image::open(format!("./testdata/fly_500.{PLATFORM}.webp")).unwrap();
        let img_png_webp_ok =
            image::open(format!("./testdata/paragliding_500.{PLATFORM}.webp")).unwrap();

        assert_eq!(img_jpg_ok, temp_img_jpg);
        assert_eq!(img_jpg_webp_ok, temp_img_jpg_webp);
        assert_eq!(img_png_webp_ok, temp_img_png_webp);

        // valid testdata thumbnails
        let img_jpg_thumbnail_ok =
            image::open(format!("./testdata/fly_thumbnail_500.{PLATFORM}.JPG")).unwrap();
        let img_jpg_webp_thumbnail_ok =
            image::open(format!("./testdata/fly_thumbnail_500.{PLATFORM}.webp")).unwrap();
        let img_png_webp_thumbnail_ok = image::open(format!(
            "./testdata/paragliding_thumbnail_500.{PLATFORM}.webp"
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

        let mut new_images = ResizeImages::new(
            String::from("media/paradise/fly.JPG"),
            tempdir.clone(),
            String::from("/www/moon/"),
            250,
            90,
        );

        new_images.run_resize_images();
        let json_data = new_images.get_metadata_json();
        println!("{}", json_data);

        let rudi = String::from(
            "{
  \"fileNameSrc\": \"/www/moon/fly.JPG\",
  \"fileNameThumbnail\": \"/www/moon/fly_thumbnail_250.JPG\",
  \"fileNameThumbnailWebp\": \"/www/moon/fly_thumbnail_250.webp\",
  \"fileName\": \"/www/moon/fly_250.JPG\",
  \"fileNameWebp\": \"/www/moon/fly_250.webp\",
  \"fileType\": \"jpg\",
  \"width\": 250
}",
        );
        assert_eq!(rudi, json_data);

        let mut temp_img_jpg_webp_path = tempdir.to_owned();
        temp_img_jpg_webp_path.push_str("/fly_250.webp");

        let temp_img_jpg_webp = image::open(temp_img_jpg_webp_path).unwrap();

        let img_jpg_webp_ok = image::open(format!("./testdata/fly_250.{PLATFORM}.webp")).unwrap();

        assert_eq!(img_jpg_webp_ok, temp_img_jpg_webp);

        remove_dir_all(tempdir).unwrap();
    }
}
