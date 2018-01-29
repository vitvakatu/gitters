#![feature(use_nested_groups)]
extern crate futures_glib;
extern crate gitter;
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::Widget;

mod msg;
mod model;
mod win;

fn main() {
    win::Win::run(()).unwrap();
}
