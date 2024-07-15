use std::{borrow::Borrow, cell::RefCell};

use glib::{property::PropertyGet, subclass::prelude::*};
use gtk::{
    prelude::*,
    subclass::{frame::FrameImpl, widget::WidgetImpl},
};
use libtpfanspeed as libtpfs;

#[derive(Default)]
pub struct TemperatureFrame {
    boxes: Vec<RefCell<super::TemperatureBox>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TemperatureFrame {
    const NAME: &'static str = "TemperatureFrame";
    type Type = super::TemperatureFrame;
    type ParentType = gtk::Frame;
}

impl ObjectImpl for TemperatureFrame {
    fn constructed(&self) {
        self.obj().set_label(Some("Temperatures"));
        self.obj().set_vexpand(true);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

        for core in libtpfs::get_cores().unwrap() {
            let _box = super::TemperatureBox::new(format!("CORE {core}"));
            let box_refcell = RefCell::new(_box);
        }

        let scrollwindow = gtk::ScrolledWindow::builder().hexpand(true).build();
        scrollwindow.set_child(Some(&vbox));

        self.obj().set_child(Some(&scrollwindow));
    }
}

impl WidgetImpl for TemperatureFrame {}

impl FrameImpl for TemperatureFrame {}
