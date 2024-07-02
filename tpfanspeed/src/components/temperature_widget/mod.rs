mod imp;

glib::wrapper! {
    pub struct TemperatureWidget(ObjectSubclass<imp::TemperatureWidget>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}
