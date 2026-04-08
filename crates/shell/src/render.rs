//! Shell surface renderer — produces HTML from Collet components + custom layout.
//!
//! Two modes:
//! - Linux: render_dock() and render_control_bar() as separate surface HTML
//! - macOS preview: render_preview() combines everything in one desktop simulation

use components::button::{Button, ButtonVariant, ButtonShape};
use components::chat_input::{ChatInput, ChatInputShape};
use components::separator::Separator;
use components::slider::{Slider, SliderShape};
use components::switch::{Switch, SwitchShape};
use design_system::{ComponentSize, Icon};
use design_system::sprite::SpriteCollector;

/// App definition for dock icons.
struct DockApp {
    id: &'static str,
    icon: Icon,
    label: &'static str,
}

const DOCK_APPS: &[DockApp] = &[
    DockApp { id: "browser", icon: Icon::Globe, label: "Browser" },
    DockApp { id: "editor", icon: Icon::FileText, label: "Editor" },
    DockApp { id: "mail", icon: Icon::Mail, label: "Mail" },
    DockApp { id: "calendar", icon: Icon::Calendar, label: "Calendar" },
];

/// Custom icons — included from SVG files, use currentColor for theme adaptation.
const FILES_ICON: &str = include_str!("../../../assets/files-icon.svg");
const SEARCH_ICON: &str = include_str!("../../../assets/search-icon.svg");

fn render_dock_buttons() -> String {
    // Files/Explorer — custom SVG at 42x42 (larger than other icons, it's the home icon)
    let files_btn = format!(
        r#"<button type="button" class="dock-btn dock-home" aria-label="Files" data-ipc="launch" data-app-id="files">{}</button>"#,
        FILES_ICON
    );

    // Standard app buttons from Collet components
    let app_buttons: String = DOCK_APPS
        .iter()
        .map(|app| {
            Button::icon_only(app.icon, app.label)
                .variant(ButtonVariant::Ghost)
                .shape(ButtonShape::Rounded)
                .size(ComponentSize::Lg)
                .data_attr("ipc", "launch")
                .data_attr("app-id", app.id)
                .render()
        })
        .collect();

    // Search is rightmost — triggers dock expansion
    let search_btn = format!(
        r#"<button type="button" class="dock-btn search-trigger" aria-label="Search" data-ipc="search">{}</button>"#,
        SEARCH_ICON
    );

    format!(
        r#"{files_btn}
        {app_buttons}
        {search_btn}"#
    )
}

