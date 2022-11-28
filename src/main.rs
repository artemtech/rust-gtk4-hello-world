use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "id.artemtech.gtk_rs.HelloWorld";

fn main() {
    // Create app wrapper
    let app = Application::builder().application_id(APP_ID).build();

    // run app
    app.run();
}
