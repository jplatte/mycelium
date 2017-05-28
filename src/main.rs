extern crate gio;
extern crate gtk;
extern crate mime_guess;

use std::io::{self, Write};
use std::fs;
use std::os::unix::ffi::OsStringExt;
use std::str::from_utf8;

// This apparently needs to come before the app module declaration for that to
// be able to use the macro
#[macro_use]
mod util;

mod ui {
    pub mod app;

    mod files_tree_view;
}

// TODO: Guess mime type based on file header (like Nautilus)

fn run() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for e in fs::read_dir("/home/jplatte/Projekte/redox")? {
        let entry = e?;
        let file_name = entry.file_name().into_vec();
        let mime_str = if entry.path().is_dir() {
            "inode/directory".to_owned()
        } else {
            file_name
                .iter()
                .rev()
                .position(|x| *x == b'.')
                .map(|rev_idx| file_name.len() - rev_idx)
                .and_then(
                    |idx| if idx == 1 {
                        // If the only '.' in the file name is at the start, regard it as having no
                        // file extension
                        None
                    } else {
                        from_utf8(&file_name[idx..]).ok()
                    }
                )
                .and_then(|x| mime_guess::get_mime_type_opt(x))
                .map(|m| m.to_string())
                .unwrap_or("[unknown]".to_owned())
        };

        stdout.write(&file_name)?;
        stdout.write(b" ")?;
        stdout.write(mime_str.as_bytes())?;
        stdout.write(b"\n")?;
    }

    Ok(())
}

fn main() {
    let app = ui::app::App::new();
    app.run();
}
