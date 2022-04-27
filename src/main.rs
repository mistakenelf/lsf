use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

mod list_item;
mod listing;

use crate::listing::Listing;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "List stuff with fancy output")]
struct Args {
    #[clap(short, long, default_value = ".", parse(from_os_str))]
    path: PathBuf,

    #[clap(short = 'i', long, takes_value = false)]
    hide_icons: bool,

    #[clap(short = 'd', long, takes_value = false)]
    hide_hidden: bool,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let listing = Listing::new(&args.path, args.hide_icons, args.hide_hidden);

    listing.print_listing()
}
