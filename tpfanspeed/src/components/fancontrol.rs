use glib::clone;
use gtk::prelude::*;

pub struct FancontrolFrame {}

impl FancontrolFrame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn setup_ui(&self) -> impl IsA<gtk::Widget> {
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .margin_top(10)
            .build();

        let frame = gtk::Frame::builder()
            .label("Fan Control")
            .hexpand(true)
            .vexpand(true)
            .build();

        let auto_radiobutton = gtk::CheckButton::builder()
            .label("Auto (Embedded Controller Adjusts)")
            .active(true)
            .build();
        let fullspeed_radiobutton = gtk::CheckButton::builder()
            .group(&auto_radiobutton)
            .label("Full Speed")
            .build();
        let disengaged_radiobutton = gtk::CheckButton::builder()
            .group(&auto_radiobutton)
            .label("Full Speed, speed monitoring disabled")
            .build();
        let manual_radiobutton = gtk::CheckButton::builder()
            .group(&auto_radiobutton)
            .label("Manual (0-7)")
            .build();

        let slider_hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .margin_start(50)
            .margin_end(5)
            .build();

        let label = gtk::Label::new(Some("0"));
        let adj = gtk::Adjustment::new(0.0, 0.0, 8.0, 1.0, 1.0, 1.0);
        let slider = gtk::Scale::builder()
            .orientation(gtk::Orientation::Horizontal)
            .adjustment(&adj)
            .hexpand(true)
            .build();

        slider.connect_value_changed(clone!(@weak slider, @weak label => move |_| {
            label.set_text(&format!("{:.0}", slider.value()));
        }));

        slider_hbox.append(&slider);
        slider_hbox.append(&label);

        vbox.append(&auto_radiobutton);
        vbox.append(&fullspeed_radiobutton);
        vbox.append(&disengaged_radiobutton);
        vbox.append(&manual_radiobutton);
        vbox.append(&slider_hbox);

        frame.set_child(Some(&vbox));

        frame
    }
}
