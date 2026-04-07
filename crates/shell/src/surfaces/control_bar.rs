//! Control Bar surface — top-right pill, system status.
//!
//! Layer: OVERLAY (above everything)
//! Anchor: top-right corner
//! Content: clock, wifi, battery, power — Mac-style minimal bar

use gtk::prelude::*;
use gtk_layer_shell::LayerShell;
use wry::{WebViewBuilder, WebViewBuilderExtUnix};

const CONTROL_BAR_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>
  :root {
    --cx-color-bg: oklch(0.185 0.000 0.0);
    --cx-color-text: oklch(0.880 0.000 0.0);
    --cx-color-text-muted: oklch(0.520 0.000 0.0);
    --cx-ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  * { margin: 0; padding: 0; box-sizing: border-box; }

  body {
    background: transparent;
    font-family: 'Geist', 'Inter', system-ui, sans-serif;
    display: flex;
    justify-content: flex-end;
    padding: 8px;
    -webkit-user-select: none;
    user-select: none;
  }

  .control-bar {
    background: oklch(0.185 0.000 0.0 / 0.80);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    border: 1px solid oklch(1.0 0.0 0.0 / 0.06);
    border-radius: 99px;
    padding: 6px 16px;
    display: flex;
    align-items: center;
    gap: 12px;
    box-shadow: 0 4px 16px oklch(0.0 0.0 0.0 / 0.2);
  }

  .control-item {
    color: var(--cx-color-text);
    font-size: 12px;
    font-weight: 400;
    letter-spacing: 0.2px;
    cursor: default;
  }

  .control-icon {
    font-size: 14px;
    cursor: pointer;
    transition: opacity 150ms var(--cx-ease-spring);
  }

  .control-icon:hover {
    opacity: 0.7;
  }

  .control-separator {
    width: 1px;
    height: 14px;
    background: oklch(1.0 0.0 0.0 / 0.08);
  }

  .clock {
    font-variant-numeric: tabular-nums;
    font-weight: 500;
  }
</style>
</head>
<body>
  <div class="control-bar" role="status" aria-label="System status">
    <span class="control-icon" title="Network">📶</span>
    <span class="control-separator"></span>
    <span class="control-item clock" id="clock">12:00</span>
    <span class="control-separator"></span>
    <span class="control-icon" title="Power">⏻</span>
  </div>

  <script>
    function updateClock() {
      const now = new Date();
      const h = String(now.getHours()).padStart(2, '0');
      const m = String(now.getMinutes()).padStart(2, '0');
      document.getElementById('clock').textContent = h + ':' + m;
    }
    updateClock();
    setInterval(updateClock, 10000);
  </script>
</body>
</html>"#;

/// Create the control bar layer-shell surface.
pub fn create() -> gtk::Window {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    // Layer shell: top-right, overlay, pill-shaped
    window.init_layer_shell();
    window.set_layer(gtk_layer_shell::Layer::Overlay);
    window.set_anchor(gtk_layer_shell::Edge::Top, true);
    window.set_anchor(gtk_layer_shell::Edge::Right, true);
    window.set_exclusive_zone(0);
    window.set_namespace("collet-control-bar");

    // Window: transparent, borderless, small
    window.set_decorated(false);
    window.set_app_paintable(true);
    window.set_default_size(220, 40);

    let container = gtk::Fixed::new();
    container.set_size_request(220, 40);
    window.add(&container);

    let _webview = WebViewBuilder::new()
        .with_transparent(true)
        .with_html(CONTROL_BAR_HTML)
        .with_ipc_handler(|msg: wry::http::Request<String>| {
            eprintln!("[collet-shell] Control bar IPC: {}", msg.body());
        })
        .build_gtk(&container)
        .expect("Failed to create control bar webview");

    window
}
