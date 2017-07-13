use std::error::Error;
use std::fs;
use std::path::PathBuf;

use mime_guess;
use gio;
use gtk;
use gtk::prelude::*;

#[derive(Debug)]
pub struct FilesTreeView {
    pub current_dir: PathBuf,
    pub tree_store: gtk::TreeStore,
    pub tree_view: gtk::TreeView,
}

impl FilesTreeView {
    pub fn new(dir: PathBuf) -> FilesTreeView {
        let tree_view = FilesTreeView {
            current_dir: dir,
            tree_store: gtk::TreeStore::new(&[
                gtk::Type::String,        // name
                gio::Icon::static_type(), // icon
                gtk::Type::String,        // mime-type
            ]),
            tree_view: gtk::TreeView::new(),
        };

        tree_view.build_ui();
        // TODO: Show error message if update() returns Err(_)
        tree_view.update();
        tree_view
    }

    fn build_ui(&self) {
        self.tree_view.set_model(&self.tree_store);
        self.tree_view.set_headers_visible(true);
        self.tree_view.set_show_expanders(false);

        // Name column
        {
            let column = gtk::TreeViewColumn::new();
            column.set_title("Name");

            // 8px (2 * 4px) padding on the left
            let pad_left = gtk::CellRendererText::new();
            pad_left.set_padding(4, 0);
            column.pack_start(&pad_left, false);

            let icon_renderer = gtk::CellRendererPixbuf::new();
            column.pack_start(&icon_renderer, false);
            column.add_attribute(&icon_renderer, "gicon", 1);

            let name_renderer = gtk::CellRendererText::new();

            // 8px padding between icon and text, text and column end
            let (_, y_pad) = name_renderer.get_padding();
            name_renderer.set_padding(8, y_pad);

            column.pack_start(&name_renderer, true);
            column.add_attribute(&name_renderer, "text", 0);

            self.tree_view.append_column(&column);
        }

        // MIME-type column
        {
            let column = gtk::TreeViewColumn::new();
            column.set_title("MIME-Type");

            let mime_type_renderer = gtk::CellRendererText::new();
            column.pack_start(&mime_type_renderer, true);
            column.add_attribute(&mime_type_renderer, "text", 2);

            self.tree_view.append_column(&column);
        }
    }

    fn update(&self) -> Result<(), Box<Error>> {
        // TODO: sort dir entries. alphabetical + directories first.
        for e in fs::read_dir(&self.current_dir)? {
            let entry = match e {
                Ok(entry) => entry,
                Err(_) => {
                    // TODO: Show error message?
                    continue
                }
            };

            let file_name = entry.file_name().to_string_lossy().into_owned();

            // TODO: initial display with mime type guessing based on file extensions determining
            //       strategy, then refinement by looking at the files – visible ones first
            //       (or only visible ones?) if possible
            let mime_str = if entry.path().is_dir() {
                "inode/directory".to_owned()
            } else {
                file_name
                    .bytes()
                    .rev()
                    .position(|x| x == b'.')
                    .map(|rev_idx| file_name.len() - rev_idx)
                    .and_then(|idx| if idx == 1 {
                        // If the only '.' in the file name is at the start, regard it as having no
                        // file extension
                        None
                    } else {
                        Some(&file_name[idx..])
                    })
                    .and_then(|x| mime_guess::get_mime_type_opt(x))
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "[unknown]".to_owned())
            };

            // This unwrap()s seem to never fail – even the '[unknown]' string,
            // which isn't in the a/b format, doesn't result in None here.
            let mut icon = gio::content_type_get_icon(&mime_str).unwrap();

            // replace ugly "missing" icon – TODO: there's probably a better solution for this
            if icon.to_string().unwrap().ends_with("-x-generic") {
                icon = gio::Icon::new_for_string("gtk-file").unwrap();
            }

            self.tree_store.insert_with_values(
                /* parent   */ None,
                /* position */ None, // append
                /* colums   */ &[0, 1, 2],
                /* values   */ &[&file_name, &icon, &mime_str],
            );
        }

        Ok(())
    }
}
