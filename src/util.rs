use std::fs;
use std::io;
use std::path::Path;

use mime_guess;

// from https://stackoverflow.com/a/43992218/1592377
#[macro_export]
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

#[derive(Debug)]
pub struct DirEntry {
    pub name: String,
    pub mime_str: String,
    pub is_directory: bool,
}

// TODO: Remove sorting from here and do it via the TreeView instead?
pub fn read_dir<P: AsRef<Path>>(directory: P) -> io::Result<Vec<DirEntry>> {
    let mut entries = fs::read_dir(directory.as_ref())?
        // TODO: Instead of just Result::ok, match and do something for Err(_)?
        .flat_map(Result::ok)
        .map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();

            // TODO: initial display with mime type guessing based on file extensions determining
            //       strategy, then refinement by looking at the files – visible ones first
            //       (or only visible ones?) if possible
            let mime_str = if entry.path().is_dir() {
                "inode/directory".to_owned()
            } else {
                name
                    .bytes()
                    .rev()
                    .position(|x| x == b'.')
                    .map(|rev_idx| name.len() - rev_idx)
                    .and_then(|idx| if idx == 1 {
                        // If the only '.' in the file name is at the start, regard it as having no
                        // file extension
                        None
                    } else {
                        Some(&name[idx..])
                    })
                    .and_then(|x| mime_guess::get_mime_type_opt(x))
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "[unknown]".to_owned())
            };

            DirEntry {
                name,
                mime_str,
                is_directory: entry.path().is_dir(),
            }
        })
        .collect::<Vec<_>>();

    entries.sort_unstable_by(|a, b| {
        Ord::cmp(&a.is_directory, &b.is_directory)
            .reverse()
            .then_with(|| {
                // TODO: normalization?
                Ord::cmp(&a.name.to_lowercase(), &b.name.to_lowercase())
            })
    });

    Ok(entries)
}
