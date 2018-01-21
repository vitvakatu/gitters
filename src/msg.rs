use gtk;

#[derive(Msg)]
pub enum Msg {
    Send(String),
    SelectRoom(Option<gtk::ListBoxRow>),
    Update(()),
    Quit,
}