fn render_control_bar_buttons(battery_pct: u8, wifi_connected: bool) -> String {
    // Order: date | sep | wifi volume battery — date reads first, icons anchor right
    let wifi_path = if wifi_connected {
        "M144,204a16,16,0,1,1-16-16A16,16,0,0,1,144,204ZM239.61,83.91a176,176,0,0,0-223.22,0,12,12,0,1,0,15.23,18.55,152,152,0,0,1,192.76,0,12,12,0,1,0,15.23-18.55Zm-32.16,35.73a128,128,0,0,0-158.9,0,12,12,0,0,0,14.9,18.81,104,104,0,0,1,129.1,0,12,12,0,0,0,14.9-18.81ZM175.07,155.3a80.05,80.05,0,0,0-94.14,0,12,12,0,0,0,14.14,19.4,56,56,0,0,1,65.86,0,12,12,0,1,0,14.14-19.4Z"
    } else {
        "M213.92,210.62a8,8,0,1,1-11.84,10.76L171.31,188a23.53,23.53,0,0,0-2.38-2.19A60,60,0,0,0,98.71,155.3a12,12,0,0,1-14.14,19.4,83.44,83.44,0,0,1,35-21.69L92.51,123a108.29,108.29,0,0,0-31.52,12.3,12,12,0,0,1-14.9-18.81,131.64,131.64,0,0,1,27.31-14.21L53.79,80.56a155.58,155.58,0,0,0-24.71,19,12,12,0,1,1-15.23-18.55,180,180,0,0,1,20.45-15.22L21.92,53.38A8,8,0,0,1,33.76,42.62Z"
    };

    let battery_path = match battery_pct {
        0..=10 => "M200,56H32A24,24,0,0,0,8,80v96a24,24,0,0,0,24,24H200a24,24,0,0,0,24-24V80A24,24,0,0,0,200,56Zm8,120a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H200a8,8,0,0,1,8,8Zm48-80v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Z",
        11..=25 => "M200,56H32A24,24,0,0,0,8,80v96a24,24,0,0,0,24,24H200a24,24,0,0,0,24-24V80A24,24,0,0,0,200,56Zm8,120a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H200a8,8,0,0,1,8,8ZM64,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm192,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Z",
        26..=50 => "M200,56H32A24,24,0,0,0,8,80v96a24,24,0,0,0,24,24H200a24,24,0,0,0,24-24V80A24,24,0,0,0,200,56Zm8,120a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H200a8,8,0,0,1,8,8ZM104,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0ZM64,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm192,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Z",
        51..=75 => "M200,56H32A24,24,0,0,0,8,80v96a24,24,0,0,0,24,24H200a24,24,0,0,0,24-24V80A24,24,0,0,0,200,56Zm8,120a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H200a8,8,0,0,1,8,8ZM144,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm-40,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0ZM64,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm192,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Z",
        _ => "M200,56H32A24,24,0,0,0,8,80v96a24,24,0,0,0,24,24H200a24,24,0,0,0,24-24V80A24,24,0,0,0,200,56Zm8,120a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H200a8,8,0,0,1,8,8ZM184,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm-40,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm-40,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0ZM64,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm192,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Z",
    };

    let battery_extra = if battery_pct <= 20 {
        format!(r#"<span class="bar-pct">{battery_pct}%</span>"#)
    } else {
        String::new()
    };

    format!(
        r#"<button type="button" class="bar-btn bar-btn--clock" aria-label="Date and time" id="c">Mon 1 Jan 00:00</button>
          <span class="bar-sep"></span>
          <button type="button" class="bar-btn" aria-label="Wi-Fi" data-ipc="wifi"><svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="{wifi_path}"/></svg></button>
          <button type="button" class="bar-btn" aria-label="Volume" data-ipc="volume"><svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81ZM32,96H72v64H32ZM144,207.64,88,164.09V91.91l56-43.55Zm54-106.08a40,40,0,0,1,0,52.88,8,8,0,0,1-12-10.58,24,24,0,0,0,0-31.72,8,8,0,0,1,12-10.58ZM248,128a79.9,79.9,0,0,1-20.37,53.34,8,8,0,0,1-11.92-10.67,64,64,0,0,0,0-85.33,8,8,0,1,1,11.92-10.67A79.83,79.83,0,0,1,248,128Z"/></svg></button>
          <button type="button" class="bar-btn" aria-label="Battery" data-ipc="battery"><svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="{battery_path}"/></svg></button>{battery_extra}"#
    )
}

/// Render the search input using Collet ChatInput component.
fn render_search_input() -> String {
    ChatInput::new("dock-search")
        .placeholder("Search apps, files, or ask anything...")
        .shape(ChatInputShape::Rounded)
        .size(ComponentSize::Sm)
        .show_action_button(false)
        .max_rows(4)
        .submit_label("Search")
        .render()
}

/// Render island content with Collet components.
fn render_island_content() -> String {
    // Sliders — Collet Slider (Md, Pill, labeled with value)
    let brightness_slider = Slider::new("brightness", "Brightness")
        .value(75.0)
        .min(0.0)
        .max(100.0)
        .step(1.0)
        .shape(SliderShape::Pill)
        .size(ComponentSize::Md)
        .show_value(true)
        .value_text("75%")
        .render();

    let volume_slider = Slider::new("volume", "Volume")
        .value(60.0)
        .min(0.0)
        .max(100.0)
        .step(1.0)
        .shape(SliderShape::Pill)
        .size(ComponentSize::Md)
        .show_value(true)
        .value_text("60%")
        .render();

    // Dividers — Collet Separator (decorative)
    let sep1 = Separator::new("island-sep-1").render();
    let sep2 = Separator::new("island-sep-2").render();

    format!(
        r#"<!-- Quick Settings — Split tiles -->
        <div class="qs-tiles">
          <div class="qs-tile active" data-qs="wifi">
            <button type="button" class="qs-tile-body" data-ipc="wifi-settings" aria-label="Wi-Fi settings">
              <span class="qs-tile-label">Wi-Fi</span>
              <span class="qs-tile-detail">Connected</span>
            </button>
            <button type="button" class="qs-tile-toggle" data-ipc="wifi-toggle" aria-label="Toggle Wi-Fi" aria-pressed="true">
              <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M144,204a16,16,0,1,1-16-16A16,16,0,0,1,144,204ZM239.61,83.91a176,176,0,0,0-223.22,0,12,12,0,1,0,15.23,18.55,152,152,0,0,1,192.76,0,12,12,0,1,0,15.23-18.55Zm-32.16,35.73a128,128,0,0,0-158.9,0,12,12,0,0,0,14.9,18.81,104,104,0,0,1,129.1,0,12,12,0,0,0,14.9-18.81ZM175.07,155.3a80.05,80.05,0,0,0-94.14,0,12,12,0,0,0,14.14,19.4,56,56,0,0,1,65.86,0,12,12,0,1,0,14.14-19.4Z"/></svg>
            </button>
          </div>
          <div class="qs-tile" data-qs="bluetooth">
            <button type="button" class="qs-tile-body" data-ipc="bt-settings" aria-label="Bluetooth settings">
              <span class="qs-tile-label">Bluetooth</span>
              <span class="qs-tile-detail">Off</span>
            </button>
            <button type="button" class="qs-tile-toggle" data-ipc="bt-toggle" aria-label="Toggle Bluetooth" aria-pressed="false">
              <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M196.8,169.6,141.33,128,196.8,86.4a8,8,0,0,0,0-12.8l-64-48A8,8,0,0,0,120,32v80L68.8,73.6a8,8,0,0,0-9.6,12.8L114.67,128,59.2,169.6a8,8,0,1,0,9.6,12.8L120,144v80a8,8,0,0,0,12.8,6.4l64-48a8,8,0,0,0,0-12.8ZM136,48l42.67,32L136,112Zm0,160V144l42.67,32Z"/></svg>
            </button>
          </div>
        </div>

        {sep1}

        <!-- Sliders — Collet Slider components with trailing icons -->
        <div class="island-slider-wrap">
          {brightness_slider}
          <svg class="island-slider-icon" width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M120,40V16a8,8,0,0,1,16,0V40a8,8,0,0,1-16,0Zm72,88a64,64,0,1,1-64-64A64.07,64.07,0,0,1,192,128Zm-16,0a48,48,0,1,0-48,48A48.05,48.05,0,0,0,176,128ZM58.34,69.66A8,8,0,0,0,69.66,58.34l-16-16A8,8,0,0,0,42.34,53.66Zm0,116.68-16,16a8,8,0,0,0,11.32,11.32l16-16a8,8,0,0,0-11.32-11.32ZM192,72a8,8,0,0,0,5.66-2.34l16-16a8,8,0,0,0-11.32-11.32l-16,16A8,8,0,0,0,192,72Zm5.66,114.34a8,8,0,0,0-11.32,11.32l16,16a8,8,0,0,0,11.32-11.32ZM48,128a8,8,0,0,0-8-8H16a8,8,0,0,0,0,16H40A8,8,0,0,0,48,128Zm80,80a8,8,0,0,0-8,8v24a8,8,0,0,0,16,0V216A8,8,0,0,0,128,208Zm112-88H216a8,8,0,0,0,0,16h24a8,8,0,0,0,0-16Z"/></svg>
        </div>
        <div class="island-slider-wrap">
          {volume_slider}
          <svg class="island-slider-icon" width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81ZM32,96H72v64H32ZM144,207.64,88,164.09V91.91l56-43.55Zm54-106.08a40,40,0,0,1,0,52.88,8,8,0,0,1-12-10.58,24,24,0,0,0,0-31.72,8,8,0,0,1,12-10.58ZM248,128a79.9,79.9,0,0,1-20.37,53.34,8,8,0,0,1-11.92-10.67,64,64,0,0,0,0-85.33,8,8,0,1,1,11.92-10.67A79.83,79.83,0,0,1,248,128Z"/></svg>
        </div>

        {sep2}

        <!-- Calendar -->
        <div id="calWrap">
          <div class="cal-header" id="calHeader"></div>
          <div class="cal-grid" id="calGrid"></div>
          <div class="cal-agenda" id="calAgenda" style="display:none"></div>
        </div>

        <!-- Profile — expandable with menu items -->
        <div class="profile-section" id="profileSection">
          <button type="button" class="profile-row" id="profileToggle" aria-expanded="false">
            <div class="profile-avatar">D</div>
            <div class="profile-info">
              <span class="profile-name">Dan</span>
              <span class="profile-host">collet-os</span>
            </div>
            <svg class="profile-chevron" width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M213.66,101.66l-80,80a8,8,0,0,1-11.32,0l-80-80a8,8,0,0,1,11.32-11.32L128,164.69l74.34-74.35a8,8,0,0,1,11.32,11.32Z"/></svg>
          </button>
          <div class="profile-menu" id="profileMenu">
            <button type="button" class="profile-menu-item" data-ipc="lock">
              <svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M208,80H176V56a48,48,0,0,0-96,0V80H48A16,16,0,0,0,32,96V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V96A16,16,0,0,0,208,80ZM96,56a32,32,0,0,1,64,0V80H96ZM208,208H48V96H208Z"/></svg>
              Lock Screen
            </button>
            <button type="button" class="profile-menu-item" data-ipc="open_settings">
              <svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M128,80a48,48,0,1,0,48,48A48.05,48.05,0,0,0,128,80Zm0,80a32,32,0,1,1,32-32A32,32,0,0,1,128,160Zm88-29.84q.06-2.16,0-4.32l14.92-18.64a8,8,0,0,0,1.48-7.06,107.21,107.21,0,0,0-10.88-26.25,8,8,0,0,0-6-3.93l-23.72-2.64q-1.48-1.56-3-3L186,40.54a8,8,0,0,0-3.94-6,107.71,107.71,0,0,0-26.25-10.87,8,8,0,0,0-7.06,1.49L130.16,40Q128,40,125.84,40L107.2,25.11a8,8,0,0,0-7.06-1.48A107.6,107.6,0,0,0,73.89,34.51a8,8,0,0,0-3.93,6L67.32,64.27q-1.56,1.49-3,3L40.54,70a8,8,0,0,0-6,3.94,107.71,107.71,0,0,0-10.87,26.25,8,8,0,0,0,1.49,7.06L40,125.84Q40,128,40,130.16L25.11,148.8a8,8,0,0,0-1.48,7.06,107.21,107.21,0,0,0,10.88,26.25,8,8,0,0,0,6,3.93l23.72,2.64q1.49,1.56,3,3L70,215.46a8,8,0,0,0,3.94,6,107.71,107.71,0,0,0,26.25,10.87,8,8,0,0,0,7.06-1.49L125.84,216q2.16.06,4.32,0l18.64,14.92a8,8,0,0,0,7.06,1.48,107.21,107.21,0,0,0,26.25-10.88,8,8,0,0,0,3.93-6l2.64-23.72q1.56-1.48,3-3L215.46,186a8,8,0,0,0,6-3.94,107.71,107.71,0,0,0,10.87-26.25,8,8,0,0,0-1.49-7.06ZM128,168a40,40,0,1,1,40-40A40,40,0,0,1,128,168Z"/></svg>
              System Settings
            </button>
            <button type="button" class="profile-menu-item" data-ipc="suspend">
              <svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M233.54,142.23a8,8,0,0,0-8-2,88.08,88.08,0,0,1-109.8-109.8,8,8,0,0,0-10-10,104.84,104.84,0,0,0-52.91,37A104,104,0,0,0,136,224a103.09,103.09,0,0,0,62.52-20.88,104.84,104.84,0,0,0,37-52.91A8,8,0,0,0,233.54,142.23Z"/></svg>
              Suspend
            </button>
            <button type="button" class="profile-menu-item profile-menu-item--danger" data-ipc="logout">
              <svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M112,216a8,8,0,0,1-8,8H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32h56a8,8,0,0,1,0,16H48V208h56A8,8,0,0,1,112,216Zm109.66-93.66-40-40a8,8,0,0,0-11.32,11.32L196.69,120H104a8,8,0,0,0,0,16h92.69l-26.35,26.34a8,8,0,0,0,11.32,11.32l40-40A8,8,0,0,0,221.66,122.34Z"/></svg>
              Log out
            </button>
          </div>
        </div>"#
    )
}

/// macOS preview — full desktop simulation in one window.
/// Dock at bottom, control bar at top-right, desktop area in the middle.
pub fn render_preview() -> String {
    use std::fs;
    use base64::Engine as _;

    SpriteCollector::reset();

    let dock_buttons = render_dock_buttons();
    let control_bar_buttons = render_control_bar_buttons(72, true);
    let island_content = render_island_content();
    let search_input = render_search_input();
    let sprites = SpriteCollector::take_sprite();
    let tokens_css = design_system::generate_tokens_css();

    // Load wallpapers as base64 data URLs
    let bg_data_url = fs::read("assets/bg-dark.jpg")
        .map(|bytes| {
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            format!("data:image/jpeg;base64,{b64}")
        })
        .unwrap_or_default();

    let bg_light_url = fs::read("assets/bg-light.jpg")
        .map(|bytes| {
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            format!("data:image/jpeg;base64,{b64}")
        })
        .unwrap_or_default();

    format!(
        r##"<!DOCTYPE html>
<html lang="en" data-theme="dark">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<script src="https://cdn.tailwindcss.com"></script>
<style>{tokens_css}</style>
<style>
  :root {{
    --cx-ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
  }}

  * {{ margin: 0; padding: 0; box-sizing: border-box; }}

  .skip-link {{
    position: absolute;
    top: -100px;
    left: 16px;
    z-index: 9999;
    background: oklch(0.2 0 0);
    color: oklch(0.9 0 0);
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
    text-decoration: none;
    transition: top var(--cx-duration-fast, 100ms) ease-out;
  }}
  .skip-link:focus {{
    top: 10px;
  }}

  html, body {{
    height: 100%;
    overflow: hidden;
  }}

  body {{
    font-family: var(--cx-font-sans, 'Geist', 'Inter', system-ui, sans-serif);
    -webkit-user-select: none;
    user-select: none;
  }}

  /* ── Desktop ─────────────────────────────────────── */
  .desktop {{
    width: 100%;
    height: 100%;
    background: url('{bg_data_url}') center/cover no-repeat, oklch(0.13 0.000 0.0);
    position: relative;
    display: flex;
    flex-direction: column;
  }}

  /* ── Control Bar ─────────────────────────────────── */
  .control-bar-container {{
    position: absolute;
    top: 10px;
    right: 14px;
    z-index: 100;
  }}

  .control-bar {{
    background: oklch(0.185 0.000 0.0 / 0.78);
    backdrop-filter: blur(24px) saturate(1.2);
    -webkit-backdrop-filter: blur(24px) saturate(1.2);
    border: 1px solid oklch(1.0 0.0 0.0 / 0.06);
    border-radius: 99px;
    padding: 6px 14px;
    display: flex;
    align-items: center;
    gap: 8px;
    box-shadow: 0 4px 20px oklch(0.0 0.0 0.0 / 0.25);
    color: oklch(0.880 0.000 0.0);
    font-size: 12px;
    white-space: nowrap;
  }}

  .bar-sep {{
    width: 1px;
    height: 14px;
    background: oklch(1.0 0.0 0.0 / 0.06);
    flex-shrink: 0;
  }}

  /* ── Control Island ─────────────────────────────── */
  .control-island {{
    position: absolute;
    top: calc(100% + 12px);
    right: 0;
    width: 360px;
    background: oklch(0.185 0.000 0.0 / 0.88);
    backdrop-filter: blur(32px) saturate(1.3);
    -webkit-backdrop-filter: blur(32px) saturate(1.3);
    border: 1px solid oklch(1.0 0.0 0.0 / 0.06);
    border-radius: 20px;
    padding: 20px;
    box-shadow: 0 8px 40px oklch(0.0 0.0 0.0 / 0.35);
    color: oklch(0.880 0.000 0.0);
    font-size: 13px;
    opacity: 0;
    pointer-events: none;
    transform: translateY(-12px) scale(0.96);
    transform-origin: top right;
    transition: opacity var(--cx-duration-normal, 200ms) ease,
                transform var(--cx-duration-smooth, 300ms) var(--cx-ease-spring, cubic-bezier(0.34,1.56,0.64,1)),
                backdrop-filter var(--cx-duration-smooth, 300ms) ease;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }}

  .control-island.open {{
    opacity: 1;
    pointer-events: auto;
    transform: translateY(0) scale(1);
  }}

  /* ── Profile row ──────────────────────────────── */
  .profile-row {{
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 10px 12px;
    border: 1px solid oklch(1.0 0 0 / 0.04);
    border-radius: 14px;
    background: oklch(1.0 0 0 / 0.03);
    cursor: pointer;
    transition: background var(--cx-duration-fast, 100ms) ease-out, transform 150ms var(--cx-ease-spring, cubic-bezier(0.34,1.56,0.64,1));
    -webkit-appearance: none;
    text-align: left;
    color: inherit;
  }}
  .profile-avatar {{
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: oklch(0.45 0.15 250 / 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 14px;
    color: oklch(0.9 0.05 250);
    flex-shrink: 0;
  }}
  .profile-info {{
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }}
  .profile-name {{
    font-size: 13px;
    font-weight: 600;
    color: oklch(0.9 0 0);
    line-height: 1;
  }}
  .profile-host {{
    font-size: 11px;
    color: oklch(0.55 0 0);
    line-height: 1;
  }}
  [data-theme="light"] .profile-name {{ color: oklch(0.2 0 0); }}
  [data-theme="light"] .profile-host {{ color: oklch(0.5 0 0); }}
  [data-theme="light"] .profile-avatar {{
    background: oklch(0.55 0.15 250 / 0.2);
    color: oklch(0.35 0.12 250);
  }}


  /* ── Profile section (expandable) ────────────── */
  .profile-section {{
    border-radius: 14px;
    overflow: hidden;
    border: 1px solid oklch(1.0 0 0 / 0.04);
    background: oklch(1.0 0 0 / 0.03);
    transition: background var(--cx-duration-fast, 100ms) ease-out;
  }}
  .profile-section:hover {{
    background: oklch(1.0 0 0 / 0.05);
  }}
  [data-theme="light"] .profile-section {{
    background: oklch(0 0 0 / 0.02);
    border-color: oklch(0 0 0 / 0.05);
  }}
  [data-theme="light"] .profile-section:hover {{
    background: oklch(0 0 0 / 0.04);
  }}
  .profile-section .profile-row {{
    border: none;
    background: none;
    border-radius: 0;
  }}
  .profile-section .profile-row:hover {{
    background: oklch(1.0 0 0 / 0.04);
  }}
  [data-theme="light"] .profile-section .profile-row:hover {{
    background: oklch(0 0 0 / 0.03);
  }}
  .profile-chevron {{
    opacity: 0.35;
    flex-shrink: 0;
    transition: opacity var(--cx-duration-fast, 100ms) ease-out, transform var(--cx-duration-smooth, 300ms) var(--cx-ease-spring, cubic-bezier(0.34,1.56,0.64,1));
  }}
  .profile-row:hover .profile-chevron {{
    opacity: 0.7;
  }}
  .profile-section.open .profile-chevron {{
    transform: rotate(180deg);
    opacity: 0.7;
  }}
  .profile-menu {{
    max-height: 0;
    overflow: hidden;
    transition: max-height var(--cx-duration-smooth, 300ms) var(--cx-ease-spring, cubic-bezier(0.34,1.56,0.64,1)),
                opacity 200ms ease;
    opacity: 0;
  }}
  .profile-section.open .profile-menu {{
    max-height: 200px;
    opacity: 1;
  }}
  .profile-menu-item {{
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 9px 16px;
    border: none;
    background: none;
    color: oklch(0.75 0 0);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    text-align: left;
    transition: background var(--cx-duration-fast, 100ms) ease-out, color var(--cx-duration-fast, 100ms) ease-out;
    -webkit-appearance: none;
  }}
  .profile-menu-item:hover {{
    background: oklch(1.0 0 0 / 0.06);
    color: oklch(0.95 0 0);
  }}
  .profile-menu-item:active {{
    background: oklch(1.0 0 0 / 0.1);
  }}
  .profile-menu-item svg {{
    opacity: 0.6;
    flex-shrink: 0;
  }}
  .profile-menu-item:hover svg {{
    opacity: 0.9;
  }}
  .profile-menu-item--danger {{
    color: oklch(0.65 0.12 25);
  }}
  .profile-menu-item--danger:hover {{
    color: oklch(0.75 0.15 25);
    background: oklch(0.65 0.12 25 / 0.08);
  }}
  [data-theme="light"] .profile-menu-item {{
    color: oklch(0.4 0 0);
  }}
  [data-theme="light"] .profile-menu-item:hover {{
    background: oklch(0 0 0 / 0.04);
    color: oklch(0.15 0 0);
  }}
  [data-theme="light"] .profile-menu-item--danger {{
    color: oklch(0.5 0.15 25);
  }}
  [data-theme="light"] .profile-menu-item--danger:hover {{
    color: oklch(0.4 0.18 25);
    background: oklch(0.5 0.15 25 / 0.08);
  }}

  /* Quick settings grid */
  /* Island component overrides — Collet components on dark surface */
  .control-island label {{
    color: oklch(0.82 0 0);
    font-size: 13px;
  }}
  [data-theme="light"] .control-island label {{
    color: oklch(0.3 0.003 90);
  }}

  /* Slider value text — thin weight + trailing icon */
  .control-island [data-slider-value] {{
    font-weight: 300 !important;
    opacity: 0.7;
    font-size: 12px !important;
  }}
  /* Label row: add room for trailing icon */
  .island-slider-wrap {{
    position: relative;
  }}
  .island-slider-icon {{
    position: absolute;
    top: 3px;
    right: 0;
    opacity: 0.5;
    pointer-events: none;
  }}
  /* Offset the slider's value so it doesn't overlap the icon */
  .island-slider-wrap [data-slider-value] {{
    margin-right: 20px;
  }}

  /* ── Quick Settings split tiles ────────────────── */
  .qs-tiles {{
    display: flex;
    gap: 10px;
  }}
  .qs-tile {{
    flex: 1;
    display: flex;
    border-radius: 14px;
    overflow: hidden;
    background: oklch(1.0 0 0 / 0.06);
    border: 1px solid oklch(1.0 0 0 / 0.04);
    transition: background var(--cx-duration-fast, 100ms) ease-out;
  }}
  .qs-tile.active {{
    background: oklch(0.45 0.15 250 / 0.25);
    border-color: oklch(0.55 0.15 250 / 0.3);
  }}
  .qs-tile-body {{
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 12px 14px;
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
    text-align: left;
    transition: background var(--cx-duration-fast, 100ms) ease-out;
    -webkit-appearance: none;
  }}
  .qs-tile-body:hover {{
    background: oklch(1.0 0 0 / 0.06);
  }}
  .qs-tile-body:active {{
    background: oklch(1.0 0 0 / 0.1);
  }}
  .qs-tile-icon {{
    opacity: 0.85;
  }}
  .qs-tile.active .qs-tile-icon {{
    opacity: 1;
  }}
  .qs-tile-label {{
    font-size: 12px;
    font-weight: 600;
    color: oklch(0.85 0 0);
    line-height: 1;
  }}
  .qs-tile.active .qs-tile-label {{
    color: oklch(0.95 0 0);
  }}
  .qs-tile-detail {{
    font-size: 10px;
    color: oklch(0.55 0 0);
    line-height: 1;
  }}
  .qs-tile.active .qs-tile-detail {{
    color: oklch(0.7 0.05 250);
  }}
  .qs-tile-toggle {{
    display: flex;
    align-items: center;
    justify-content: center;
    width: 42px;
    background: none;
    border: none;
    border-left: 1px solid oklch(1.0 0 0 / 0.06);
    color: oklch(0.6 0 0);
    cursor: pointer;
    transition: background var(--cx-duration-fast, 100ms) ease-out, color var(--cx-duration-fast, 100ms) ease-out;
    -webkit-appearance: none;
  }}
  .qs-tile-toggle:hover {{
    background: oklch(1.0 0 0 / 0.08);
    color: oklch(0.9 0 0);
  }}
  .qs-tile-toggle:active {{
    background: oklch(1.0 0 0 / 0.14);
  }}
  .qs-tile-toggle[aria-pressed="true"] {{
    color: oklch(0.85 0.1 250);
  }}
  .qs-tile-toggle svg {{
    pointer-events: none;
  }}

  /* Light mode overrides for tiles */
  [data-theme="light"] .qs-tile {{
    background: oklch(0 0 0 / 0.04);
    border-color: oklch(0 0 0 / 0.06);
  }}
  [data-theme="light"] .qs-tile.active {{
    background: oklch(0.55 0.15 250 / 0.15);
    border-color: oklch(0.5 0.15 250 / 0.25);
  }}
  [data-theme="light"] .qs-tile-body:hover {{
    background: oklch(0 0 0 / 0.04);
  }}
  [data-theme="light"] .qs-tile-label {{
    color: oklch(0.25 0 0);
  }}
  [data-theme="light"] .qs-tile.active .qs-tile-label {{
    color: oklch(0.15 0.05 250);
  }}
  [data-theme="light"] .qs-tile-detail {{
    color: oklch(0.45 0 0);
  }}
  [data-theme="light"] .qs-tile.active .qs-tile-detail {{
    color: oklch(0.35 0.08 250);
  }}
  [data-theme="light"] .qs-tile-toggle {{
    border-left-color: oklch(0 0 0 / 0.08);
    color: oklch(0.4 0 0);
  }}
  [data-theme="light"] .qs-tile.active .qs-tile-toggle {{
    color: oklch(0.35 0.12 250);
  }}
  [data-theme="light"] .qs-tile-toggle:hover {{
    background: oklch(0 0 0 / 0.06);
    color: oklch(0.2 0 0);
  }}

  /* Calendar mini view */
  .cal-header {{
    font-size: 13px;
    font-weight: 500;
    color: oklch(0.75 0.0 0.0);
    text-align: center;
    padding-bottom: 8px;
  }}

  .cal-grid {{
    display: grid;
    grid-template-columns: 28px repeat(7, 1fr);
    gap: 2px;
    text-align: center;
    font-size: 11px;
  }}

  .cal-day-name {{
    color: oklch(0.45 0.0 0.0);
    font-size: 10px;
    padding: 4px 0;
    font-weight: 500;
  }}

  .cal-day {{
    padding: 4px 0;
    border-radius: 8px;
    cursor: pointer;
    color: oklch(0.65 0.0 0.0);
    transition: background var(--cx-duration-fast, 100ms) ease-out, transform 100ms var(--cx-ease-spring, cubic-bezier(0.34,1.56,0.64,1));
  }}
  .cal-day:hover {{
    background: oklch(1.0 0 0 / 0.08);
  }}
  .cal-day:active {{
    transform: scale(0.9);
  }}
  .cal-day.other-month:hover {{
    background: none;
    cursor: default;
  }}

  .cal-day.today {{
    background: oklch(1.0 0.0 0.0 / 0.12);
    color: oklch(0.95 0.0 0.0);
    font-weight: 600;
  }}

  .cal-day.other-month {{
    color: oklch(0.3 0.0 0.0);
  }}

  .cal-wk {{
    font-size: 10px;
    color: oklch(0.38 0 0);
    font-weight: 500;
    padding: 4px 0;
    text-align: center;
  }}
  .cal-agenda {{
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding-top: 10px;
    animation: agenda-in 300ms var(--cx-ease-spring, cubic-bezier(0.34,1.56,0.64,1)) forwards;
  }}
  @keyframes agenda-in {{
    from {{ opacity: 0; transform: translateY(-6px); }}
    to {{ opacity: 1; transform: translateY(0); }}
  }}
  .cal-agenda-item {{
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 10px;
    background: oklch(1.0 0 0 / 0.04);
    transition: background var(--cx-duration-fast, 100ms) ease-out;
    cursor: pointer;
  }}
  .cal-agenda-item:hover {{
    background: oklch(1.0 0 0 / 0.09);
  }}
  .cal-agenda-dot {{
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }}
  .cal-agenda-time {{
    font-size: 11px;
    color: oklch(0.5 0 0);
    min-width: 44px;
    font-variant-numeric: tabular-nums;
  }}
  .cal-agenda-title {{
    font-size: 12px;
    color: oklch(0.82 0 0);
    font-weight: 500;
  }}
  [data-theme="light"] .cal-wk {{ color: oklch(0.55 0 0); }}
  [data-theme="light"] .cal-day:hover {{ background: oklch(0 0 0 / 0.06); }}
  [data-theme="light"] .cal-agenda-item {{ background: oklch(0 0 0 / 0.03); }}
  [data-theme="light"] .cal-agenda-item:hover {{ background: oklch(0 0 0 / 0.06); }}
  [data-theme="light"] .cal-agenda-time {{ color: oklch(0.45 0 0); }}
  [data-theme="light"] .cal-agenda-title {{ color: oklch(0.25 0 0); }}

  .cal-day-selected {{
    background: oklch(0.5 0.15 250 / 0.3) !important;
    color: oklch(0.95 0 0) !important;
    font-weight: 600;
  }}
  [data-theme="light"] .cal-day-selected {{
    background: oklch(0.55 0.15 250 / 0.2) !important;
    color: oklch(0.15 0.05 250) !important;
  }}

  .bar-btn {{
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border: 1px solid transparent;
    border-radius: 6px;
    background: transparent;
    color: oklch(0.85 0 0);
    cursor: pointer;
    transition: background-color 100ms ease-out, color 100ms ease-out, transform 150ms var(--cx-ease-spring, cubic-bezier(0.34,1.56,0.64,1));
    -webkit-appearance: none;
    padding: 0;
    flex-shrink: 0;
  }}

  .bar-btn.bar-btn--clock {{
    width: auto;
    padding: 0 8px;
    font-variant-numeric: tabular-nums;
    font-weight: 500;
    letter-spacing: 0.3px;
    font-size: 12px;
    white-space: nowrap;
    flex-shrink: 0;
  }}
  .bar-btn:hover {{
    background: oklch(1.0 0 0 / 0.1);
    color: oklch(1.0 0 0);
  }}
  .bar-btn:active {{
    background: oklch(1.0 0 0 / 0.18);
    transform: scale(0.93);
  }}
  .bar-btn:focus-visible,
  .qs-tile-body:focus-visible,
  .qs-tile-toggle:focus-visible,
  .profile-row:focus-visible,
  .profile-menu-item:focus-visible,
  .cal-day:focus-visible,
  .search-result:focus-visible,
  .dock button:focus-visible,
  .dock-btn:focus-visible {{
    outline: 2px solid oklch(0.6 0.15 250);
    outline-offset: 2px;
  }}
  .bar-btn svg {{
    pointer-events: none;
  }}
  .bar-pct {{
    font-size: 10px;
    font-weight: 500;
    color: oklch(0.65 0.12 25);
    margin-left: -4px;
  }}

  /* ── Notifications ─────────────────────────────── */
  .notif-stack {{
    position: absolute;
    top: 60px;
    right: 14px;
    width: 340px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    z-index: 90;
    pointer-events: none;
  }}
  .notif {{
    background: oklch(0.185 0 0 / 0.92);
    backdrop-filter: blur(24px) saturate(1.3);
    -webkit-backdrop-filter: blur(24px) saturate(1.3);
    border: 1px solid oklch(1.0 0 0 / 0.08);
    border-radius: 14px;
    padding: 14px 16px;
    display: flex;
    gap: 12px;
    align-items: flex-start;
    pointer-events: auto;
    opacity: 0;
    transform: translateX(20px) scale(0.95);
    animation: notif-in var(--cx-duration-smooth, 300ms) var(--cx-ease-spring, cubic-bezier(0.34,1.56,0.64,1)) forwards;
    box-shadow: 0 4px 20px oklch(0 0 0 / 0.3);
    cursor: pointer;
    transition: opacity 200ms ease, transform 200ms ease;
  }}
  .notif.dismiss {{
    opacity: 0;
    transform: translateX(40px) scale(0.9);
  }}
  @keyframes notif-in {{
    from {{ opacity: 0; transform: translateX(20px) scale(0.95); }}
    to {{ opacity: 1; transform: translateX(0) scale(1); }}
  }}
  .notif-icon {{
    width: 32px;
    height: 32px;
    border-radius: 8px;
    background: oklch(0.45 0.15 250 / 0.3);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    color: oklch(0.8 0.1 250);
  }}
  .notif-body {{
    flex: 1;
    min-width: 0;
  }}
  .notif-title {{
    font-size: 13px;
    font-weight: 600;
    color: oklch(0.92 0 0);
    line-height: 1.3;
  }}
  .notif-text {{
    font-size: 12px;
    color: oklch(0.6 0 0);
    line-height: 1.4;
    margin-top: 2px;
  }}
  .notif-time {{
    font-size: 10px;
    color: oklch(0.4 0 0);
    flex-shrink: 0;
    margin-top: 2px;
  }}
  [data-theme="light"] .notif {{
    background: oklch(1.0 0 0 / 0.88);
    border-color: oklch(0 0 0 / 0.08);
  }}
  [data-theme="light"] .notif-title {{ color: oklch(0.15 0 0); }}
  [data-theme="light"] .notif-text {{ color: oklch(0.45 0 0); }}

  /* ── OSD (on-screen display) ──────────────────── */
  .osd {{
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%) scale(0.9);
    background: oklch(0.15 0 0 / 0.85);
    backdrop-filter: blur(32px);
    -webkit-backdrop-filter: blur(32px);
    border: 1px solid oklch(1.0 0 0 / 0.08);
    border-radius: 16px;
    padding: 20px 28px;
    display: flex;
    align-items: center;
    gap: 16px;
    z-index: 200;
    opacity: 0;
    pointer-events: none;
    transition: opacity var(--cx-duration-normal, 200ms) ease,
                transform var(--cx-duration-smooth, 300ms) var(--cx-ease-spring, cubic-bezier(0.34,1.56,0.64,1));
    box-shadow: 0 8px 40px oklch(0 0 0 / 0.5);
  }}
  .osd.visible {{
    opacity: 1;
    transform: translate(-50%, -50%) scale(1);
    pointer-events: auto;
  }}
  .osd-icon {{
    color: oklch(0.85 0 0);
    flex-shrink: 0;
  }}
  .osd-track {{
    width: 160px;
    height: 4px;
    background: oklch(1.0 0 0 / 0.1);
    border-radius: 9999px;
    overflow: hidden;
  }}
  .osd-fill {{
    height: 100%;
    background: oklch(0.85 0.1 250);
    border-radius: 9999px;
    transition: width 100ms ease-out;
  }}
  .osd-value {{
    font-size: 14px;
    font-weight: 300;
    color: oklch(0.75 0 0);
    font-variant-numeric: tabular-nums;
    min-width: 36px;
    text-align: right;
  }}
  [data-theme="light"] .osd {{
    background: oklch(1.0 0 0 / 0.82);
    border-color: oklch(0 0 0 / 0.1);
  }}
  [data-theme="light"] .osd-icon {{ color: oklch(0.25 0 0); }}
  [data-theme="light"] .osd-fill {{ background: oklch(0.5 0.15 250); }}
  [data-theme="light"] .osd-value {{ color: oklch(0.35 0 0); }}

  /* ── Dock ────────────────────────────────────────── */
  .dock-container {{
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    z-index: 100;
    min-width: 520px;
  }}

  .dock {{
    background: oklch(0.145 0.000 0.0 / 0.88);
    backdrop-filter: blur(24px) saturate(1.2);
    -webkit-backdrop-filter: blur(24px) saturate(1.2);
    border: 1px solid oklch(1.0 0.0 0.0 / 0.06);
    border-bottom: none;
    border-radius: 16px 16px 0 0;
    padding: 10px 16px 12px;
    display: flex;
    align-items: center;
    gap: 6px;
    box-shadow: 0 -4px 40px oklch(0.0 0.0 0.0 / 0.25);
    transition: all 300ms var(--cx-ease-spring);
  }}

  .sep {{
    width: 1px;
    height: 28px;
    background: oklch(1.0 0.0 0.0 / 0.06);
    margin: 0 2px;
    flex-shrink: 0;
  }}

  /* ── Dock buttons (both Collet and custom) ───────── */
  .dock-btn {{
    width: 40px; height: 40px;
    border-radius: 10px;
    border: none;
    background: transparent;
    display: flex; align-items: center; justify-content: center;
    color: oklch(0.85 0.000 0.0);
    cursor: pointer;
    transition: all 180ms var(--cx-ease-spring);
    padding: 0;
  }}

  .dock-btn svg, .dock-btn img {{
    pointer-events: none;
  }}

  .dock-home {{
    width: 48px;
    height: 48px;
    border-radius: 12px;
  }}

  /* ── Light mode overrides ────────────────────────── */
  [data-theme="light"] .dock button,
  [data-theme="light"] .dock-btn {{
    color: oklch(0.25 0.003 90) !important;
  }}

  [data-theme="light"] .dock button:hover,
  [data-theme="light"] .dock-btn:hover {{
    color: oklch(0.15 0.003 90) !important;
    background: oklch(0.0 0.0 0.0 / 0.06) !important;
  }}

  /* ChatInput light mode — prevent dock button overrides from clobbering component */
  [data-theme="light"] .dock [data-chat-input] {{
    background: oklch(0.94 0.003 90) !important;
    border-color: oklch(0 0 0 / 0.1) !important;
  }}
  [data-theme="light"] .dock [data-chat-textarea] {{
    color: oklch(0.15 0.003 90) !important;
  }}
  [data-theme="light"] .dock [data-chat-submit] {{
    background-color: oklch(0.355 0.003 90) !important;
    color: oklch(0.955 0.005 90) !important;
  }}

  [data-theme="light"] .sep,
  [data-theme="light"] .bar-sep {{
    background: oklch(0.0 0.0 0.0 / 0.08);
  }}
  [data-theme="light"] .bar-btn {{
    color: oklch(0.3 0 0);
  }}
  [data-theme="light"] .bar-btn:hover {{
    background: oklch(0.0 0 0 / 0.08);
    color: oklch(0.15 0 0);
  }}
  [data-theme="light"] .bar-btn:active {{
    background: oklch(0.0 0 0 / 0.15);
  }}

  [data-theme="light"] .dock {{
    border-color: oklch(0.0 0.0 0.0 / 0.06);
    box-shadow: 0 -4px 40px oklch(0.0 0.0 0.0 / 0.1);
  }}

  [data-theme="light"] .control-bar {{
    border-color: oklch(0.0 0.0 0.0 / 0.06);
    color: oklch(0.27 0.003 90);
  }}

  [data-theme="light"] .control-island {{
    border-color: oklch(0.0 0.0 0.0 / 0.06);
    color: oklch(0.27 0.003 90);
  }}

  [data-theme="light"] .cal-day {{
    color: oklch(0.4 0.003 90);
  }}

  [data-theme="light"] .cal-day.today {{
    background: oklch(0.0 0.0 0.0 / 0.08);
    color: oklch(0.15 0.003 90);
  }}

  [data-theme="light"] .search-input {{
    background: oklch(0.0 0.0 0.0 / 0.04);
    border-color: oklch(0.0 0.0 0.0 / 0.08);
    color: oklch(0.2 0.003 90);
  }}

  [data-theme="light"] .island-divider {{ background: oklch(0.0 0.0 0.0 / 0.06); }}
  [data-theme="light"] .dock-search-divider {{ background: oklch(0.0 0.0 0.0 / 0.06); }}

  .dock-btn:hover {{
    color: oklch(1.0 0.000 0.0);
    background: oklch(1.0 0.0 0.0 / 0.08);
    transform: scale(1.12);
  }}

  .dock-btn:active {{
    transform: scale(0.93);
    transition-duration: 60ms;
  }}

  .dock button {{
    color: oklch(0.85 0.000 0.0) !important;
    opacity: 1 !important;
  }}

  .dock button:hover {{
    color: oklch(1.0 0.000 0.0) !important;
    background: oklch(1.0 0.0 0.0 / 0.08) !important;
    transform: scale(1.12);
    transition: all 180ms var(--cx-ease-spring);
  }}

  .dock button:active {{
    transform: scale(0.93);
    transition-duration: 60ms;
  }}

  .dock button svg,
  .dock-btn svg {{
    opacity: 0.9;
  }}

  .dock button:hover svg,
  .dock-btn:hover svg {{
    opacity: 1;
  }}

  /* ── Search expansion — dock grows vertically ──── */
  .dock {{
    flex-direction: column;
    align-items: stretch;
  }}

  .dock-icons {{
    display: flex;
    align-items: center;
    gap: 6px;
    justify-content: center;
  }}

  .dock-search {{
    max-height: 0;
    opacity: 0;
    overflow: hidden;
    transition: max-height 350ms var(--cx-ease-spring),
                opacity 250ms ease,
                padding 350ms var(--cx-ease-spring);
    padding: 0 4px;
  }}

  .dock.expanded .dock-search {{
    max-height: 320px;
    opacity: 1;
    padding: 12px 4px 4px;
  }}

  .dock-search-divider {{
    height: 1px;
    background: oklch(1.0 0.0 0.0 / 0.06);
    margin-bottom: 12px;
  }}

  .search-input {{
    width: 100%;
    background: oklch(1.0 0.0 0.0 / 0.06);
    border: 1px solid oklch(1.0 0.0 0.0 / 0.08);
    border-radius: 10px;
    padding: 10px 14px;
    color: oklch(0.88 0.0 0.0);
    font-family: var(--cx-font-sans, 'Geist', system-ui, sans-serif);
    font-size: 14px;
    outline: none;
    transition: border-color 200ms var(--cx-ease-spring);
    resize: none;
    overflow: hidden;
    line-height: 1.5;
    min-height: 40px;
    max-height: 120px;
  }}

  .search-input:focus {{
    border-color: oklch(1.0 0.0 0.0 / 0.15);
  }}

  .search-input::placeholder {{
    color: oklch(0.45 0.0 0.0);
    font-size: 13px;
  }}

  .search-results:empty {{
    display: none;
  }}

  .search-results {{
    color: oklch(0.5 0.0 0.0);
    font-size: 13px;
    padding-top: 8px;
  }}
  .search-result {{
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 100%;
    padding: 10px 12px;
    border: none;
    background: none;
    color: inherit;
    text-align: left;
    cursor: pointer;
    border-radius: 10px;
    transition: background 100ms ease-out;
    -webkit-appearance: none;
  }}
  .search-result:hover {{
    background: oklch(1.0 0 0 / 0.08);
  }}
  .search-result:active {{
    background: oklch(1.0 0 0 / 0.14);
  }}
  .search-result-name {{
    font-size: 14px;
    font-weight: 500;
    color: oklch(0.92 0 0);
  }}
  .search-result-desc {{
    font-size: 11px;
    color: oklch(0.5 0 0);
  }}
  .search-empty {{
    padding: 12px;
    text-align: center;
    color: oklch(0.45 0 0);
    font-size: 13px;
  }}
  [data-theme="light"] .search-result-name {{ color: oklch(0.15 0 0); }}
  [data-theme="light"] .search-result-desc {{ color: oklch(0.5 0 0); }}
  [data-theme="light"] .search-result:hover {{ background: oklch(0 0 0 / 0.05); }}

  /* ── High contrast ────────────────────────────── */
  @media (forced-colors: active) {{
    .desktop {{ background: Canvas !important; }}
    .dock, .control-bar, .control-island, .notif, .osd, .profile-section {{
      background: Canvas !important;
      border: 1px solid ButtonText !important;
      color: CanvasText !important;
      backdrop-filter: none !important;
      -webkit-backdrop-filter: none !important;
    }}
    .bar-btn, .dock-btn, .dock button, .qs-tile-body, .qs-tile-toggle,
    .profile-menu-item, .profile-row, .cal-day, .search-result, .nav-item {{
      color: ButtonText !important;
      forced-color-adjust: none;
    }}
    .bar-btn:hover, .dock button:hover, .qs-tile-body:hover,
    .qs-tile-toggle:hover, .profile-menu-item:hover, .profile-row:hover,
    .cal-day:hover, .search-result:hover, .nav-item:hover {{
      background: Highlight !important;
      color: HighlightText !important;
    }}
    :focus-visible {{
      outline: 2px solid Highlight !important;
      outline-offset: 2px !important;
    }}
    .qs-tile.active {{
      background: Highlight !important;
      color: HighlightText !important;
      border-color: Highlight !important;
    }}
    .cal-day.today, .cal-day-selected {{
      background: Highlight !important;
      color: HighlightText !important;
    }}
  }}

  /* ── Lock Screen Overlay ─────────────────────────── */
  .lock-overlay {{
    position: absolute;
    inset: 0;
    z-index: 500;
    display: flex;
    align-items: center;
    justify-content: center;
    background: url('{bg_data_url}') center/cover no-repeat;
  }}
  .lock-overlay .lock-backdrop {{
    position: absolute;
    inset: 0;
    backdrop-filter: blur(40px) saturate(1.4) brightness(0.5);
    -webkit-backdrop-filter: blur(40px) saturate(1.4) brightness(0.5);
  }}
  .lock-overlay .lock-clock {{
    position: absolute;
    top: 60px;
    left: 50%;
    transform: translateX(-50%);
    text-align: center;
    z-index: 1;
    color: oklch(0.95 0 0);
  }}
  .lock-overlay .lock-time {{
    font-size: 64px;
    font-weight: 200;
    letter-spacing: -2px;
    font-variant-numeric: tabular-nums;
    line-height: 1;
  }}
  .lock-overlay .lock-date {{
    font-size: 16px;
    color: oklch(0.7 0 0);
    margin-top: 8px;
  }}
  .lock-overlay .lock-card {{
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    padding: 40px;
    border-radius: 24px;
    background: oklch(0.15 0 0 / 0.6);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid oklch(1.0 0 0 / 0.06);
    min-width: 320px;
  }}
  .lock-overlay .lock-avatar {{
    width: 80px; height: 80px;
    border-radius: 50%;
    background: oklch(0.45 0.15 250 / 0.4);
    display: flex; align-items: center; justify-content: center;
    font-weight: 700; font-size: 32px; color: oklch(0.9 0.05 250);
  }}
  .lock-overlay .lock-name {{
    font-size: 18px; font-weight: 600; color: oklch(0.95 0 0);
  }}
  .lock-overlay .lock-input {{
    width: 260px; padding: 12px 16px;
    border-radius: 12px;
    border: 1px solid oklch(1.0 0 0 / 0.1);
    background: oklch(1.0 0 0 / 0.06);
    color: oklch(0.95 0 0);
    font-family: inherit; font-size: 15px;
    outline: none; text-align: center; letter-spacing: 4px;
    transition: border-color 100ms ease-out;
  }}
  .lock-overlay .lock-input:focus {{
    border-color: oklch(0.6 0.15 250);
  }}
  .lock-overlay .lock-input.shake {{
    animation: lock-shake 400ms ease-out;
  }}
  @keyframes lock-shake {{
    0%, 100% {{ transform: translateX(0); }}
    20% {{ transform: translateX(-12px); }}
    40% {{ transform: translateX(10px); }}
    60% {{ transform: translateX(-6px); }}
    80% {{ transform: translateX(4px); }}
  }}
  .lock-overlay .lock-status {{
    font-size: 13px; color: oklch(0.65 0.12 25); min-height: 20px;
  }}
</style>
</head>
<body>
  <a href="#dock" class="skip-link">Skip to dock</a>
  <a href="#controlBar" class="skip-link">Skip to settings</a>
  {sprites}
  <div class="desktop" role="application">
    <!-- Lock screen overlay (preview mode) -->
    <div class="lock-overlay" id="lockOverlay" style="display:none">
      <div class="lock-backdrop"></div>
      <div class="lock-clock">
        <div class="lock-time" id="lockTime2">00:00</div>
        <div class="lock-date" id="lockDate2">Monday, 1 January</div>
      </div>
      <div class="lock-card">
        <div class="lock-avatar">D</div>
        <div class="lock-name">Dan</div>
        <input type="password" class="lock-input" id="lockPass2" autocomplete="current-password" aria-label="Password" placeholder="Password">
        <div class="lock-status" id="lockStatus2" aria-live="assertive"></div>
      </div>
    </div>

    <!-- Notification stack -->
    <div class="notif-stack" id="notifStack" role="log" aria-live="polite" aria-label="Notifications"></div>

    <!-- OSD overlay -->
    <div class="osd" id="osd" role="status" aria-live="polite">
      <svg class="osd-icon" id="osdIcon" width="24" height="24" viewBox="0 0 256 256" fill="currentColor"></svg>
      <div class="osd-track"><div class="osd-fill" id="osdFill"></div></div>
      <span class="osd-value" id="osdValue">75%</span>
    </div>

    <!-- Control Bar (top-right pill) + Island -->
    <div class="control-bar-container">
      <div class="control-bar" id="controlBar" role="status" aria-label="System controls">
        {control_bar_buttons}
      </div>

      <div class="control-island" id="controlIsland" role="dialog" aria-label="Quick settings">
        {island_content}
      </div>
    </div>

    <!-- Dock (bottom-center, attached, expandable) -->
    <div class="dock-container">
      <nav class="dock" id="dock" role="toolbar" aria-label="Collet Dock">
        <div class="dock-icons">
          {dock_buttons}
        </div>
        <div class="dock-search">
          <div class="dock-search-divider"></div>
          {search_input}
          <div class="search-results" id="searchResults"></div>
        </div>
      </nav>
    </div>
  </div>

  <script>
    // Clock — Date + Time (e.g. "Mon 7 Apr 14:32")
    function u(){{const n=new Date();
    const day=['Sun','Mon','Tue','Wed','Thu','Fri','Sat'][n.getDay()];
    const mon=['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'][n.getMonth()];
    const t=String(n.getHours()).padStart(2,'0')+':'+String(n.getMinutes()).padStart(2,'0');
    document.getElementById('c').textContent=day+' '+n.getDate()+' '+mon+' '+t;}}
    u();setInterval(u,1000);

    // Theme toggle — dark/light mode via event delegation
    let isDark = true;
    const desktop = document.querySelector('.desktop');
    const htmlEl = document.documentElement;
    document.addEventListener('click', (e) => {{
      // Dark mode switch — the only role="switch" in the island
      const switchBtn = e.target.closest('.control-island button[role="switch"]');
      if (!switchBtn) return;

      isDark = !isDark;
      htmlEl.setAttribute('data-theme', isDark ? 'dark' : 'light');

      // Flip the switch visually (no WASM runtime to do this)
      switchBtn.setAttribute('aria-checked', String(isDark));
      const thumb = switchBtn.querySelector('span');
      if (thumb) {{
        thumb.style.transform = isDark ? 'translateX(100%)' : 'translateX(0)';
      }}
      // Also flip the hidden checkbox
      const checkbox = switchBtn.closest('label')?.querySelector('input[type="checkbox"]');
      if (checkbox) checkbox.checked = isDark;

      if (isDark) {{
        desktop.style.background = "url('{bg_data_url}') center/cover no-repeat, oklch(0.13 0 0)";
      }} else {{
        desktop.style.background = "url('{bg_light_url}') center/cover no-repeat, oklch(0.95 0.005 90)";
      }}

      const dock = document.querySelector('.dock');
      const bar = document.querySelector('.control-bar');
      const islandEl = document.querySelector('.control-island');
      if (isDark) {{
        dock.style.background = 'oklch(0.145 0 0 / 0.88)';
        bar.style.background = 'oklch(0.185 0 0 / 0.78)';
        islandEl.style.background = 'oklch(0.185 0 0 / 0.88)';
      }} else {{
        dock.style.background = 'oklch(1.0 0 0 / 0.75)';
        bar.style.background = 'oklch(1.0 0 0 / 0.68)';
        islandEl.style.background = 'oklch(1.0 0 0 / 0.82)';
      }}
      localStorage.setItem('collet-theme', isDark ? 'dark' : 'light');
    }});

    // Quick settings tile toggles (Wi-Fi, Bluetooth on/off)
    document.addEventListener('click', (e) => {{
      const toggleBtn = e.target.closest('.qs-tile-toggle');
      if (!toggleBtn) return;
      e.stopPropagation();
      const tile = toggleBtn.closest('.qs-tile');
      const isOn = toggleBtn.getAttribute('aria-pressed') === 'true';
      toggleBtn.setAttribute('aria-pressed', String(!isOn));
      tile.classList.toggle('active', !isOn);
      const detail = tile.querySelector('.qs-tile-detail');
      if (detail) detail.textContent = !isOn ? 'Connected' : 'Off';
    }});

    // Control Island toggle
    const controlBar = document.querySelector('#controlBar');
    const island = document.querySelector('#controlIsland');
    let islandOpen = false;

    function toggleIsland() {{
      islandOpen = !islandOpen;
      island.classList.toggle('open', islandOpen);
      if (islandOpen) {{
        island.setAttribute('aria-modal', 'true');
        setTimeout(() => {{
          const first = island.querySelector('button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])');
          if (first) first.focus();
        }}, 100);
      }} else {{
        island.removeAttribute('aria-modal');
        controlBar.focus();
      }}
    }}

    controlBar.addEventListener('click', (e) => {{
      e.stopPropagation();
      toggleIsland();
    }});

    // Focus trap inside island
    island.addEventListener('keydown', (e) => {{
      if (e.key === 'Tab') {{
        const focusable = island.querySelectorAll('button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])');
        if (focusable.length === 0) return;
        const first = focusable[0];
        const last = focusable[focusable.length - 1];
        if (e.shiftKey && document.activeElement === first) {{
          e.preventDefault();
          last.focus();
        }} else if (!e.shiftKey && document.activeElement === last) {{
          e.preventDefault();
          first.focus();
        }}
      }}
    }});

    document.addEventListener('click', (e) => {{
      if (islandOpen && !e.target.closest('.control-island') && !e.target.closest('.control-bar')) {{
        toggleIsland();
      }}
    }});

    // Calendar with week numbers + focus mode
    let calFocusRow = null;
    function isoWeek(d) {{
      const t = new Date(Date.UTC(d.getFullYear(), d.getMonth(), d.getDate()));
      t.setUTCDate(t.getUTCDate() + 4 - (t.getUTCDay() || 7));
      const y1 = new Date(Date.UTC(t.getUTCFullYear(), 0, 1));
      return Math.ceil((((t - y1) / 86400000) + 1) / 7);
    }}
    function buildCal() {{
      const now = new Date();
      const year = now.getFullYear();
      const month = now.getMonth();
      const today = now.getDate();
      const monthNames = ['January','February','March','April','May','June','July','August','September','October','November','December'];
      document.querySelector('#calHeader').textContent = monthNames[month] + ' ' + year;

      const firstDay = new Date(year, month, 1).getDay();
      const daysInMonth = new Date(year, month + 1, 0).getDate();
      const daysInPrev = new Date(year, month, 0).getDate();
      const startDay = (firstDay === 0) ? 6 : firstDay - 1;

      // Header row
      let html = '<span class="cal-wk" data-row="h">Wk</span>';
      const dayNames = ['Mo','Tu','We','Th','Fr','Sa','Su'];
      html += dayNames.map(d => '<span class="cal-day-name" data-row="h">' + d + '</span>').join('');

      // Build all days as flat array first
      const cells = [];
      for (let i = startDay - 1; i >= 0; i--) {{
        cells.push({{ day: daysInPrev - i, cls: 'cal-day other-month', month: month - 1 }});
      }}
      for (let d = 1; d <= daysInMonth; d++) {{
        cells.push({{ day: d, cls: d === today ? 'cal-day today' : 'cal-day', month: month }});
      }}
      const totalCells = cells.length;
      const remaining = (7 - (totalCells % 7)) % 7;
      for (let i = 1; i <= remaining; i++) {{
        cells.push({{ day: i, cls: 'cal-day other-month', month: month + 1 }});
      }}

      // Render rows with week numbers
      for (let i = 0; i < cells.length; i += 7) {{
        const row = Math.floor(i / 7);
        const mondayDate = new Date(year, cells[i].month, cells[i].day);
        const wk = isoWeek(mondayDate);
        html += '<span class="cal-wk" data-row="' + row + '">' + wk + '</span>';
        for (let j = 0; j < 7; j++) {{
          const c = cells[i + j];
          html += '<span class="' + c.cls + '" data-row="' + row + '" data-day="' + c.day + '">' + c.day + '</span>';
        }}
      }}
      document.querySelector('#calGrid').innerHTML = html;
    }}
    buildCal();

    // Calendar focus mode
    const calGrid = document.getElementById('calGrid');
    const calAgenda = document.getElementById('calAgenda');
    const calWrap = document.getElementById('calWrap');
    const agendaEvents = [
      {{ time: '09:00', title: 'Team standup', color: 'oklch(0.6 0.15 250)' }},
      {{ time: '11:30', title: 'Design review', color: 'oklch(0.6 0.12 145)' }},
      {{ time: '14:00', title: 'Sprint planning', color: 'oklch(0.6 0.1 40)' }},
    ];

    function exitCalFocus() {{
      calFocusRow = null;
      Array.from(calGrid.children).forEach(el => el.style.display = '');
      calAgenda.style.display = 'none';
      calWrap.classList.remove('cal-focus');
    }}

    calGrid.addEventListener('click', (e) => {{
      const day = e.target.closest('.cal-day:not(.other-month)');
      if (!day) return;
      e.stopPropagation();
      const row = day.dataset.row;

      // Same day clicked again → exit focus
      if (day.classList.contains('cal-day-selected')) {{
        exitCalFocus();
        return;
      }}

      // Different day, same week → just move selection
      if (calFocusRow === row) {{
        calGrid.querySelectorAll('.cal-day').forEach(d => d.classList.remove('cal-day-selected'));
        day.classList.add('cal-day-selected');
        return;
      }}

      // New week → enter/switch focus
      calFocusRow = row;
      calWrap.classList.add('cal-focus');
      Array.from(calGrid.children).forEach(el => {{
        if (el.dataset.row !== row && el.dataset.row !== 'h') {{
          el.style.display = 'none';
        }} else {{
          el.style.display = '';
        }}
      }});

      calGrid.querySelectorAll('.cal-day').forEach(d => d.classList.remove('cal-day-selected'));
      day.classList.add('cal-day-selected');

      // Show agenda
      calAgenda.style.display = '';
      calAgenda.innerHTML = agendaEvents.map(ev =>
        '<div class="cal-agenda-item"><span class="cal-agenda-dot" style="background:' + ev.color + '"></span><span class="cal-agenda-time">' + ev.time + '</span><span class="cal-agenda-title">' + ev.title + '</span></div>'
      ).join('');
    }});

    // Profile section expand/collapse
    const profileSection = document.getElementById('profileSection');
    const profileToggle = document.getElementById('profileToggle');
    profileToggle.addEventListener('click', (e) => {{
      e.stopPropagation();
      profileSection.classList.toggle('open');
      profileToggle.setAttribute('aria-expanded', profileSection.classList.contains('open'));
    }});

    // Slider fill — update --slider-percent on drag so the track gradient follows the thumb
    document.querySelectorAll('input[data-slider]').forEach(s => {{
      s.addEventListener('input', () => {{
        const pct = (s.value - s.min) / (s.max - s.min) * 100;
        s.style.setProperty('--slider-percent', pct);
        const valEl = s.closest('[part="base"]')?.querySelector('[data-slider-value]');
        if (valEl) valEl.textContent = Math.round(s.value) + '%';
      }});
    }});

    // Roving tabindex for dock toolbar
    const dockIcons = document.querySelector('.dock-icons');
    if (dockIcons) {{
      const dockBtns = dockIcons.querySelectorAll('button');
      dockBtns.forEach((btn, i) => btn.setAttribute('tabindex', i === 0 ? '0' : '-1'));
      dockIcons.addEventListener('keydown', (e) => {{
        const btns = Array.from(dockIcons.querySelectorAll('button'));
        const idx = btns.indexOf(document.activeElement);
        if (idx === -1) return;
        let next = -1;
        if (e.key === 'ArrowRight' || e.key === 'ArrowDown') next = (idx + 1) % btns.length;
        else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') next = (idx - 1 + btns.length) % btns.length;
        else if (e.key === 'Home') next = 0;
        else if (e.key === 'End') next = btns.length - 1;
        if (next !== -1) {{
          e.preventDefault();
          btns[idx].setAttribute('tabindex', '-1');
          btns[next].setAttribute('tabindex', '0');
          btns[next].focus();
        }}
      }});
    }}

    // Dock expansion — search surface grows from within the dock
    const dock = document.querySelector('#dock');
    const searchInput = document.querySelector('#dock-search-textarea');
    let expanded = false;

    // Enter = submit search, Shift+Enter = new line (ChatInput handles auto-expand)
    if (searchInput) {{
      searchInput.addEventListener('keydown', (e) => {{
        if (e.key === 'Enter' && !e.shiftKey) {{
          e.preventDefault();
          const q = searchInput.value.trim();
          if (q) {{
            console.log('[collet] search:', q);
            if (window.ipc) {{
              window.ipc.postMessage(JSON.stringify({{ action: 'search', payload: {{ query: q }} }}));
            }}
          }}
        }}
      }});
    }}

    function toggleDock() {{
      expanded = !expanded;
      dock.classList.toggle('expanded', expanded);
      if (expanded) {{
        setTimeout(() => searchInput.focus(), 250);
      }} else {{
        searchInput.value = '';
        searchInput.style.height = 'auto';
        document.getElementById('searchResults').innerHTML = '';
      }}
    }}

    // Close on Escape
    document.addEventListener('keydown', (e) => {{
      // Escape — priority chain: island > calendar focus > dock
      if (e.key === 'Escape') {{
        if (islandOpen) {{ toggleIsland(); return; }}
        if (calFocusRow !== null) {{ exitCalFocus(); return; }}
        if (expanded) toggleDock();
        return;
      }}
      // Super/Meta — toggle dock search
      if (e.key === 'Meta' && !e.ctrlKey && !e.shiftKey && !e.altKey) {{
        return; // Meta keydown fires before keyup; handle on keyup instead
      }}
      // Ctrl+L / Cmd+L — lock screen
      if (e.key === 'l' && (e.ctrlKey || e.metaKey)) {{
        e.preventDefault();
        const lockBtn = document.querySelector('[data-ipc="lock"]');
        if (lockBtn) lockBtn.click();
        return;
      }}
      // Space — toggle island when control bar focused
      if (e.key === ' ' && document.activeElement === controlBar) {{
        e.preventDefault();
        toggleIsland();
        return;
      }}
    }});

    // Meta key — toggle dock on keyup (not keydown, avoids double-fire)
    document.addEventListener('keyup', (e) => {{
      if (e.key === 'Meta' && !e.ctrlKey && !e.shiftKey && !e.altKey) {{
        toggleDock();
      }}
    }});

    // Single click handler for everything
    document.addEventListener('click', (e) => {{
      // Check for IPC buttons first
      const btn = e.target.closest('[data-ipc]');
      if (btn) {{
        const action = btn.dataset.ipc;
        if (action === 'search') {{
          toggleDock();
          return;
        }}
        if (action === 'lock') {{
          const overlay = document.getElementById('lockOverlay');
          overlay.style.display = '';
          const lockClock = () => {{
            const n = new Date();
            document.getElementById('lockTime2').textContent = String(n.getHours()).padStart(2,'0') + ':' + String(n.getMinutes()).padStart(2,'0');
            const days = ['Sunday','Monday','Tuesday','Wednesday','Thursday','Friday','Saturday'];
            const months = ['January','February','March','April','May','June','July','August','September','October','November','December'];
            document.getElementById('lockDate2').textContent = days[n.getDay()] + ', ' + n.getDate() + ' ' + months[n.getMonth()];
          }};
          lockClock();
          const lockTimer = setInterval(lockClock, 1000);
          const lockPass = document.getElementById('lockPass2');
          const lockStatus = document.getElementById('lockStatus2');
          setTimeout(() => lockPass.focus(), 100);
          lockPass.addEventListener('keydown', function handler(e) {{
            if (e.key === 'Enter') {{
              if (lockPass.value) {{
                overlay.style.display = 'none';
                lockPass.value = '';
                lockStatus.textContent = '';
                clearInterval(lockTimer);
                lockPass.removeEventListener('keydown', handler);
              }} else {{
                lockStatus.textContent = 'Enter your password';
                lockPass.classList.add('shake');
                setTimeout(() => lockPass.classList.remove('shake'), 450);
              }}
            }}
          }});
          return;
        }}
        if (action === 'open_settings') {{
          window.open('settings.html', '_blank', 'width=900,height=600');
          return;
        }}
        const appId = btn.dataset.appId || '';
        console.log('[collet]', action, appId);
        if (window.ipc) {{
          window.ipc.postMessage(JSON.stringify({{ action, payload: {{ app_id: appId }} }}));
        }}
        return;
      }}

      // Close dock if clicking outside it
      if (expanded && !e.target.closest('.dock')) {{
        toggleDock();
      }}
    }});

    // Notification system
    const notifStack = document.getElementById('notifStack');
    let notifId = 0;
    function showNotif(title, text, iconSvg) {{
      const n = document.createElement('div');
      n.className = 'notif';
      n.setAttribute('role', 'alert');
      n.innerHTML = '<div class="notif-icon">' + (iconSvg || '<svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M221.8,175.94C216.25,166.38,208,139.33,208,104a80,80,0,1,0-160,0c0,35.34-8.26,62.38-13.81,71.94A16,16,0,0,0,48,200H88.81a40,40,0,0,0,78.38,0H208a16,16,0,0,0,13.8-24.06Z"/></svg>') + '</div><div class="notif-body"><div class="notif-title">' + title + '</div><div class="notif-text">' + text + '</div></div><span class="notif-time">now</span>';
      n.addEventListener('click', () => {{
        n.classList.add('dismiss');
        setTimeout(() => n.remove(), 250);
      }});
      notifStack.prepend(n);
      setTimeout(() => {{
        if (n.parentNode) {{
          n.classList.add('dismiss');
          setTimeout(() => n.remove(), 250);
        }}
      }}, 5000);
    }}
    // Demo notification on first island open
    let notifShown = false;

    // OSD system
    const osd = document.getElementById('osd');
    const osdIcon = document.getElementById('osdIcon');
    const osdFill = document.getElementById('osdFill');
    const osdValue = document.getElementById('osdValue');
    let osdTimer = null;
    const sunPath = 'M120,40V16a8,8,0,0,1,16,0V40a8,8,0,0,1-16,0Zm72,88a64,64,0,1,1-64-64A64.07,64.07,0,0,1,192,128Zm-16,0a48,48,0,1,0-48,48A48.05,48.05,0,0,0,176,128ZM58.34,69.66A8,8,0,0,0,69.66,58.34l-16-16A8,8,0,0,0,42.34,53.66Zm0,116.68-16,16a8,8,0,0,0,11.32,11.32l16-16a8,8,0,0,0-11.32-11.32ZM192,72a8,8,0,0,0,5.66-2.34l16-16a8,8,0,0,0-11.32-11.32l-16,16A8,8,0,0,0,192,72Zm5.66,114.34a8,8,0,0,0-11.32,11.32l16,16a8,8,0,0,0,11.32-11.32ZM48,128a8,8,0,0,0-8-8H16a8,8,0,0,0,0,16H40A8,8,0,0,0,48,128Zm80,80a8,8,0,0,0-8,8v24a8,8,0,0,0,16,0V216A8,8,0,0,0,128,208Zm112-88H216a8,8,0,0,0,0,16h24a8,8,0,0,0,0-16Z';
    const volPath = 'M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81ZM32,96H72v64H32ZM144,207.64,88,164.09V91.91l56-43.55Zm54-106.08a40,40,0,0,1,0,52.88,8,8,0,0,1-12-10.58,24,24,0,0,0,0-31.72,8,8,0,0,1,12-10.58ZM248,128a79.9,79.9,0,0,1-20.37,53.34,8,8,0,0,1-11.92-10.67,64,64,0,0,0,0-85.33,8,8,0,1,1,11.92-10.67A79.83,79.83,0,0,1,248,128Z';
    function showOSD(type, pct) {{
      osdIcon.innerHTML = '<path d="' + (type === 'brightness' ? sunPath : volPath) + '"/>';
      osdFill.style.width = pct + '%';
      osdValue.textContent = Math.round(pct) + '%';
      osd.classList.add('visible');
      clearTimeout(osdTimer);
      osdTimer = setTimeout(() => osd.classList.remove('visible'), 1500);
    }}
    // Hook sliders to OSD
    document.querySelectorAll('input[data-slider]').forEach(s => {{
      s.addEventListener('input', () => {{
        const pct = (s.value - s.min) / (s.max - s.min) * 100;
        const type = s.id.includes('brightness') ? 'brightness' : 'volume';
        showOSD(type, pct);
      }});
    }});

    // Search results rendering — queries backend for .desktop apps
    const searchResultsEl = document.getElementById('searchResults');
    let searchDebounce = null;
    function renderSearchResults(results) {{
      if (results.length === 0) {{
        const q = searchInput ? searchInput.value.trim() : '';
        searchResultsEl.innerHTML = q ? '<div class="search-empty">No results for "' + q + '"</div>' : '';
        return;
      }}
      searchResultsEl.innerHTML = results.map(a =>
        '<button class="search-result" data-ipc="launch_app" data-app-id="' + (a.id || a.name.toLowerCase().replace(/\s/g,'-')) + '">' +
        '<span class="search-result-name">' + a.name + '</span>' +
        (a.description ? '<span class="search-result-desc">' + a.description + '</span>' : '') +
        '</button>'
      ).join('');
    }}
    if (searchInput) {{
      searchInput.addEventListener('input', () => {{
        const q = searchInput.value.trim();
        if (!q) {{ searchResultsEl.innerHTML = ''; return; }}
        clearTimeout(searchDebounce);
        searchDebounce = setTimeout(() => {{
          if (window.ipc) {{
            // Send search query to backend — results rendered via IPC response
            window.ipc.postMessage(JSON.stringify({{ action: 'search_apps', payload: {{ query: q }} }}));
          }}
          // Fallback for preview: local filter
          const fallback = [
            {{ name: 'Files', description: 'Browse and manage files', id: 'org.gnome.Nautilus' }},
            {{ name: 'Firefox', description: 'Web browser', id: 'firefox' }},
            {{ name: 'Terminal', description: 'Command line interface', id: 'org.gnome.Terminal' }},
            {{ name: 'Settings', description: 'System preferences', id: 'collet-settings' }},
            {{ name: 'Text Editor', description: 'Edit text files', id: 'org.gnome.TextEditor' }},
            {{ name: 'Calculator', description: 'Basic calculations', id: 'org.gnome.Calculator' }},
          ];
          const matches = fallback.filter(a =>
            a.name.toLowerCase().includes(q.toLowerCase()) ||
            a.description.toLowerCase().includes(q.toLowerCase())
          );
          renderSearchResults(matches);
        }}, 150);
      }});
    }}

    // localStorage theme persistence
    const savedTheme = localStorage.getItem('collet-theme');
    if (savedTheme === 'light') {{
      isDark = false;
      htmlEl.setAttribute('data-theme', 'light');
      desktop.style.background = "url('{bg_light_url}') center/cover no-repeat, oklch(0.95 0.005 90)";
      document.querySelector('.dock').style.background = 'oklch(1.0 0 0 / 0.75)';
      document.querySelector('.control-bar').style.background = 'oklch(1.0 0 0 / 0.68)';
      document.querySelector('.control-island').style.background = 'oklch(1.0 0 0 / 0.82)';
      const sw = document.querySelector('.control-island button[role="switch"]');
      if (sw) {{
        sw.setAttribute('aria-checked', 'false');
        const t = sw.querySelector('span');
        if (t) t.style.transform = 'translateX(0)';
        const cb = sw.closest('label')?.querySelector('input[type="checkbox"]');
        if (cb) cb.checked = false;
      }}
    }}

    // System state alert monitoring
    function checkSystemAlerts(state) {{
      if (state.battery.percentage < 10 && !state.battery.charging) {{
        showNotif('Battery critically low', 'Connect charger — ' + state.battery.percentage + '% remaining');
      }}
    }}

    // Demo notification after 2s
    setTimeout(() => {{
      showNotif('Welcome to Collet OS', 'Your desktop is ready. Click the control bar to access settings.');
    }}, 2000);
  </script>
</body>
</html>"##
    )
}

/// Render the Settings window — a standalone page for system configuration.
/// Opens in its own wry window (Linux) or browser tab (macOS preview).
pub fn render_settings() -> String {
    SpriteCollector::reset();

    // Dark mode switch
    let theme_switch = Switch::new("settings-theme", "Dark mode")
        .shape(SwitchShape::Pill)
        .size(ComponentSize::Md)
        .checked(true)
        .render();

    // Build settings sections using Collet components
    let display_brightness = Slider::new("display-brightness", "Brightness")
        .value(75.0).min(0.0).max(100.0).step(1.0)
        .shape(SliderShape::Pill).size(ComponentSize::Md)
        .show_value(true).value_text("75%")
        .render();

    let display_nightlight = Switch::new("nightlight", "Night Light")
        .shape(SwitchShape::Pill).size(ComponentSize::Md)
        .checked(false)
        .render();

    let sound_output = Slider::new("sound-output", "Output volume")
        .value(60.0).min(0.0).max(100.0).step(1.0)
        .shape(SliderShape::Pill).size(ComponentSize::Md)
        .show_value(true).value_text("60%")
        .render();

    let sound_input = Slider::new("sound-input", "Input volume")
        .value(80.0).min(0.0).max(100.0).step(1.0)
        .shape(SliderShape::Pill).size(ComponentSize::Md)
        .show_value(true).value_text("80%")
        .render();

    let wifi_switch = Switch::new("settings-wifi", "Wi-Fi")
        .shape(SwitchShape::Pill).size(ComponentSize::Md)
        .checked(true)
        .render();

    let bt_switch = Switch::new("settings-bt", "Bluetooth")
        .shape(SwitchShape::Pill).size(ComponentSize::Md)
        .checked(false)
        .render();

    let autolock_switch = Switch::new("autolock", "Automatic screen lock")
        .shape(SwitchShape::Pill).size(ComponentSize::Md)
        .checked(true)
        .render();

    let sep = Separator::new("settings-sep").render();

    let sprites = SpriteCollector::take_sprite();
    let tokens_css = design_system::generate_tokens_css();

    format!(
        r##"<!DOCTYPE html>
<html lang="en" data-theme="dark">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<script src="https://cdn.tailwindcss.com"></script>
<style>{tokens_css}</style>
<style>
  :root {{ --cx-ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1); }}
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  html, body {{ height: 100%; }}
  body {{
    font-family: var(--cx-font-sans, 'Geist', 'Inter', system-ui, sans-serif);
    background: oklch(0.12 0 0);
    color: oklch(0.88 0 0);
    display: flex;
  }}
  [data-theme="light"] body {{
    background: oklch(0.955 0.005 90);
    color: oklch(0.2 0 0);
  }}

  /* Sidebar nav */
  .settings-nav {{
    width: 240px;
    flex-shrink: 0;
    padding: 20px 0;
    border-right: 1px solid oklch(1.0 0 0 / 0.06);
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow-y: auto;
  }}
  [data-theme="light"] .settings-nav {{ border-right-color: oklch(0 0 0 / 0.08); }}

  .nav-header {{
    padding: 8px 20px 16px;
    font-size: 18px;
    font-weight: 700;
    letter-spacing: -0.02em;
  }}

  .nav-item {{
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 20px;
    font-size: 13px;
    font-weight: 500;
    color: oklch(0.65 0 0);
    cursor: pointer;
    border: none;
    background: none;
    width: 100%;
    text-align: left;
    transition: background 100ms ease-out, color 100ms ease-out;
    -webkit-appearance: none;
  }}
  .nav-item:hover {{ background: oklch(1.0 0 0 / 0.06); color: oklch(0.9 0 0); }}
  .nav-item.active {{ background: oklch(1.0 0 0 / 0.08); color: oklch(0.95 0 0); font-weight: 600; }}
  .nav-item svg {{ opacity: 0.6; flex-shrink: 0; }}
  .nav-item.active svg {{ opacity: 1; }}
  [data-theme="light"] .nav-item {{ color: oklch(0.45 0 0); }}
  [data-theme="light"] .nav-item:hover {{ background: oklch(0 0 0 / 0.04); color: oklch(0.2 0 0); }}
  [data-theme="light"] .nav-item.active {{ background: oklch(0 0 0 / 0.06); color: oklch(0.1 0 0); }}

  /* Content area */
  .settings-content {{
    flex: 1;
    padding: 24px 32px;
    overflow-y: auto;
  }}

  .settings-page {{ display: none; }}
  .settings-page.active {{ display: block; }}

  .page-title {{
    font-size: 22px;
    font-weight: 700;
    letter-spacing: -0.02em;
    margin-bottom: 24px;
  }}

  .section {{
    margin-bottom: 28px;
  }}
  .section-title {{
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: oklch(0.5 0 0);
    margin-bottom: 12px;
  }}
  [data-theme="light"] .section-title {{ color: oklch(0.45 0 0); }}

  .setting-row {{
    padding: 4px 0;
  }}

  .info-row {{
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 0;
    font-size: 13px;
  }}
  .info-label {{ color: oklch(0.55 0 0); }}
  .info-value {{ color: oklch(0.85 0 0); font-weight: 500; }}
  [data-theme="light"] .info-label {{ color: oklch(0.45 0 0); }}
  [data-theme="light"] .info-value {{ color: oklch(0.2 0 0); }}

  /* Network list */
  .network-item {{
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border-radius: 10px;
    cursor: pointer;
    transition: background 100ms ease-out;
  }}
  .network-item:hover {{ background: oklch(1.0 0 0 / 0.06); }}
  [data-theme="light"] .network-item:hover {{ background: oklch(0 0 0 / 0.04); }}
  .network-name {{ font-size: 13px; font-weight: 500; flex: 1; }}
  .network-status {{ font-size: 11px; color: oklch(0.5 0 0); }}
  .network-connected {{ color: oklch(0.6 0.15 145); }}

  /* Profile card */
  .profile-card {{
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 20px;
    border-radius: 16px;
    background: oklch(1.0 0 0 / 0.04);
    border: 1px solid oklch(1.0 0 0 / 0.06);
    margin-bottom: 24px;
  }}
  [data-theme="light"] .profile-card {{
    background: oklch(0 0 0 / 0.02);
    border-color: oklch(0 0 0 / 0.06);
  }}
  .profile-card-avatar {{
    width: 56px; height: 56px;
    border-radius: 50%;
    background: oklch(0.45 0.15 250 / 0.4);
    display: flex; align-items: center; justify-content: center;
    font-weight: 700; font-size: 22px; color: oklch(0.9 0.05 250);
    flex-shrink: 0;
  }}
  [data-theme="light"] .profile-card-avatar {{
    background: oklch(0.55 0.15 250 / 0.2);
    color: oklch(0.35 0.12 250);
  }}
  .profile-card-name {{ font-size: 16px; font-weight: 600; }}
  .profile-card-email {{ font-size: 12px; color: oklch(0.5 0 0); margin-top: 2px; }}
</style>
</head>
<body>
  {sprites}

  <nav class="settings-nav" role="navigation" aria-label="Settings categories">
    <div class="nav-header">Settings</div>
    <button class="nav-item active" data-page="appearance">
      <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM48,48H208V96H48ZM48,208V112H208v96Z"/></svg>
      Appearance
    </button>
    <button class="nav-item" data-page="display">
      <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M208,40H48A24,24,0,0,0,24,64V176a24,24,0,0,0,24,24H208a24,24,0,0,0,24-24V64A24,24,0,0,0,208,40Zm8,136a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H208a8,8,0,0,1,8,8Zm-48,48a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h64A8,8,0,0,1,168,224Z"/></svg>
      Display
    </button>
    <button class="nav-item" data-page="sound">
      <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81Z"/></svg>
      Sound
    </button>
    <button class="nav-item" data-page="network">
      <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Z"/></svg>
      Network
    </button>
    <button class="nav-item" data-page="bluetooth">
      <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M196.8,169.6,141.33,128,196.8,86.4a8,8,0,0,0,0-12.8l-64-48A8,8,0,0,0,120,32v80L68.8,73.6a8,8,0,0,0-9.6,12.8L114.67,128,59.2,169.6a8,8,0,1,0,9.6,12.8L120,144v80a8,8,0,0,0,12.8,6.4l64-48a8,8,0,0,0,0-12.8ZM136,48l42.67,32L136,112Zm0,160V144l42.67,32Z"/></svg>
      Bluetooth
    </button>
    <button class="nav-item" data-page="power">
      <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,0,1-47.42-162.4,8,8,0,1,1,8.6,13.48A72,72,0,1,0,200,128a71.56,71.56,0,0,0-33.19-60.55,8,8,0,0,1,8.58-13.51A88,88,0,0,1,128,216Zm8-128V80a8,8,0,0,0-16,0v48a8,8,0,0,0,16,0Z"/></svg>
      Power
    </button>
    <button class="nav-item" data-page="users">
      <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M230.92,212c-15.23-26.33-38.7-45.21-66.09-54.16a72,72,0,1,0-73.66,0C63.78,166.78,40.31,185.66,25.08,212a8,8,0,1,0,13.85,8C55.71,192.33,90.05,176,128,176s72.29,16.33,89.07,44a8,8,0,1,0,13.85-8Z"/></svg>
      Users
    </button>
    <button class="nav-item" data-page="about">
      <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm-8-80V128a8,8,0,0,1,16,0v8a8,8,0,0,1-16,0Zm20-36a12,12,0,1,1-12-12A12,12,0,0,1,140,100Z"/></svg>
      About
    </button>
  </nav>

  <main class="settings-content">
    <!-- Appearance -->
    <div class="settings-page active" id="page-appearance">
      <h1 class="page-title">Appearance</h1>
      <div class="section">
        <div class="section-title">Theme</div>
        <div class="setting-row">{theme_switch}</div>
      </div>
    </div>

    <!-- Display -->
    <div class="settings-page" id="page-display">
      <h1 class="page-title">Display</h1>
      <div class="section">
        <div class="section-title">Brightness</div>
        <div class="setting-row">{display_brightness}</div>
      </div>
      <div class="section">
        <div class="section-title">Night Light</div>
        <div class="setting-row">{display_nightlight}</div>
      </div>
    </div>

    <!-- Sound -->
    <div class="settings-page" id="page-sound">
      <h1 class="page-title">Sound</h1>
      <div class="section">
        <div class="section-title">Output</div>
        <div class="setting-row">{sound_output}</div>
      </div>
      {sep}
      <div class="section">
        <div class="section-title">Input</div>
        <div class="setting-row">{sound_input}</div>
      </div>
    </div>

    <!-- Network -->
    <div class="settings-page" id="page-network">
      <h1 class="page-title">Network</h1>
      <div class="section">
        <div class="section-title">Wi-Fi</div>
        <div class="setting-row">{wifi_switch}</div>
        <div style="margin-top:12px">
          <div class="network-item">
            <svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M144,204a16,16,0,1,1-16-16A16,16,0,0,1,144,204ZM239.61,83.91a176,176,0,0,0-223.22,0,12,12,0,1,0,15.23,18.55,152,152,0,0,1,192.76,0,12,12,0,1,0,15.23-18.55Z"/></svg>
            <span class="network-name">Collet-Home</span>
            <span class="network-status network-connected">Connected</span>
          </div>
          <div class="network-item">
            <svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M144,204a16,16,0,1,1-16-16A16,16,0,0,1,144,204ZM239.61,83.91a176,176,0,0,0-223.22,0,12,12,0,1,0,15.23,18.55,152,152,0,0,1,192.76,0,12,12,0,1,0,15.23-18.55Z"/></svg>
            <span class="network-name">Guest-5G</span>
            <span class="network-status">Saved</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Bluetooth -->
    <div class="settings-page" id="page-bluetooth">
      <h1 class="page-title">Bluetooth</h1>
      <div class="section">
        <div class="setting-row">{bt_switch}</div>
      </div>
    </div>

    <!-- Power -->
    <div class="settings-page" id="page-power">
      <h1 class="page-title">Power</h1>
      <div class="section">
        <div class="section-title">Security</div>
        <div class="setting-row">{autolock_switch}</div>
      </div>
    </div>

    <!-- Users -->
    <div class="settings-page" id="page-users">
      <h1 class="page-title">Users</h1>
      <div class="profile-card">
        <div class="profile-card-avatar">D</div>
        <div>
          <div class="profile-card-name">Dan</div>
          <div class="profile-card-email">dan@collet-os</div>
        </div>
      </div>
    </div>

    <!-- About -->
    <div class="settings-page" id="page-about">
      <h1 class="page-title">About</h1>
      <div class="section">
        <div class="section-title">System</div>
        <div class="info-row"><span class="info-label">OS</span><span class="info-value">Collet OS 1.0</span></div>
        <div class="info-row"><span class="info-label">Kernel</span><span class="info-value">Linux 6.12</span></div>
        <div class="info-row"><span class="info-label">Desktop</span><span class="info-value">COSMIC + Collet Shell</span></div>
        <div class="info-row"><span class="info-label">Architecture</span><span class="info-value">aarch64</span></div>
        <div class="info-row"><span class="info-label">Hostname</span><span class="info-value">collet-os</span></div>
      </div>
    </div>
  </main>

  <script>
    // Page navigation
    document.querySelectorAll('.nav-item').forEach(btn => {{
      btn.addEventListener('click', () => {{
        document.querySelectorAll('.nav-item').forEach(b => {{ b.classList.remove('active'); b.removeAttribute('aria-current'); }});
        document.querySelectorAll('.settings-page').forEach(p => p.classList.remove('active'));
        btn.classList.add('active');
        btn.setAttribute('aria-current', 'page');
        const page = document.getElementById('page-' + btn.dataset.page);
        if (page) page.classList.add('active');
      }});
    }});

    // Slider fill tracking
    document.querySelectorAll('input[data-slider]').forEach(s => {{
      s.addEventListener('input', () => {{
        const pct = (s.value - s.min) / (s.max - s.min) * 100;
        s.style.setProperty('--slider-percent', pct);
        const valEl = s.closest('[part="base"]')?.querySelector('[data-slider-value]');
        if (valEl) valEl.textContent = Math.round(s.value) + '%';
      }});
    }});
  </script>
</body>
</html>"##
    )
}

