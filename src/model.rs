use gtk;
use gtk::{EntryExt, ListBoxExt, StackExt, WidgetExt};
use relm::Relm;
use win::Win;
use msg::Msg;
use msg::AppState;

use std::collections::HashMap;

pub struct Model {
    pub builder: gtk::Builder,
    pub gtk_app: gtk::ApplicationWindow,
    // pub username: String,
    pub current_room: String,
    pub rooms: HashMap<gtk::ListBoxRow, String>,
    pub room_list_box: gtk::ListBox,
    pub message_text: gtk::Entry,
    pub messages_box: gtk::ListBox,
}

impl Model {
    pub fn new(relm: &Relm<Win>) -> Self {
        let builder = gtk::Builder::new_from_file("src/ui/app_window.glade");
        let gtk_app: gtk::ApplicationWindow = builder.get_object("app_window").unwrap();
        connect!(
            relm,
            gtk_app,
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

        let message_text: gtk::Entry = builder.get_object("msg_entry").unwrap();
        let message_text_ = message_text.clone();
        connect!(
            relm,
            message_text,
            connect_activate(_),
            Msg::Send(message_text_.get_text().unwrap_or_default())
        );

        let messages_box: gtk::ListBox = builder.get_object("message_list").unwrap();

        Model {
            message_text,
            messages_box,
            room_list_box,
            builder,
            gtk_app,
            current_room: String::new(),
            rooms: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn set_state(state: AppState) {
        let widget_title = match state {
            AppState::Chat => "chat",
            AppState::Directory => "directory",
            AppState::Loading => "loading",
        };

        let builder = gtk::Builder::new();
        builder
            .get_object::<gtk::Stack>("main_content_stack")
            .expect("Can't find main_content_stack")
            .set_visible_child_name(widget_title);
    }
}
