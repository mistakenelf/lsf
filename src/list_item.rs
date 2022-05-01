use chrono::{DateTime, Local};
use crossterm::style::Stylize;
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::fs::DirEntry;
use std::io::stdout;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ListItem {
    pub file_name: String,
    size: u64,
    modified_date: DateTime<Local>,
    is_dir: bool,
}

impl ListItem {
    pub fn new(entry: DirEntry) -> ListItem {
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
        }
    }

    pub fn display_details(&self) {
        execute!(
            stdout(),
            Print(format!(
                "{0:<6} {1:<10} ",
                ListItem::format_size(self.size),
                self.modified_date.format("%_d %b %H:%M").to_string()
            )),
        )
        .unwrap();
    }

    pub fn display_icon(&self) {
        let icon: String;
        let color: Color;

        if self.is_dir {
            icon = String::from("\u{f74a}");
            color = Color::Rgb {
                r: 227,
                g: 177,
                b: 77,
            };
        } else {
            icon = String::from("\u{f723}");
            color = Color::Rgb {
                r: 65,
                g: 129,
                b: 190,
            };
        }

        execute!(
            stdout(),
            SetForegroundColor(color),
            Print(format!(" {0:<2}", icon)),
            ResetColor,
        )
        .unwrap();
    }

    pub fn display_filename(&self) {
        let file_name: String;

        if self.is_dir {
            file_name = format!("{:<10}", self.file_name.clone().bold().blue());
        } else {
            file_name = format!("{:<10}", self.file_name);
        }

       print!("{} ", file_name); 
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
