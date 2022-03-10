use gtk::ffi::GtkAssistantPage;
use gtk::prelude::*;
use gtk::{
    self, Application, ApplicationWindow, Assistant, AssistantPage, AssistantPageType, Box, Button,
    Orientation,
};

use gtk::gio;

mod window;
use window::window;
fn main() {
    gio::resources_register_include!("ultramarine_welcome.gresource")
        .expect("Failed to register resource");

    let app = Application::builder()
        .application_id("org.ultramarine.welcome")
        .build();
    // Connect to "activate" signal of `app`
    app.connect_activate(window);

    // Run the application
    app.run();
}
