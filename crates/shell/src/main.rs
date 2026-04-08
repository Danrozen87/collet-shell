//! Collet OS Shell
//!
//! On Linux/Wayland: layer-shell surfaces (dock, control bar)
//! On macOS: windowed preview for development (same HTML/CSS)

mod ipc;
mod render;
mod system;

#[cfg(target_os = "linux")]
mod surfaces;

#[cfg(target_os = "linux")]
fn main() {
    use gtk::prelude::*;

    gtk::init().expect("Failed to initialize GTK");

    let dock = surfaces::dock::create();
    let control_bar = surfaces::control_bar::create();

    dock.show_all();
    control_bar.show_all();

    gtk::main();
}

/// macOS preview — two windows showing dock and control bar.
/// Same HTML/CSS/components as production Linux surfaces.
#[cfg(not(target_os = "linux"))]
fn main() {
    use tao::event::{Event, WindowEvent};
    use tao::event_loop::{ControlFlow, EventLoop};
    use tao::window::WindowBuilder;
    use wry::WebViewBuilder;

    let event_loop = EventLoop::new();

    // Full desktop preview — dock + control bar in one window
    let window = WindowBuilder::new()
        .with_title("Collet OS — Shell Preview")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let preview_html = render::render_preview();

    // Debug: dump HTML to file for Safari inspection
    std::fs::write("preview/live.html", &preview_html).ok();

    // Also dump settings page for preview
    let settings_html = render::render_settings();
    std::fs::write("preview/settings.html", &settings_html).ok();

    let lock_html = render::render_lock_screen();
    std::fs::write("preview/lock.html", &lock_html).ok();
    let _webview = WebViewBuilder::new()
        .with_html(&preview_html)
        .with_ipc_handler(|msg: wry::http::Request<String>| {
            let response = ipc::handler::handle(msg.body());
            eprintln!("[collet-shell] IPC: {} → {}", msg.body(), response);
        })
        .build(&window)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
