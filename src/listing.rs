use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::list_item::ListItem;

pub struct Listing {
    dir_name: PathBuf,
    hide_icons: bool,
    hide_hidden: bool,    
}

impl Listing {
    pub fn new(dir_name: &PathBuf, hide_icons: bool, hide_hidden: bool) -> Listing {
        Listing {
            dir_name: dir_name.to_path_buf(),
            hide_icons,
            hide_hidden,
        }
    }

    pub fn print_listing(&self) -> Result<(), Box<dyn Error>> {
        if self.dir_name.is_dir() {
            let mut items: Vec<ListItem> = Vec::new();
            for entry in fs::read_dir(&self.dir_name)? {
                if let Ok(entry) = entry {
                    let item = ListItem::new(entry, self.hide_icons, self.hide_hidden);
                    items.push(item);
                }
            }

            items.sort_by_key(|i| i.file_name.clone());

            for sorted_item in items {
                sorted_item.print()
            }
        }

        Ok(())
    }
}
