use chrono::{DateTime, Local};
use crossterm::style::Stylize;
use crossterm::{execute, style, style::Color, style::Print, style::ResetColor};
use std::io::stdout;
use std::fs::DirEntry;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ListItem {
    pub file_name: String,
    size: u64,
    modified_date: DateTime<Local>,
    is_dir: bool,
    hide_icons: bool,
    hide_hidden: bool,
}

impl ListItem {
    pub fn new(entry: DirEntry, hide_icons: bool, hide_hidden: bool) -> ListItem {
        let file_name = entry.file_name().into_string().unwrap();
        let metadata = entry.metadata().unwrap();
        let size = metadata.len();
        let modified: DateTime<Local> = DateTime::from(metadata.modified().unwrap());
        let is_dir = metadata.is_dir();

        ListItem {
            file_name,
            size,
            modified_date: modified,
            is_dir,
            hide_icons,
            hide_hidden,
        }
    }

    pub fn print(&self) {
        let file_name: String;
        let color: Color;
        let mut icon: String;

        if self.is_dir {
            icon = String::from("\u{f74a}");
            file_name = format!(
                "{0:<10}\n",
                self.file_name.clone().bold().blue()
            );
            
            if self.hide_icons {
                icon = String::from("");
            }

            if self.hide_hidden && self.file_name.starts_with(".") {
                return 
            }

            color = Color::Rgb {
                r: 227,
                g: 177,
                b: 77,
            };
        } else {
            file_name = format!("{0:<10}\n", self.file_name);
            icon = String::from("\u{f723}");

            if self.hide_icons {
                icon = String::from("");
            }

            if self.hide_hidden && file_name.starts_with(".") {
                return
            }

            color = Color::Rgb {
                r: 65,
                g: 129,
                b: 190,
            };
        }

        execute!(
            stdout(),
            Print(format!(
                "{0:<6} {1:<10}",
                ListItem::format_size(self.size),
                self.modified_date.format("%_d %b %H:%M").to_string()
            )),
            style::SetForegroundColor(color),
            Print(format!(" {0:<2}", icon)),
            ResetColor,
            Print(file_name),
        )
        .unwrap();
    }

    fn format_size(size: u64) -> String {
        if size < 1000 {
            return format!("{}B", size);
        }

        let suffix = vec!["K", "M", "G", "T", "P", "E", "Z", "Y"];
        let mut current_size = size as f64 / 1000 as f64;

        for s in suffix.iter() {
            if current_size < 10.0 {
                return format!("{:.1}{}", current_size - 0.0499 as f64, s);
            } else if current_size < 1000.0 {
                return format!("{:.1}{}", current_size, s);
            }

            current_size /= 1000.0
        }

        return "".to_string();
    }
}
