use std::error::Error;
use std::path::PathBuf;

use gio;
use gtk;
use gtk::prelude::*;

use util;

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
        for entry in util::read_dir(&self.current_dir)? {
            // This unwrap()s seem to never fail – even the '[unknown]' string,
            // which isn't in the a/b format, doesn't result in None here.
            let mut icon = gio::content_type_get_icon(&entry.mime_str).unwrap();

            // replace ugly "missing" icon – TODO: there's probably a better solution for this
            if icon.to_string().unwrap().ends_with("-x-generic") {
                icon = gio::Icon::new_for_string("gtk-file").unwrap();
            }

            self.tree_store.insert_with_values(
                // parent
                None,
                // position
                None,
                // colums
                &[0, 1, 2],
                // values
                &[&entry.name, &icon, &entry.mime_str],
            );
        }

        Ok(())
    }
}
