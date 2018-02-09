extern crate futures_glib;
extern crate gitter;
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::Widget;

mod core;

fn main() {
    core::win::Win::run(()).unwrap();
}

// TODO: next features with css.
// fn style() {
//     let css = gtk::CssProvider::new();
//     css.load_from_resource("resources/style.css");
//     gtk::StyleContext::add_provider_for_screen(&gdk::Screen::get_default().unwrap(), &css, 600);
// }
