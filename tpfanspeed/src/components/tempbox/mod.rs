use glib::Object;

mod imp;

glib::wrapper! {
    pub struct TemperatureBox(ObjectSubclass<imp::TemperatureBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl TemperatureBox {
    pub fn new(core_name: String) -> Self {
        let obj: TemperatureBox = Object::builder().property("core-name", core_name).build();
        obj
    }
}
