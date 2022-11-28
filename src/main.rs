use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "id.artemtech.gtk_rs.HelloWorld";

fn main() {
    // Create app wrapper
    let app = Application::builder().application_id(APP_ID).build();

    // call our ui builder here -- as entrypoint
    app.connect_activate(build_ui);

    // run app
    app.run();
}

fn build_ui(app: &Application) {
    // create a window and title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello World Rust Gtk4")
        .build();

    // display our window
    window.present();
}