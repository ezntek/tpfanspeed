use glib::Object;

mod imp;

glib::wrapper! {
    pub struct TemperatureBox(ObjectSubclass<imp::TemperatureBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl TemperatureBox {
    pub fn new(core_id: u8) -> Self {
        let obj: TemperatureBox = Object::builder().property("core-id", core_id).build();
        obj
    }
}
