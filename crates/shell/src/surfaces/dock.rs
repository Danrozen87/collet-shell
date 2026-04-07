//! Dock surface — bottom-center, expandable, the primary interaction surface.
//!
//! Layer: TOP (above windows, below overlays)
//! Anchor: bottom-center, not edge-to-edge
//! Behavior: compact at rest, expands vertically for search/AI

use gtk::prelude::*;
use gtk_layer_shell::LayerShell;
use wry::{WebViewBuilder, WebViewBuilderExtUnix};

/// The dock HTML — rendered from Collet design tokens.
/// In production, this comes from the Collet component library.
/// For the PoC, inline HTML with the design system CSS variables.
const DOCK_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>
  :root {
    --cx-color-bg: oklch(0.145 0.000 0.0);
    --cx-color-surface: oklch(0.185 0.000 0.0);
    --cx-color-text: oklch(0.880 0.000 0.0);
    --cx-color-text-muted: oklch(0.520 0.000 0.0);
    --cx-color-border: oklch(0.290 0.000 0.0);
    --cx-ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  * { margin: 0; padding: 0; box-sizing: border-box; }

  body {
    background: transparent;
    font-family: 'Geist', 'Inter', system-ui, sans-serif;
    display: flex;
    justify-content: center;
    align-items: flex-end;
    height: 100vh;
    padding-bottom: 8px;
    -webkit-user-select: none;
    user-select: none;
  }

  .dock {
    background: oklch(0.145 0.000 0.0 / 0.85);
    backdrop-filter: blur(24px);
    -webkit-backdrop-filter: blur(24px);
    border: 1px solid oklch(1.0 0.0 0.0 / 0.06);
    border-radius: 16px;
    padding: 8px 16px;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: all 300ms var(--cx-ease-spring);
    box-shadow: 0 8px 32px oklch(0.0 0.0 0.0 / 0.3);
  }

  .dock-icon {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    background: var(--cx-color-surface);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--cx-color-text);
    font-size: 18px;
    cursor: pointer;
    transition: transform 180ms var(--cx-ease-spring);
  }

  .dock-icon:hover {
    transform: scale(1.15);
  }

  .dock-icon:active {
    transform: scale(0.95);
    transition-duration: 60ms;
  }

  .dock-separator {
    width: 1px;
    height: 24px;
    background: var(--cx-color-border);
    margin: 0 4px;
  }

  .dock-label {
    color: var(--cx-color-text-muted);
    font-size: 11px;
    letter-spacing: 0.3px;
  }
</style>
</head>
<body>
  <nav class="dock" role="toolbar" aria-label="Collet Dock">
    <div class="dock-icon" title="Files">📁</div>
    <div class="dock-icon" title="Browser">🌐</div>
    <div class="dock-icon" title="Terminal">⌨</div>
    <div class="dock-icon" title="Editor">📝</div>
    <div class="dock-icon" title="Mail">📧</div>
    <div class="dock-separator"></div>
    <div class="dock-icon" title="Search">🔍</div>
    <div class="dock-separator"></div>
    <div class="dock-icon" title="Power">⏻</div>
  </nav>
</body>
</html>"#;

/// Create the dock layer-shell surface with a wry webview.
pub fn create() -> gtk::Window {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    // Layer shell: bottom of screen, centered, floating
    window.init_layer_shell();
    window.set_layer(gtk_layer_shell::Layer::Top);
    window.set_anchor(gtk_layer_shell::Edge::Bottom, true);
    window.set_anchor(gtk_layer_shell::Edge::Left, false);
    window.set_anchor(gtk_layer_shell::Edge::Right, false);
    window.set_exclusive_zone(0); // Float over content, don't push windows
    window.set_namespace("collet-dock");

    // Window: transparent, borderless
    window.set_decorated(false);
    window.set_app_paintable(true);
    window.set_default_size(600, 64);

    // Create container for the webview
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&container);

    // Build wry webview inside the container
    let _webview = WebViewBuilder::new()
        .with_transparent(true)
        .with_html(DOCK_HTML)
        .with_ipc_handler(|msg: wry::http::Request<String>| {
            eprintln!("[collet-shell] IPC: {}", msg.body());
        })
        .build_gtk(&container)
        .expect("Failed to create dock webview");

    window
}
