#![allow(dead_code)]
use gtk;

#[derive(Msg)]
pub enum Msg {
    Send(String),
    SelectRoom(Option<gtk::ListBoxRow>),
    Update(()),
    Quit,
}

#[derive(Msg)]
#[repr(u8)]
pub enum AppState {
    Chat,
    Directory,
    Loading,
    Room,
    NoRoom,
    Connect,
}
