use gtk::prelude::*;
use gtk::{
    self, Application, ApplicationWindow, Assistant, AssistantPage, AssistantPageType, Box, Button,
    Orientation,
};

use super::page1::page_1;
use super::page1::page_2;

pub fn window(app: &Application) {
    // New Assistant
    let window = Assistant::builder()
        .application(app)
        .title("Welcome to Ultramarine")
        .build();

    page_1(&window);
    page_2(&window);

    // exit button
    window.connect_cancel(|ass| {
        ass.destroy(); // Ass destroyed
    });

    window.connect_close(|ass| {
        ass.destroy(); // Ass destroyed
    });
    window.set_title(Some("Welcome to Ultramarine"));
    window.present();
}
