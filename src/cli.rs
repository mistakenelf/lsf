use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

use crate::listing::Listing;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = ".", parse(from_os_str))]
    path: PathBuf,

    #[clap(short, long, takes_value = false)]
    hide_icons: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let listing = Listing::new(&args.path, args.hide_icons);

    listing.print_listing()
}
