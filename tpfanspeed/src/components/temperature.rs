use gtk::prelude::*;

pub struct TemperatureFrame {}

mod imp {
    use gtk::{prelude::*, subclass::prelude::*};
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct TemperatureBox {
        progressbar: RefCell<Option<gtk::ProgressBar>>,
        label: RefCell<Option<gtk::Label>>,
        core_id: u16,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TemperatureBox {
        const NAME: &'static str = "TemperatureBox";
        type Type = super::TemperatureBox;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for TemperatureBox {
        fn constructed(&self) {
            self.obj().set_orientation(gtk::Orientation::Horizontal);
            self.obj().set_spacing(5);
            self.obj().set_margin_top(10);

            let progressbar = gtk::ProgressBar::builder()
                .text(format!("Core {}", self.core_id))
                .show_text(true)
                .margin_start(5)
                .margin_bottom(5)
                .hexpand(true)
                .margin_end(125)
                //.halign(gtk::Align::Start)
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

    impl WidgetImpl for TemperatureBox {}

    impl BoxImpl for TemperatureBox {}
}

glib::wrapper! {
    pub struct TemperatureBox(ObjectSubclass<imp::TemperatureBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl TemperatureFrame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn setup_ui(&self) -> impl IsA<gtk::Widget> {
        let frame = gtk::Frame::new(Some("Temperatures"));
        frame.set_vexpand(true);
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

        for core in 0..28 as u16 {}

        let scrollwindow = gtk::ScrolledWindow::builder().hexpand(true).build();
        scrollwindow.set_child(Some(&vbox));

        frame.set_child(Some(&scrollwindow));

        frame
    }
}
