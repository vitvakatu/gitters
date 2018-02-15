use gitter::{self, Gitter};
use gtk;
use gtk::{ContainerExt, EntryExt, LabelExt, WidgetExt};
use relm::Update;
use relm::{Relm, Widget};

use std::time::Duration;
use std::env;

use core::msg::{AppState, Msg};
use core::model::Model;
use futures_glib;

pub struct Win {
    model: Model,
    window: gtk::ApplicationWindow,
    api: Gitter<'static>,
}

impl Win {
    fn create_msg_label(text: &str) -> gtk::Label {
        let label = gtk::Label::new(Some(text));
        label.set_selectable(true);
        label.set_line_wrap(true);
        label.set_margin_top(5);
        label.set_margin_bottom(5);
        label.set_margin_left(5);
        label.set_justify(gtk::Justification::Fill);
        label.set_halign(gtk::Align::Start);
        label
    }

    fn update_impl(&mut self) {
        let rooms = self.api.get_rooms().unwrap();
        for room in self.model.room_list_box.get_children() {
            self.model.room_list_box.remove(&room);
        }
        for room in rooms {
            let unread = room.unread_items;
            let text = if unread > 0 {
                format!("({}) {}", unread, room.name)
            } else {
                room.name
            };
            let text: &str = &text;
            let label = gtk::Label::new(Some(text));
            let row = gtk::ListBoxRow::new();
            row.add(&label);
            self.model.room_list_box.add(&row);
            self.model.rooms.insert(row, room.id);
        }
        self.model.room_list_box.show_all();
        self.model.set_state(AppState::Chat);

        for message in self.model.messages_box.get_children() {
            self.model.messages_box.remove(&message);
        }
        let room_id = self.model.current_room.clone();
        self.model.current_room = room_id.clone();
        let pagination = gitter::Pagination {
            skip: 0,
            before_id: None,
            after_id: None,
            limit: 15,
            query: None,
        };
        if let Ok(messages) = self.api.get_messages(&room_id, Some(pagination)) {
            let message_ids: Vec<_> = messages.iter().map(|m| m.id.clone()).collect();
            let user_id = self.api.get_user().unwrap().id;
            let _ = self.api
                .mark_messages_as_read(&user_id, &room_id, &message_ids);
            for message in messages {
                let text: &str = &format!("{}: {}", message.from.username, message.text);
                let label = Self::create_msg_label(text);
                self.model.messages_box.add(&label);
            }
            self.model.messages_box.show_all();
        } else {
            // self.model.set_state(AppState::Loading);
            println!("Could not get messages");
        }
    }
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        Model::new(relm)
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Send(ref msg) => {
                self.api
                    .send_message(&self.model.current_room, msg)
                    .unwrap();
                self.model.message_text.get_text().unwrap_or_default();
            }
            Msg::Update(()) => {
                self.update_impl();
            }
            Msg::SelectRoom(Some(ref row)) => {
                let needs_update = {
                    let room_id = &self.model.rooms[row];
                    let needs_update = self.model.current_room == room_id.as_str();
                    self.model.current_room = room_id.clone();
                    needs_update
                };
                if needs_update {
                    self.update_impl();
                }
            }
            Msg::Quit => {
                gtk::main_quit();
            }
            _ => {}
        }
    }

    // TODO: Rewrite.
    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let update_stream = futures_glib::Interval::new(Duration::from_secs(3));
        relm.connect_exec_ignore_err(update_stream, Msg::Update);
    }
}

impl Widget for Win {
    type Root = gtk::ApplicationWindow;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(_: &Relm<Self>, model: Self::Model) -> Self {
        let token = env::var("GITTER_API_TOKEN").expect("Needs GITTER_API_TOKEN env var");
        let api = Gitter::new(token).unwrap();

        let window = model.gtk_app.clone();
        window.show_all();

        Win { api, model, window }
    }
}
