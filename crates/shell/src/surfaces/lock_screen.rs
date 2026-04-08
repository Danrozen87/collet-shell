//! Lock Screen — full-screen overlay surface.

use gtk::prelude::*;
use gtk_layer_shell::LayerShell;
use wry::{WebViewBuilder, WebViewBuilderExtUnix};

pub fn create() -> gtk::Window {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.init_layer_shell();
    window.set_layer(gtk_layer_shell::Layer::Overlay);
    window.set_anchor(gtk_layer_shell::Edge::Top, true);
    window.set_anchor(gtk_layer_shell::Edge::Bottom, true);
    window.set_anchor(gtk_layer_shell::Edge::Left, true);
    window.set_anchor(gtk_layer_shell::Edge::Right, true);
    window.set_exclusive_zone(-1);
    window.set_keyboard_interactivity(true);
    window.set_namespace("collet-lock-screen");

    window.set_decorated(false);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&container);

    let html = crate::render::render_lock_screen();
    let _webview = WebViewBuilder::new()
        .with_transparent(true)
        .with_html(&html)
        .with_ipc_handler(|msg: wry::http::Request<String>| {
            eprintln!("[collet-shell] Lock screen IPC: {}", msg.body());
        })
        .build_gtk(&container)
        .expect("Failed to create lock screen webview");

    eprintln!("[collet-shell] Lock screen webview created");
    window
}
