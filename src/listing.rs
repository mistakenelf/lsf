use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::list_item::ListItem;

pub struct Listing {
    dir_name: PathBuf,
    show_icons: bool,
}

impl Listing {
    pub fn new(dir_name: &PathBuf, show_icons: bool) -> Listing {
        Listing {
            dir_name: dir_name.to_path_buf(),
            show_icons,
        }
    }

    pub fn print_listing(&self) -> Result<(), Box<dyn Error>> {
        if self.dir_name.is_dir() {
            for entry in fs::read_dir(&self.dir_name)? {
                if let Ok(entry) = entry {
                    let item = ListItem::new(entry, self.show_icons);
                    item.print();
                }
            }
        }

        Ok(())
    }
}
