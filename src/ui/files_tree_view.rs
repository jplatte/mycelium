use gtk;
use gtk::prelude::*;

#[derive(Debug)]
pub struct FilesTreeView {
    pub gtk_tree_view: gtk::TreeView,
}

impl FilesTreeView {
    pub fn new() -> FilesTreeView {
        FilesTreeView {
            gtk_tree_view: gtk::TreeView::new(),
        }
    }
}
