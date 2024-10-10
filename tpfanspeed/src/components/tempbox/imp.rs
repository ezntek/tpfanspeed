use async_channel::{Receiver, Sender};
use glib::Properties;
use gtk::{prelude::*, subclass::prelude::*};
use libtpfanspeed as libtpfs;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, Properties)]
#[properties(wrapper_type = super::TemperatureBox)]
pub struct TemperatureBox {
    progressbar: RefCell<Option<gtk::ProgressBar>>,
    label: RefCell<Option<gtk::Label>>,

    #[property(get, set = Self::set_core_id, type = u8, name = "core-id", construct)]
    core_id: RefCell<u8>,

    temperature: Rc<RefCell<u32>>,
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

        // set up temperature
        // build ui
        self.obj().set_orientation(gtk::Orientation::Horizontal);
        self.obj().set_spacing(5);
        self.obj().set_margin_top(10);

        let progressbar = gtk::ProgressBar::builder()
            .text(format!("CORE {}", self.obj().core_id()))
            .show_text(true)
            .margin_start(5)
            .margin_bottom(5)
            .hexpand(true)
            .margin_end(50)
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

        let core_id = *self.core_id.borrow();
        let temperature = self.temperature.clone();
        let (sender, receiver): (Sender<u32>, Receiver<u32>) = async_channel::bounded(1);
        glib::spawn_future_local(glib::clone!(
            #[strong]
            sender,
            #[weak]
            temperature,
            async move {
                loop {
                    glib::timeout_future_seconds(1).await;
                    {
                        let temps = libtpfs::get_temps().expect("cant get temps lol");
                        let core_temp = temps.cores.get(&core_id).expect("cant get core temp lol");

                        let mut temp = RefCell::borrow_mut(&temperature);
                        *temp = core_temp.temp as u32;
                    } // drop the mutable borrow at the end

                    let temp = *RefCell::borrow(&temperature);
                    sender.send(temp).await.expect("the channel must be open");
                }
            }
        ));

        glib::spawn_future_local(glib::clone!(
            #[weak]
            label,
            #[weak]
            progressbar,
            async move {
                while let Ok(new_temp) = receiver.recv().await {
                    let newtxt = format!("{new_temp}°C");
                    label.set_text(&newtxt);

                    let percentage = new_temp as f64 / 100.0;
                    progressbar.set_fraction(percentage);
                }
            }
        ));

        *self.progressbar.borrow_mut() = Some(progressbar);
        *self.label.borrow_mut() = Some(label);
    }
}

impl TemperatureBox {
    pub fn set_core_id(&self, value: u8) {
        *self.core_id.borrow_mut() = value;
    }
}

impl WidgetImpl for TemperatureBox {}

impl BoxImpl for TemperatureBox {}
