use gtk::prelude::*;
use gtk::{
    self, Application, ApplicationWindow, Assistant, AssistantPageType, Box, Button, Image, Label,
    Orientation,
};

pub fn page_1(main: &Assistant) -> Box {
    let button = Button::builder()
        .label("test")
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    // new box with button
    let gbox = Box::builder().orientation(Orientation::Horizontal).build();
    gbox.append(&button);
    main.append_page(&gbox);
    main.set_page_type(&gbox, AssistantPageType::Intro);
    main.set_page_complete(&gbox, true);

    // return the gbox
    gbox
}

pub fn page_2(main: &Assistant) -> Box {
    let label = Label::builder()
        .label("Welcome to Ultramarine Linux!")
        .margin_top(32)
        .margin_bottom(32)
        .build();
    let image = Image::builder()
        .icon_name("fedora-logo-icon")
        .pixel_size(200)
        .build();
    // new box
    let gbox = Box::builder().orientation(Orientation::Vertical).build();
    gbox.append(&label);
    gbox.append(&image);
    // return the gbox
    main.append_page(&gbox);
    main.set_page_type(&gbox, AssistantPageType::Content);
    main.set_page_complete(&gbox, true);
    gbox
}
