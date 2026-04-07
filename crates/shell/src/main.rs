//! Collet OS Shell — Proof of Concept
//!
//! Creates two layer-shell surfaces with wry webviews:
//! 1. Control Bar (top-right pill) — system status
//! 2. Dock (bottom-center) — app launcher + search palette
//!
//! This is the 50-line proof-of-concept to validate:
//! - gtk-layer-shell + wry webview initialization timing
//! - Transparent, borderless layer surfaces on COSMIC/Wayland
//! - HTML rendering from Collet design system tokens

mod surfaces;

use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    // Create the dock surface (bottom, centered)
    let dock = surfaces::dock::create();

    // Create the control bar surface (top-right, pill)
    let control_bar = surfaces::control_bar::create();

    // Show both surfaces
    dock.show_all();
    control_bar.show_all();

    gtk::main();
}
