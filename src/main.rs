#![feature(conservative_impl_trait)]

extern crate gio;
extern crate gtk;
extern crate mime_guess;

// This apparently needs to come before the app module declaration for that to
// be able to use the macro
#[macro_use]
mod util;

mod ui {
    pub mod app;

    mod files_tree_view;
}

fn main() {
    let app = ui::app::App::new();
    app.run();
}
