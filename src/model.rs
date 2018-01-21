use gtk::{self, ButtonExt, EntryExt, ListBoxExt, WidgetExt};
use relm::Relm;
use win::Win;
use msg::Msg;

use std::collections::HashMap;

pub struct Model {
    pub current_room: String,
    pub rooms: HashMap<gtk::ListBoxRow, String>,
    pub room_list_box: gtk::ListBox,
    pub message_text: gtk::Entry,
    pub messages_box: gtk::Box,
    pub builder: gtk::Builder,
}

impl Model {
    pub fn new(relm: &Relm<Win>) -> Self {
        let builder = gtk::Builder::new();
        builder.add_from_file("src/ui/layout.glade").unwrap();
        let window: gtk::ApplicationWindow = builder.get_object("window").unwrap();
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), gtk::Inhibit(false))
        );

        let room_list_box: gtk::ListBox = builder.get_object("room_list_box").unwrap();
        let list_box = room_list_box.clone();
        connect!(
            relm,
            room_list_box,
            connect_row_selected(_, _),
            Msg::SelectRoom(list_box.get_selected_row())
        );

        let message_text: gtk::Entry = builder.get_object("message_text").unwrap();
        let send_button: gtk::Button = builder.get_object("send_msg_button").unwrap();
        let message_text_ = message_text.clone();
        connect!(
            relm,
            send_button,
            connect_clicked(_),
            Msg::Send(message_text_.get_text().unwrap_or_default())
        );

        let messages_box: gtk::Box = builder.get_object("messages_box").unwrap();

        Model {
            messages_box,
            message_text,
            room_list_box,
            current_room: String::new(),
            rooms: HashMap::new(),
            builder,
        }
    }
}
