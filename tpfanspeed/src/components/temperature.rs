use gtk::prelude::*;

pub struct TemperatureFrame {}

pub struct TemperatureUnit {
    core_id: u16,
}

impl TemperatureUnit {
    pub fn new(core_id: u16) -> Self {
        Self { core_id }
    }

    pub fn setup_ui(&self) -> impl IsA<gtk::Widget> {
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        hbox.set_margin_top(10);

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
        hbox.append(&progressbar);
        hbox.append(&label);

        hbox
    }
}

impl TemperatureFrame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn setup_ui(&self) -> impl IsA<gtk::Widget> {
        let frame = gtk::Frame::new(Some("Temperatures"));
        frame.set_vexpand(true);
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

        for core in 0..28 as u16 {
            let unit = TemperatureUnit::new(core);
            vbox.append(&unit.setup_ui());
        }

        let scrollwindow = gtk::ScrolledWindow::builder().hexpand(true).build();
        scrollwindow.set_child(Some(&vbox));

        frame.set_child(Some(&scrollwindow));

        frame
    }
}
