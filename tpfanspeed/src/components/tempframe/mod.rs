use super::TemperatureBox;
use glib::Object;
use gtk::prelude::*;

use libtpfanspeed as libtpfs;

mod imp;

glib::wrapper! {
    pub struct TemperatureFrame(ObjectSubclass<imp::TemperatureFrame>)
        @extends gtk::Widget, gtk::Frame,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl TemperatureFrame {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
