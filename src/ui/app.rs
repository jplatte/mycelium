use std::env;

use gio;
use gtk;
use gtk::prelude::*;

use super::files_tree_view::FilesTreeView;

const APP_ID: &str = "org.jplatte.mycelium";

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

        let files_tree_view_l = FilesTreeView::new(env::home_dir().unwrap());
        let files_gtk_tree_view_l = files_tree_view_l.tree_view.clone();

        gtk_app.connect_activate(move |app| {
            // Set up shutdown callback
            let window: gtk::Window = gtk_builder
                .get_object("main_window")
                .expect("Couldn't find main_window in ui file.");

            window.connect_delete_event(clone!(app => move |_, _| {
                app.quit();
                Inhibit(false)
            }));

            let files_scrolled_window_l: gtk::ScrolledWindow = gtk_builder
                .get_object("files_scrolled_window_l")
                .expect("Couldn't find files_scrolled_window_l in ui file.");
            files_scrolled_window_l.add(&files_gtk_tree_view_l);

            // Associate window with the Application and show it
            window.set_application(Some(app));
            window.show_all();
        });

        //gtk_app.connect_handle_local_options(
        // TODO: first positional argument = initial directory if present
        //);

        App {
            gtk_app,
            files_tree_view_l,
            files_tree_view_r: None,
        }
    }

    pub fn run(self) {
        // Convert the args to a Vec<&str>. Application::run requires argv as &[&str]
        // and envd::args() returns an iterator of Strings.
        let args = env::args().collect::<Vec<_>>();
        let args_refs = args.iter().map(|x| &x[..]).collect::<Vec<_>>();

        // Run the main loop.
        self.gtk_app.run(&args_refs);
    }
}
