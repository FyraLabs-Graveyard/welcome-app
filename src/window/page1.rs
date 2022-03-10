use gtk::glib::GString;
use gtk::prelude::*;
use gtk::{
    self, Application, ApplicationWindow, Assistant, AssistantPageType, Box, Button, Image, Label,
    Orientation,
};
use gtk::gio;
use sourceview;

pub fn page_1(main: &Assistant) -> Box {
    let label = Label::builder()
    // this is pango markup btw, see https://docs.gtk.org/Pango/pango_markup.html
        .label("<span size=\"xx-large\">Welcome</span>")
        .margin_top(32)
        .margin_bottom(128)
        // set text size
        .use_markup(true)
        .justify(gtk::Justification::Center)

        .build();
    let image = Image::builder()
        .icon_name("fedora-logo-icon")
        .pixel_size(200)
        .build();
    // new box
    let gbox = Box::builder().orientation(Orientation::Vertical).build();
    gbox.append(&label);
    gbox.append(&image);

    gbox.append(
        &Label::builder()
            // insert some intro text
            .label("You are only a few steps away from finishing your Ultramarine installation.")
            .margin_top(32)
            .margin_bottom(32)
            .build(),
    );
    // justify gbox to center of window
    gbox.set_halign(gtk::Align::Center);
    gbox.set_valign(gtk::Align::Center);

    main.append_page(&gbox);
    main.set_page_type(&gbox, AssistantPageType::Content);
    main.set_page_complete(&gbox, true);
    main.set_page_title(&gbox, "Welcome");
    gbox
}

pub fn page_2(main: &Assistant) -> Box {
    let gbox = Box::builder().orientation(Orientation::Vertical).build();
    gbox.append(
        &Label::builder()
            // insert some intro text
            .label("You can choose to install Ultramarine on your own computer or on a server.")
            .margin_top(32)
            .margin_bottom(32)
            .build(),
    );

    // Embed a terminal inside
    let terminal = sourceview::

    // make a shell session
    gbox.append(&terminal);
    // return the gbox
    main.append_page(&gbox);
    main.set_page_type(&gbox, AssistantPageType::Content);
    main.set_page_complete(&gbox, true);
    gbox
}
