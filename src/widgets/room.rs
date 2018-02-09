#![allow(unused_imports)]
#![allow(dead_code)]

use gtk;
use gtk::{BuilderExt, EventBoxExt, ImageExt};

use model::Model;

/*
* Room row widget for the room sidebar.
* Shows the avatar, title and the number of unread messages.
* +-----+----------------------+-----+
* |Image|   Room Title         |  3  |
* +-----+----------------------+-----+
*/

pub struct Room {
    pub builder: gtk::Builder,
    pub icon: gtk::Image,
    pub title: gtk::Label,
    pub notifications: gtk::Label,
    pub progress: gtk::ProgressBar,
    pub widget: gtk::EventBox,
}

impl Room {
    pub fn new() -> Room {
        let icon = builder.get_object("user_menu_button").unwrap();
        let title = builder.get_object("").unwrap();
        let notifications = builder.get_object("").unwrap();
        let progress = builder.get_object("").unwrap();
        let widget = builder.get_object("").unwrap();

        Room {
            builder,
            icon,
            title,
            notifications,
            progress,
            widget,
        }
    }
}
