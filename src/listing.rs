use std::error::Error;
use std::path::PathBuf;
use std::{fs, io};

use crate::list_item::ListItem;

pub struct Listing {
    dir_name: PathBuf,
    icons: bool,
    all: bool,
    long: bool,
    single: bool,
}

impl Listing {
    pub fn new(dir_name: &PathBuf, icons: bool, all: bool, long: bool, single: bool) -> Listing {
        Listing {
            dir_name: dir_name.to_path_buf(),
            icons,
            all,
            long,
            single,
        }
    }

    pub fn print_listing(&self) -> Result<(), Box<dyn Error>> {
        if self.dir_name.is_dir() {
            let mut items: Vec<ListItem> = vec![];
            let mut entries = fs::read_dir(&self.dir_name)?
                .map(|res| res.map(|e| e))
                .collect::<Result<Vec<_>, io::Error>>()?;

            entries.sort_by_key(|e| e.file_name().clone());

            for entry in entries {
                let item = ListItem::new(entry);

                if self.all {
                    items.push(item);
                } else if !item.file_name.starts_with(".") {
                    items.push(item);
                }
            }

            for sorted_item in &items {
                if self.long {
                    sorted_item.display_details();
                }

                if self.icons {
                    sorted_item.display_icon();
                }

                sorted_item.display_filename();

                if self.long || self.single {
                    print!("\n");
                }
            }
        }

        Ok(())
    }
}
