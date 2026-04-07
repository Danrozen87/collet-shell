//! Control Bar — top-right pill.

use gtk::prelude::*;
use gtk_layer_shell::LayerShell;
use wry::{WebViewBuilder, WebViewBuilderExtUnix};

const CONTROL_BAR_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body {
    background: transparent;
    font-family: 'Geist', system-ui, sans-serif;
    display: flex;
    justify-content: flex-end;
    padding: 8px;
  }
  .bar {
    background: rgba(40, 40, 40, 0.80);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    border: 1px solid rgba(255,255,255,0.06);
    border-radius: 99px;
    padding: 6px 16px;
    display: flex;
    align-items: center;
    gap: 12px;
    box-shadow: 0 4px 16px rgba(0,0,0,0.2);
    color: #e0e0e0;
    font-size: 12px;
  }
  .sep { width: 1px; height: 14px; background: rgba(255,255,255,0.08); }
  .clock { font-variant-numeric: tabular-nums; font-weight: 500; }
</style>
</head>
<body>
  <div class="bar">
    <span>📶</span>
    <span class="sep"></span>
    <span class="clock" id="c">12:00</span>
    <span class="sep"></span>
    <span>⏻</span>
  </div>
  <script>
    function u(){const n=new Date();document.getElementById('c').textContent=
    String(n.getHours()).padStart(2,'0')+':'+String(n.getMinutes()).padStart(2,'0');}
    u();setInterval(u,10000);
  </script>
</body>
</html>"#;

pub fn create() -> gtk::Window {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.init_layer_shell();
    window.set_layer(gtk_layer_shell::Layer::Overlay);
    window.set_anchor(gtk_layer_shell::Edge::Top, true);
    window.set_anchor(gtk_layer_shell::Edge::Right, true);
    window.set_exclusive_zone(0);
    window.set_namespace("collet-control-bar");

    window.set_decorated(false);
    window.set_default_size(250, 48);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&container);

    let _webview = WebViewBuilder::new()
        .with_transparent(true)
        .with_html(CONTROL_BAR_HTML)
        .with_ipc_handler(|msg: wry::http::Request<String>| {
            eprintln!("[collet-shell] Control bar IPC: {}", msg.body());
        })
        .build_gtk(&container)
        .expect("Failed to create control bar webview");

    eprintln!("[collet-shell] Control bar webview created");
    window
}
