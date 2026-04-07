//! Dock surface — bottom-center, expandable.

use gtk::prelude::*;
use gtk_layer_shell::LayerShell;
use wry::{WebViewBuilder, WebViewBuilderExtUnix};

const DOCK_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>
  :root {
    --cx-ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body {
    background: transparent;
    font-family: 'Geist', system-ui, sans-serif;
    display: flex;
    justify-content: center;
    align-items: flex-end;
    height: 100vh;
    padding-bottom: 8px;
  }
  .dock {
    background: rgba(30, 30, 30, 0.85);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    border: 1px solid rgba(255,255,255,0.06);
    border-radius: 16px;
    padding: 8px 16px;
    display: flex;
    align-items: center;
    gap: 8px;
    box-shadow: 0 8px 32px rgba(0,0,0,0.3);
  }
  .dock-icon {
    width: 40px; height: 40px;
    border-radius: 10px;
    background: rgba(255,255,255,0.06);
    display: flex; align-items: center; justify-content: center;
    font-size: 18px;
    cursor: pointer;
    transition: transform 180ms var(--cx-ease-spring);
  }
  .dock-icon:hover { transform: scale(1.15); }
  .dock-icon:active { transform: scale(0.95); transition-duration: 60ms; }
  .sep { width: 1px; height: 24px; background: rgba(255,255,255,0.08); margin: 0 4px; }
</style>
</head>
<body>
  <nav class="dock">
    <div class="dock-icon">📁</div>
    <div class="dock-icon">🌐</div>
    <div class="dock-icon">⌨</div>
    <div class="dock-icon">📝</div>
    <div class="dock-icon">📧</div>
    <div class="sep"></div>
    <div class="dock-icon">🔍</div>
    <div class="sep"></div>
    <div class="dock-icon">⏻</div>
  </nav>
</body>
</html>"#;

pub fn create() -> gtk::Window {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.init_layer_shell();
    window.set_layer(gtk_layer_shell::Layer::Top);
    window.set_anchor(gtk_layer_shell::Edge::Bottom, true);
    window.set_anchor(gtk_layer_shell::Edge::Left, false);
    window.set_anchor(gtk_layer_shell::Edge::Right, false);
    window.set_exclusive_zone(0);
    window.set_namespace("collet-dock");

    window.set_decorated(false);
    window.set_default_size(600, 80);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&container);

    let _webview = WebViewBuilder::new()
        .with_transparent(true)
        .with_html(DOCK_HTML)
        .with_ipc_handler(|msg: wry::http::Request<String>| {
            eprintln!("[collet-shell] IPC: {}", msg.body());
        })
        .build_gtk(&container)
        .expect("Failed to create dock webview");

    eprintln!("[collet-shell] Dock webview created");
    window
}
