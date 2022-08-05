mod command_parser;
mod image_optimizer;
mod resize_image;

use clap::Parser;
use command_parser::Args;
use resize_image::ResizeImage;
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

    ResizeImage::run_resize_images(
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
