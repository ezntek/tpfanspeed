use gtk::prelude::*;

const APP_ID: &'static str = "com.ezntek.tpfanspeed";

fn on_activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);

    let tpfs = tpfanspeed::TPFanSpeed::new();

    window.set_child(Some(&tpfs.setup_ui()));
    window.present();
}

fn main() {
    //let app = gtk::Application::builder().application_id(APP_ID).build();

    //app.connect_activate(on_activate);
    //app.run();
    let temps = libtpfanspeed::get_temps();
    println!("{:?}", temps);
}
