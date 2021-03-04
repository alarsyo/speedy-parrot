use std::path::PathBuf;

use clap::Clap;
use image::io::Reader as ImageReader;

use speedy_parrot::to_black_and_white;

#[derive(Clap)]
#[clap(
    version = "0.1",
    author = "Antoine Martin <antoine97.martin@gmail.com>"
)]
struct Opts {
    #[clap(short, long, parse(from_os_str))]
    input: PathBuf,
    #[clap(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let img = ImageReader::open(opts.input)?.decode()?;
    let img = img.to_luma8();

    let res = to_black_and_white(img);

    res.save(opts.output)?;

    Ok(())
}
