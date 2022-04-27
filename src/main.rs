mod cli;
mod list_item;
mod listing;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cli::run()
}