/// Render lock screen HTML — full-screen overlay with password authentication.
pub fn render_lock_screen() -> String {
    use std::fs;
    use base64::Engine as _;

    SpriteCollector::reset();
    let tokens_css = design_system::generate_tokens_css();
    let sprites = SpriteCollector::take_sprite();

    let bg_data_url = fs::read("assets/bg-dark.jpg")
        .map(|bytes| {
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            format!("data:image/jpeg;base64,{b64}")
        })
        .unwrap_or_default();

    format!(
        r##"<!DOCTYPE html>
<html lang="en" data-theme="dark">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<script src="https://cdn.tailwindcss.com"></script>
<style>{tokens_css}</style>
<style>
  :root {{ --cx-ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1); }}
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  html, body {{ height: 100%; overflow: hidden; }}
  body {{
    font-family: var(--cx-font-sans, 'Geist', 'Inter', system-ui, sans-serif);
    background: url('{bg_data_url}') center/cover no-repeat, oklch(0.08 0 0);
    display: flex;
    align-items: center;
    justify-content: center;
  }}
  .lock-backdrop {{
    position: absolute;
    inset: 0;
    backdrop-filter: blur(40px) saturate(1.4) brightness(0.5);
    -webkit-backdrop-filter: blur(40px) saturate(1.4) brightness(0.5);
    z-index: 0;
  }}
  .lock-clock {{
    position: absolute;
    top: 60px;
    left: 50%;
    transform: translateX(-50%);
    text-align: center;
    z-index: 1;
    color: oklch(0.95 0 0);
  }}
  .lock-time {{
    font-size: 64px;
    font-weight: 200;
    letter-spacing: -2px;
    font-variant-numeric: tabular-nums;
    line-height: 1;
  }}
  .lock-date {{
    font-size: 16px;
    font-weight: 400;
    color: oklch(0.7 0 0);
    margin-top: 8px;
  }}
  .lock-card {{
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    padding: 40px;
    border-radius: 24px;
    background: oklch(0.15 0 0 / 0.6);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid oklch(1.0 0 0 / 0.06);
    min-width: 320px;
  }}
  .lock-avatar {{
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: oklch(0.45 0.15 250 / 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 32px;
    color: oklch(0.9 0.05 250);
  }}
  .lock-name {{
    font-size: 18px;
    font-weight: 600;
    color: oklch(0.95 0 0);
  }}
  .lock-input {{
    width: 260px;
    padding: 12px 16px;
    border-radius: 12px;
    border: 1px solid oklch(1.0 0 0 / 0.1);
    background: oklch(1.0 0 0 / 0.06);
    color: oklch(0.95 0 0);
    font-family: inherit;
    font-size: 15px;
    outline: none;
    text-align: center;
    letter-spacing: 4px;
    transition: border-color var(--cx-duration-fast, 100ms) ease-out, transform var(--cx-duration-smooth, 300ms) var(--cx-ease-spring);
  }}
  .lock-input:focus {{
    border-color: oklch(0.6 0.15 250);
  }}
  .lock-input.shake {{
    animation: lock-shake 400ms ease-out;
  }}
  @keyframes lock-shake {{
    0%, 100% {{ transform: translateX(0); }}
    20% {{ transform: translateX(-12px); }}
    40% {{ transform: translateX(10px); }}
    60% {{ transform: translateX(-6px); }}
    80% {{ transform: translateX(4px); }}
  }}
  .lock-status {{
    font-size: 13px;
    color: oklch(0.65 0.12 25);
    min-height: 20px;
  }}
  .lock-battery {{
    position: absolute;
    bottom: 20px;
    right: 24px;
    z-index: 1;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: oklch(0.55 0 0);
  }}
</style>
</head>
<body role="dialog" aria-label="Lock screen" aria-modal="true">
  {sprites}
  <div class="lock-backdrop"></div>
  <div class="lock-clock">
    <div class="lock-time" id="lockTime">00:00</div>
    <div class="lock-date" id="lockDate">Monday, 1 January</div>
  </div>
  <div class="lock-card">
    <div class="lock-avatar">D</div>
    <div class="lock-name">Dan</div>
    <input type="password" class="lock-input" id="lockPass" autocomplete="current-password" aria-label="Password" placeholder="Password" autofocus>
    <div class="lock-status" id="lockStatus" aria-live="assertive"></div>
  </div>
  <div class="lock-battery">
    <svg width="14" height="14" viewBox="0 0 256 256" fill="currentColor"><path d="M200,56H32A24,24,0,0,0,8,80v96a24,24,0,0,0,24,24H200a24,24,0,0,0,24-24V80A24,24,0,0,0,200,56Zm8,120a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V80a8,8,0,0,1,8-8H200a8,8,0,0,1,8,8ZM144,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm-40,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0ZM64,96v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Zm192,0v64a8,8,0,0,1-16,0V96a8,8,0,0,1,16,0Z"/></svg>
    <span>72%</span>
  </div>
  <script>
    // Clock
    function updateClock() {{
      const n = new Date();
      const h = String(n.getHours()).padStart(2, '0');
      const m = String(n.getMinutes()).padStart(2, '0');
      document.getElementById('lockTime').textContent = h + ':' + m;
      const days = ['Sunday','Monday','Tuesday','Wednesday','Thursday','Friday','Saturday'];
      const months = ['January','February','March','April','May','June','July','August','September','October','November','December'];
      document.getElementById('lockDate').textContent = days[n.getDay()] + ', ' + n.getDate() + ' ' + months[n.getMonth()];
    }}
    updateClock();
    setInterval(updateClock, 1000);

    // Password authentication
    const passInput = document.getElementById('lockPass');
    const status = document.getElementById('lockStatus');

    function attemptUnlock() {{
      const pw = passInput.value;
      if (!pw) return;
      // SECURITY: Stub — real implementation requires PAM integration
      if (window.ipc) {{
        window.ipc.postMessage(JSON.stringify({{ action: 'unlock', payload: {{ password: pw }} }}));
      }}
      // For preview: accept any non-empty password
      passInput.value = '';
      status.textContent = '';
    }}

    function showError(msg) {{
      status.textContent = msg;
      passInput.classList.add('shake');
      passInput.value = '';
      passInput.focus();
      setTimeout(() => passInput.classList.remove('shake'), 450);
    }}

    passInput.addEventListener('keydown', (e) => {{
      if (e.key === 'Enter') attemptUnlock();
    }});
  </script>
</body>
</html>"##
    )
}

