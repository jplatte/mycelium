use std::env;
use std::thread;

use gio;
use gtk;
use gtk::prelude::*;

use super::files_tree_view::FilesTreeView;

const APP_ID: &str = "org.jplatte.mycelium";
const INITIAL_DIR: &str = "/"; // TODO

pub struct App {
    /// GTK Application which runs the main loop.
    gtk_app: gtk::Application,
    files_tree_view_l: FilesTreeView,
    files_tree_view_r: Option<FilesTreeView>,
}

impl App {
    /// Create an App instance
    pub fn new() -> App {
        let gtk_app = gtk::Application::new(Some(APP_ID), gio::ApplicationFlags::empty()).unwrap();

        let gtk_builder = gtk::Builder::new_from_string(include_str!("../../res/ui.glade"));

        let files_tree_view_l = FilesTreeView::new();
        let files_gtk_tree_view_l = files_tree_view_l.gtk_tree_view.clone();

        gtk_app.connect_activate(move |app| {
            // Set up shutdown callback
            let window: gtk::Window = gtk_builder.get_object("main_window")
                .expect("Couldn't find main_window in ui file.");

            window.connect_delete_event(clone!(app => move |_, _| {
                app.quit();
                Inhibit(false)
            }));

            let treeview_paned: gtk::Paned = gtk_builder.get_object("treeview_paned")
                .expect("Couldn't find treeview_paned in ui file.");

            let model = gtk::TreeStore::new(&[gtk::Type::String, gtk::Type::String]);
            files_gtk_tree_view_l.set_model(&model);
            files_gtk_tree_view_l.set_headers_visible(true);

            let column = gtk::TreeViewColumn::new();
            let name_renderer = gtk::CellRendererText::new();
            column.pack_start(&name_renderer, true);
            column.add_attribute(&name_renderer, "text", 0);

            files_gtk_tree_view_l.append_column(&column);

            treeview_paned.add1(&files_gtk_tree_view_l);

            // Associate window with the Application and show it
            window.set_application(Some(app));
            window.show_all();
        });

        App {
            gtk_app,
            files_tree_view_l,
            files_tree_view_r: None,
        }
    }

    pub fn run(self) {
        // Convert the args to a Vec<&str>.  Application::run requires argv as &[&str]
        // and envd::args() returns an iterator of Strings.
        let args = env::args().collect::<Vec<_>>();
        let args_refs = args.iter().map(|x| &x[..]).collect::<Vec<_>>();

        // Run the main loop.
        self.gtk_app.run(args_refs.len() as i32, &args_refs);
    }
}
