use std::borrow::BorrowMut;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct TemperatureWidget {
    core_id: u16,
    progressbar: RefCell<Option<gtk::ProgressBar>>,
    label: RefCell<Option<gtk::Label>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TemperatureWidget {
    const NAME: &'static str = "TemperatureWidget";
    type Type = super::TemperatureWidget;
    type ParentType = gtk::Box;
}

impl ObjectImpl for TemperatureWidget {
    fn constructed(&self) {
        self.parent_constructed();

        let progress_bar = gtk::ProgressBar::builder()
            .text(format!("Core {}", self.core_id))
            .show_text(true)
            .margin_start(5)
            .margin_bottom(5)
            .hexpand(true)
            .margin_end(125)
            //.halign(gtk::Align::Start)
            .build();

        let label = gtk::Label::builder()
            .label("42°C")
            .halign(gtk::Align::End)
            .valign(gtk::Align::Start)
            .hexpand(true)
            .margin_end(5)
            .build();

        self.obj().set_orientation(gtk::Orientation::Vertical);
        self.obj().append(&progress_bar);
        self.obj().append(&label);

        *self.progressbar.borrow_mut() = Some(progress_bar);
        *self.label.borrow_mut() = Some(label);
    }
}

impl WidgetImpl for TemperatureWidget {}

impl BoxImpl for TemperatureWidget {}
