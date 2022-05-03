use clap::Parser;
use std::path::PathBuf;
use std::process;

mod list_item;
mod listing;

use crate::listing::Listing;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "List stuff with fancy output")]
struct Args {
    #[clap(short, long, default_value = ".", parse(from_os_str))]
    path: PathBuf,

    #[clap(short, long, takes_value = false)]
    icons: bool,

    #[clap(short, long, takes_value = false)]
    all: bool,

    #[clap(short, long, takes_value = false)]
    long: bool,

    #[clap(short = '1', long, takes_value = false)]
    single: bool,
}

pub fn main() {
    let args = Args::parse();
    let mut listing = Listing::new(&args.path, args.icons, args.all, args.long, args.single);

    if let Err(ref e) = listing.print_listing() {
        println!("{}", e);
        process::exit(1);
    }
}
