#![allow(dead_code)]
use gtk;

#[derive(Msg)]
pub enum Msg {
    Send(String),
    SelectRoom(Option<gtk::ListBoxRow>),
    Update(()),
    Quit,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AppState {
    Chat,
    Directory,
    Loading,
}
