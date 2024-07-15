mod components;

use gtk::prelude::*;

pub struct DashboardPage {}

pub struct AboutPage {}

#[derive(Default)]
pub struct TPFanSpeed {}

impl DashboardPage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn setup_ui(&self) -> impl IsA<gtk::Widget> {
        let temp_frame = components::TemperatureFrame::new();

        let fancontrol_frame = components::FancontrolFrame::new();

        let log_frame = gtk::Frame::builder()
            .label("Log")
            .hexpand(true)
            .margin_top(10)
            .build();

        log_frame.set_child(Some(&gtk::Label::new(Some(
            "the contents of the log frame",
        ))));

        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .vexpand(true)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .margin_bottom(10)
            .build();
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .margin_start(10)
            .build();

        vbox.append(&fancontrol_frame.setup_ui());
        vbox.append(&log_frame);
        //hbox.append(&temp_frame.setup_ui());
        hbox.append(&vbox);

        hbox
    }
}

impl AboutPage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn setup_ui(&self) -> impl IsA<gtk::Widget> {
        gtk::Label::new(Some("about page haha"))
    }
}

impl TPFanSpeed {
    pub fn new() -> Self {
        Self {}
    }

    pub fn setup_ui(&self) -> impl IsA<gtk::Widget> {
        let notebook = gtk::Notebook::new();
        let dashboard = DashboardPage::new();
        let about = AboutPage::new();

        notebook.append_page(
            &dashboard.setup_ui(),
            Some(&gtk::Label::new(Some("Dashboard"))),
        );

        notebook.append_page(&about.setup_ui(), Some(&gtk::Label::new(Some("About"))));
        notebook
    }
}
