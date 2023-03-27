use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Sets the source folder or a source file: ./media or ./media/paradise/fly.JPG
    #[clap(short, long, value_parser)]
    pub destination: String,

    /// write metadata.json
    #[clap(short, long, value_parser, default_value = "")]
    pub jsonfile: String,

    /// "Sets the source folder or a source file: ./media or ./media/paradise/fly.JPG"
    #[clap(short, long, value_parser)]
    pub source: String,

    /// "Sets the prefix for the fileNames in the jsonfile: /www/moon/"
    #[clap(short, long, value_parser, default_value = "")]
    pub prefix: String,

    /// Sets the qualities of the optimized images: 90, 80
    #[clap(short, long, num_args(1..), required = true)]
    pub qualities: Vec<u8>,

    /// Sets the widths of the optimized images: 250 100
    #[clap(short, long, num_args(1..), required = true)]
    pub widths: Vec<u32>,
}