/// Render dock surface HTML (Linux layer-shell).
pub fn render_dock() -> String {
    SpriteCollector::reset();
    let dock_buttons = render_dock_buttons();
    let sprites = SpriteCollector::take_sprite();
    let tokens_css = design_system::generate_tokens_css();

    format!(
        r##"<!DOCTYPE html>
<html lang="en" data-theme="dark">
<head>
<meta charset="utf-8">
<script src="https://cdn.tailwindcss.com"></script>
<style>{tokens_css}</style>
<style>
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{
    background: transparent;
    font-family: var(--cx-font-sans, 'Geist', system-ui, sans-serif);
    display: flex; justify-content: center; align-items: flex-end;
    height: 100vh; padding-bottom: 8px;
  }}
  .dock {{
    background: oklch(0.145 0.000 0.0 / 0.82);
    backdrop-filter: blur(24px); -webkit-backdrop-filter: blur(24px);
    border: 1px solid oklch(1.0 0.0 0.0 / 0.06);
    border-radius: 16px; padding: 8px 14px;
    display: flex; align-items: center; gap: 6px;
    box-shadow: 0 8px 40px oklch(0.0 0.0 0.0 / 0.35);
  }}
  .sep {{ width: 1px; height: 28px; background: oklch(1.0 0.0 0.0 / 0.06); margin: 0 2px; }}
  .dock button {{ color: oklch(0.85 0 0) !important; }}
  .dock button:hover {{ color: oklch(1.0 0 0) !important; background: oklch(1.0 0 0 / 0.08) !important; transform: scale(1.12); transition: all 180ms cubic-bezier(0.34,1.56,0.64,1); }}
  .dock button:active {{ transform: scale(0.93); transition-duration: 60ms; }}
</style>
</head>
<body>{sprites}
  <nav class="dock">{dock_buttons}</nav>
</body></html>"##
    )
}

