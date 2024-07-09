use glib::{property, Properties};
use gtk::{prelude::*, subclass::prelude::*};
use std::cell::RefCell;

#[derive(Default, Properties)]
#[properties(wrapper_type = super::TemperatureBox)]
pub struct TemperatureBox {
    progressbar: RefCell<Option<gtk::ProgressBar>>,
    label: RefCell<Option<gtk::Label>>,

    #[property(get, set = Self::set_core_name, type = String, name = "core-name", construct)]
    core_name: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for TemperatureBox {
    const NAME: &'static str = "TemperatureBox";
    type Type = super::TemperatureBox;
    type ParentType = gtk::Box;
}

#[glib::derived_properties]
impl ObjectImpl for TemperatureBox {
    fn constructed(&self) {
        self.parent_constructed();

        self.obj().set_orientation(gtk::Orientation::Horizontal);
        self.obj().set_spacing(5);
        self.obj().set_margin_top(10);

        let progressbar = gtk::ProgressBar::builder()
            .text(self.obj().core_name())
            .show_text(true)
            .margin_start(5)
            .margin_bottom(5)
            .hexpand(true)
            .margin_end(125)
            .build();
        progressbar.set_fraction(0.35);

        let label = gtk::Label::builder()
            .label("42°C")
            .halign(gtk::Align::End)
            .valign(gtk::Align::Start)
            .hexpand(true)
            .margin_end(5)
            .build();

        self.obj().append(&progressbar);
        self.obj().append(&label);
        *self.progressbar.borrow_mut() = Some(progressbar);
        *self.label.borrow_mut() = Some(label);
    }
}

impl TemperatureBox {
    pub fn set_core_name(&self, value: String) {
        *self.core_name.borrow_mut() = value;
    }
}

impl WidgetImpl for TemperatureBox {}

impl BoxImpl for TemperatureBox {}
