use std::time::Duration;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

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
    // create button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // add event listener to button
    button.connect_clicked(move |button| {
        // change button to label
        button.set_label("Hello World!");
        std::thread::sleep(Duration::from_secs(10));
        button.set_label("Press me!");
    });
    // create a window and title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello World Rust Gtk4")
        .child(&button)
        .build();

    // display our window
    window.present();
}