/// Render control bar surface HTML (Linux layer-shell).
pub fn render_control_bar() -> String {
    SpriteCollector::reset();
    let bar_buttons = render_control_bar_buttons(72, true);
    let sprites = SpriteCollector::take_sprite();
    let tokens_css = design_system::generate_tokens_css();

    format!(
        r##"<!DOCTYPE html>
<html lang="en" data-theme="dark">
<head>
<meta charset="utf-8">
<script src="https://cdn.tailwindcss.com"></script>
<style>{tokens_css}</style>
<style>
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{ background: transparent; font-family: var(--cx-font-sans, 'Geist', system-ui, sans-serif); display: flex; justify-content: flex-end; padding: 8px; }}
  .bar {{ background: oklch(0.185 0 0 / 0.78); backdrop-filter: blur(24px); border: 1px solid oklch(1.0 0 0 / 0.06); border-radius: 99px; padding: 3px 10px; display: flex; align-items: center; gap: 6px; box-shadow: 0 4px 20px oklch(0 0 0 / 0.25); color: oklch(0.88 0 0); font-size: 12px; }}
  .bar-sep {{ width: 1px; height: 14px; background: oklch(1.0 0 0 / 0.06); }}
  .clock {{ font-variant-numeric: tabular-nums; font-weight: 500; }}
  .bar button {{ color: oklch(0.85 0 0) !important; }}
  .bar button:hover {{ color: oklch(1.0 0 0) !important; }}
</style>
</head>
<body>{sprites}<div class="bar">{bar_buttons}</div>
<script>function u(){{const n=new Date();document.getElementById('c').textContent=String(n.getHours()).padStart(2,'0')+':'+String(n.getMinutes()).padStart(2,'0');}}u();setInterval(u,1000);</script>
</body></html>"##
    )
}
