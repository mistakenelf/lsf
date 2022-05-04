use chrono::{DateTime, Local};
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::fs::DirEntry;
use std::io::stdout;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ListItem {
    pub file_name: String,
    size: u64,
    modified_date: DateTime<Local>,
    is_dir: bool,
    mode: u16,
}

impl ListItem {
    pub fn new(entry: DirEntry) -> ListItem {
        let file_name = entry.file_name().into_string().unwrap();
        let metadata = entry.metadata().unwrap();
        let size = metadata.len();
        let modified: DateTime<Local> = DateTime::from(metadata.modified().unwrap());
        let is_dir = metadata.is_dir();
        let mode = metadata.permissions().mode();

        ListItem {
            file_name,
            size,
            modified_date: modified,
            is_dir,
            mode: mode as u16,
        }
    }

    pub fn display_details(&self) {
        let permissions = ListItem::parse_permissions(self.mode);

        execute!(
            stdout(),
            Print(format!(
                "{} {:<6} {:<10} ",
                permissions,
                ListItem::format_size(self.size),
                self.modified_date.format("%D %H:%M").to_string()
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

    fn triplet(mode: u16, read: u16, write: u16, execute: u16) -> String {
        match (mode & read, mode & write, mode & execute) {
            (0, 0, 0) => "---",
            (_, 0, 0) => "r--",
            (0, _, 0) => "-w-",
            (0, 0, _) => "--x",
            (_, 0, _) => "r-x",
            (_, _, 0) => "rw-",
            (0, _, _) => "-wx",
            (_, _, _) => "rwx",
        }
        .to_string()
    }

    fn parse_permissions(mode: u16) -> String {
        let user = ListItem::triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
        let group = ListItem::triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
        let other = ListItem::triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);

        [user, group, other].join("")
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
