use gtk;

#[allow(dead_code)]
#[derive(Msg)]
pub enum Msg {
    Send(String),
    SelectRoom(Option<gtk::ListBoxRow>),
    Update(()),
    Quit,
}

#[allow(dead_code)]
#[derive(Msg, Clone)]
pub enum RoomState {
    Room,
    NoRoom,
    Loading,
}

#[allow(dead_code)]
#[derive(Msg, Clone)]
pub enum AppState {
    Chat,
    Directory,
    Loading,
}
