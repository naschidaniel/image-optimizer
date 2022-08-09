mod command_parser;
mod image_optimizer;
mod resize_image;

use clap::Parser;
use command_parser::Args;
use resize_image::ResizeImage;
use std::fs;
use std::io::Write;
use std::time::Instant;

/// A tool to optimize images for web. Check out the README on https://github.com/naschidaniel/image-optimizer/blob/main/README.md for more details

fn main() {
    let start_time = Instant::now();

    // Command line arguments
    let args = Args::parse();

    println!(
        "Running image-optimizer version {}",
        env!("CARGO_PKG_VERSION")
    );
    println!("Source: {}", &args.source);
    println!("Destination folder: {}", &args.destination);
    println!("Widths: {:?}", &args.widths);
    println!("Qualities: {:?}", &args.qualities);
    if !args.jsonfile.is_empty() {
        println!("JSON File: {}/{}", &args.destination, &args.jsonfile);
    }

    let mut json_data = Vec::new();
    for (index, width) in args.widths.iter().enumerate() {
        let mut new_images = ResizeImage::new(
            args.source.to_owned(),
            args.destination.to_owned(),
            args.prefix.to_owned(),
            width.to_owned(),
            args.qualities[index].to_owned(),
        );
        new_images.run_resize_images();
        json_data.push(new_images.get_metadata_json());
    }
    if !args.jsonfile.is_empty() {
        let metadata_file = fs::canonicalize(&args.destination)
            .unwrap()
            .join(args.jsonfile);
        let mut file = fs::File::create(&metadata_file).unwrap();
        let content = format!("[{}]", json_data.join(", "));
        file.write_all(content.as_bytes()).unwrap();
    }

    let end_time = start_time.elapsed();
    println!("Duration {} in Seconds", end_time.as_secs());
}
