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
    sorted_entries: Vec<ListItem>,
}

impl Listing {
    pub fn new(dir_name: &PathBuf, icons: bool, all: bool, long: bool, single: bool) -> Listing {
        Listing {
            dir_name: dir_name.to_path_buf(),
            icons,
            all,
            long,
            single,
            sorted_entries: Vec::new(),
        }
    }

    pub fn get_entries(&mut self) -> Result<(), Box<dyn Error>> {
        let mut entries = fs::read_dir(&self.dir_name)?
            .map(|res| res.map(|e| e))
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort_by_key(|e| e.file_name().clone());

        for entry in entries {
            let item = ListItem::new(entry);

            if self.all {
                self.sorted_entries.push(item);
            } else if !item.file_name.starts_with(".") {
                self.sorted_entries.push(item);
            }
        }

        Ok(())
    }

    pub fn print_listing(&mut self) -> Result<(), Box<dyn Error>> {
        if self.dir_name.is_dir() {
            self.get_entries()?;

            for item in &self.sorted_entries {
                if self.long {
                    item.display_details();
                }

                if self.icons {
                    item.display_icon();
                }

                item.display_filename();

                if self.long || self.single {
                    print!("\n");
                }
            }
        }

        Ok(())
    }
}
