use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Sets the source folder or a source file: ./media or ./media/paradise/fly.JPG
    #[clap(short, long, value_parser)]
    pub destination: String,

    /// "Sets the source folder or a source file: ./media or ./media/paradise/fly.JPG"
    #[clap(short = 'r', long, value_parser)]
    pub source: String,

    /// Sets the suffix of the optimized images: sm
    #[clap(short, long, value_parser)]
    pub suffix: String,

    /// Generate a copy in thumbnail of optimized images
    #[clap(short, long, value_parser)]
    pub thumbnail: String,

    /// Sets the quality of the optimized images
    #[clap(short, long, value_parser)]
    pub quality: u8,

    /// Generate a copy in WebP Format of optimized images
    #[clap(short = 'x', long, value_parser)]
    pub webpimage: String,

    /// Sets the width of the optimized images
    #[clap(short, long, value_parser)]
    pub width: u32,
}
