//! Shell surface renderer — produces HTML from Collet components + custom layout.
//!
//! Two modes:
//! - Linux: render_dock() and render_control_bar() as separate surface HTML
//! - macOS preview: render_preview() combines everything in one desktop simulation

use components::button::{Button, ButtonVariant, ButtonShape};
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
    // Custom Files/Explorer button with bespoke SVG
    let files_btn = format!(
        r#"<button type="button" class="dock-btn" aria-label="Files" data-ipc="launch" data-app-id="files">{}</button>"#,
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

fn render_control_bar_buttons() -> String {
    let bell_btn = Button::icon_only(Icon::Bell, "Notifications")
        .variant(ButtonVariant::Ghost)
        .size(ComponentSize::Sm)
        .render();

    let settings_btn = Button::icon_only(Icon::Settings, "Settings")
        .variant(ButtonVariant::Ghost)
        .size(ComponentSize::Sm)
        .render();

    let power_btn = Button::icon_only(Icon::LogOut, "Power")
        .variant(ButtonVariant::Ghost)
        .size(ComponentSize::Sm)
        .render();

    format!(
        r#"{bell_btn}
        <span class="bar-sep"></span>
        <span class="clock" id="c">00:00</span>
        <span class="bar-sep"></span>
        {settings_btn}
        <span class="bar-sep"></span>
        {power_btn}"#
    )
}

/// macOS preview — full desktop simulation in one window.
/// Dock at bottom, control bar at top-right, desktop area in the middle.
pub fn render_preview() -> String {
    use std::fs;
    use base64::Engine as _;

    SpriteCollector::reset();

    let dock_buttons = render_dock_buttons();
    let control_bar_buttons = render_control_bar_buttons();
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
    padding: 6px 16px;
    display: flex;
    align-items: center;
    gap: 10px;
    box-shadow: 0 4px 20px oklch(0.0 0.0 0.0 / 0.25);
    color: oklch(0.880 0.000 0.0);
    font-size: 12px;
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
    width: 320px;
    background: oklch(0.185 0.000 0.0 / 0.88);
    backdrop-filter: blur(32px) saturate(1.3);
    -webkit-backdrop-filter: blur(32px) saturate(1.3);
    border: 1px solid oklch(1.0 0.0 0.0 / 0.06);
    border-radius: 20px;
    padding: 16px;
    box-shadow: 0 8px 40px oklch(0.0 0.0 0.0 / 0.35);
    color: oklch(0.880 0.000 0.0);
    font-size: 13px;
    opacity: 0;
    pointer-events: none;
    transform: translateY(-12px) scale(0.96);
    transform-origin: top right;
    transition: opacity 200ms ease,
                transform 400ms var(--cx-ease-spring),
                backdrop-filter 300ms ease;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }}

  .control-island.open {{
    opacity: 1;
    pointer-events: auto;
    transform: translateY(0) scale(1);
  }}

  /* Quick settings grid */
  .qs-grid {{
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }}

  .qs-tile {{
    background: oklch(1.0 0.0 0.0 / 0.06);
    border: 1px solid oklch(1.0 0.0 0.0 / 0.04);
    border-radius: 12px;
    padding: 10px 12px;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    transition: all 150ms var(--cx-ease-spring);
    font-size: 12px;
    color: oklch(0.78 0.0 0.0);
  }}

  .qs-tile:hover {{
    background: oklch(1.0 0.0 0.0 / 0.10);
    color: oklch(0.95 0.0 0.0);
  }}

  .qs-tile.active {{
    background: oklch(1.0 0.0 0.0 / 0.12);
    color: oklch(0.95 0.0 0.0);
  }}

  .qs-icon {{
    font-size: 16px;
    flex-shrink: 0;
  }}

  /* Island divider */
  .island-divider {{
    height: 1px;
    background: oklch(1.0 0.0 0.0 / 0.06);
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
    grid-template-columns: repeat(7, 1fr);
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
    cursor: default;
    color: oklch(0.65 0.0 0.0);
  }}

  .cal-day.today {{
    background: oklch(1.0 0.0 0.0 / 0.12);
    color: oklch(0.95 0.0 0.0);
    font-weight: 600;
  }}

  .cal-day.other-month {{
    color: oklch(0.3 0.0 0.0);
  }}

  /* Profile row */
  .profile-row {{
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px;
    border-radius: 12px;
    cursor: pointer;
    transition: background 150ms ease;
  }}

  .profile-row:hover {{
    background: oklch(1.0 0.0 0.0 / 0.06);
  }}

  .profile-avatar {{
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: oklch(1.0 0.0 0.0 / 0.10);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
  }}

  .profile-name {{
    font-size: 13px;
    font-weight: 500;
  }}

  .profile-sub {{
    font-size: 11px;
    color: oklch(0.5 0.0 0.0);
  }}

  /* Log out */
  .logout-btn {{
    width: 100%;
    background: none;
    border: 1px solid oklch(1.0 0.0 0.0 / 0.06);
    border-radius: 10px;
    padding: 8px;
    color: oklch(0.6 0.0 0.0);
    font-family: inherit;
    font-size: 12px;
    cursor: pointer;
    transition: all 150ms ease;
  }}

  .logout-btn:hover {{
    background: oklch(0.58 0.095 25.0 / 0.15);
    color: oklch(0.75 0.08 25.0);
    border-color: oklch(0.58 0.095 25.0 / 0.2);
  }}

  .clock {{
    font-variant-numeric: tabular-nums;
    font-weight: 500;
    letter-spacing: 0.3px;
    font-size: 12px;
  }}

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

  /* ── Light mode overrides ────────────────────────── */
  [data-theme="light"] .dock button,
  [data-theme="light"] .dock-btn,
  [data-theme="light"] .control-bar button {{
    color: oklch(0.25 0.003 90) !important;
  }}

  [data-theme="light"] .dock button:hover,
  [data-theme="light"] .dock-btn:hover,
  [data-theme="light"] .control-bar button:hover {{
    color: oklch(0.15 0.003 90) !important;
    background: oklch(0.0 0.0 0.0 / 0.06) !important;
  }}

  [data-theme="light"] .sep,
  [data-theme="light"] .bar-sep {{
    background: oklch(0.0 0.0 0.0 / 0.08);
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

  [data-theme="light"] .qs-tile {{
    background: oklch(0.0 0.0 0.0 / 0.04);
    border-color: oklch(0.0 0.0 0.0 / 0.04);
    color: oklch(0.35 0.003 90);
  }}

  [data-theme="light"] .qs-tile:hover,
  [data-theme="light"] .qs-tile.active {{
    background: oklch(0.0 0.0 0.0 / 0.08);
    color: oklch(0.2 0.003 90);
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

  [data-theme="light"] .profile-name {{ color: oklch(0.2 0.003 90); }}
  [data-theme="light"] .logout-btn {{ color: oklch(0.4 0.003 90); border-color: oklch(0.0 0.0 0.0 / 0.08); }}
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

  .dock button,
  .control-bar button {{
    color: oklch(0.85 0.000 0.0) !important;
    opacity: 1 !important;
  }}

  .dock button:hover,
  .control-bar button:hover {{
    color: oklch(1.0 0.000 0.0) !important;
    background: oklch(1.0 0.0 0.0 / 0.08) !important;
    transform: scale(1.12);
    transition: all 180ms var(--cx-ease-spring);
  }}

  .dock button:active,
  .control-bar button:active {{
    transform: scale(0.93);
    transition-duration: 60ms;
  }}

  .dock button svg,
  .dock-btn svg,
  .control-bar button svg {{
    opacity: 0.9;
  }}

  .dock button:hover svg,
  .dock-btn:hover svg,
  .control-bar button:hover svg {{
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
</style>
</head>
<body>
  {sprites}
  <div class="desktop">
    <!-- Control Bar (top-right pill) + Island -->
    <div class="control-bar-container">
      <div class="control-bar" id="controlBar" role="status" aria-label="System controls">
        {control_bar_buttons}
      </div>

      <div class="control-island" id="controlIsland">
        <!-- Quick Settings (Phosphor icons) -->
        <div class="qs-grid">
          <div class="qs-tile active">
            <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M140,204a12,12,0,1,1-12-12A12,12,0,0,1,140,204ZM237.08,87A172,172,0,0,0,18.92,87,8,8,0,0,0,29.08,99.37a156,156,0,0,1,197.84,0A8,8,0,0,0,237.08,87ZM205,122.77a124,124,0,0,0-153.94,0A8,8,0,0,0,61,135.31a108,108,0,0,1,134.06,0,8,8,0,0,0,11.24-1.3A8,8,0,0,0,205,122.77Zm-32.26,35.76a76.05,76.05,0,0,0-89.42,0,8,8,0,0,0,9.42,12.94,60,60,0,0,1,70.58,0,8,8,0,1,0,9.42-12.94Z"/></svg>
            Wi-Fi
          </div>
          <div class="qs-tile">
            <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M196.8,169.6,141.33,128,196.8,86.4a8,8,0,0,0,0-12.8l-64-48A8,8,0,0,0,120,32v80L68.8,73.6a8,8,0,0,0-9.6,12.8L114.67,128,59.2,169.6a8,8,0,1,0,9.6,12.8L120,144v80a8,8,0,0,0,12.8,6.4l64-48a8,8,0,0,0,0-12.8ZM136,48l42.67,32L136,112Zm0,160V144l42.67,32Z"/></svg>
            Bluetooth
          </div>
          <div class="qs-tile">
            <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor"><path d="M155.51,24.81a8,8,0,0,0-8.42.88L77.25,80H32A16,16,0,0,0,16,96v64a16,16,0,0,0,16,16H77.25l69.84,54.31A8,8,0,0,0,160,224V32A8,8,0,0,0,155.51,24.81ZM32,96H72v64H32ZM144,207.64,88,164.09V91.91l56-43.55Zm54-106.08a40,40,0,0,1,0,52.88,8,8,0,0,1-12-10.58,24,24,0,0,0,0-31.72,8,8,0,0,1,12-10.58ZM248,128a79.9,79.9,0,0,1-20.37,53.34,8,8,0,0,1-11.92-10.67,64,64,0,0,0,0-85.33,8,8,0,1,1,11.92-10.67A79.83,79.83,0,0,1,248,128Z"/></svg>
            Sound
          </div>
          <div class="qs-tile active" id="themeToggle">
            <svg width="16" height="16" viewBox="0 0 256 256" fill="currentColor" id="themeIcon"><path d="M233.54,142.23a8,8,0,0,0-8-2,88.08,88.08,0,0,1-109.8-109.8,8,8,0,0,0-10-10,104.84,104.84,0,0,0-52.91,37A104,104,0,0,0,136,224a103.09,103.09,0,0,0,62.52-20.88,104.84,104.84,0,0,0,37-52.91A8,8,0,0,0,233.54,142.23ZM188.9,190.34A88,88,0,0,1,65.66,67.11a89,89,0,0,1,31.4-26A106,106,0,0,0,96,56,104.11,104.11,0,0,0,200,160a106,106,0,0,0,14.92-1.06A89,89,0,0,1,188.9,190.34Z"/></svg>
            <span id="themeLabel">Dark</span>
          </div>
        </div>

        <div class="island-divider"></div>

        <!-- Calendar -->
        <div>
          <div class="cal-header" id="calHeader"></div>
          <div class="cal-grid" id="calGrid"></div>
        </div>

        <div class="island-divider"></div>

        <!-- Profile -->
        <div class="profile-row" data-ipc="settings">
          <div class="profile-avatar">👤</div>
          <div>
            <div class="profile-name">User</div>
            <div class="profile-sub">Account settings</div>
          </div>
        </div>

        <!-- Log out -->
        <button class="logout-btn" data-ipc="logout">Log out</button>
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
          <textarea class="search-input" id="searchInput" rows="1"
                 placeholder="Search apps, files, or ask anything..."
                 autocomplete="off" spellcheck="false"></textarea>
          <div class="search-results" id="searchResults"></div>
        </div>
      </nav>
    </div>
  </div>

  <script>
    // Clock
    function u(){{const n=new Date();document.getElementById('c').textContent=
    String(n.getHours()).padStart(2,'0')+':'+String(n.getMinutes()).padStart(2,'0');}}
    u();setInterval(u,1000);

    // Theme toggle — dark/light mode
    let isDark = true;
    const themeToggle = document.querySelector('#themeToggle');
    const themeLabel = document.querySelector('#themeLabel');
    const desktop = document.querySelector('.desktop');
    const htmlEl = document.documentElement;

    themeToggle.addEventListener('click', (e) => {{
      e.stopPropagation();
      isDark = !isDark;
      htmlEl.setAttribute('data-theme', isDark ? 'dark' : 'light');
      themeLabel.textContent = isDark ? 'Dark' : 'Light';
      themeToggle.classList.toggle('active', isDark);

      // Swap background image
      if (isDark) {{
        desktop.style.background = "url('{bg_data_url}') center/cover no-repeat, oklch(0.13 0 0)";
      }} else {{
        desktop.style.background = "url('{bg_light_url}') center/cover no-repeat, oklch(0.95 0.005 90)";
      }}

      // Update dock and bar colors
      const dock = document.querySelector('.dock');
      const bar = document.querySelector('.control-bar');
      const island = document.querySelector('.control-island');
      if (isDark) {{
        dock.style.background = 'oklch(0.145 0 0 / 0.88)';
        bar.style.background = 'oklch(0.185 0 0 / 0.78)';
        island.style.background = 'oklch(0.185 0 0 / 0.88)';
      }} else {{
        dock.style.background = 'oklch(1.0 0 0 / 0.75)';
        bar.style.background = 'oklch(1.0 0 0 / 0.68)';
        island.style.background = 'oklch(1.0 0 0 / 0.82)';
      }}
    }});

    // Control Island toggle
    const controlBar = document.querySelector('#controlBar');
    const island = document.querySelector('#controlIsland');
    let islandOpen = false;

    function toggleIsland() {{
      islandOpen = !islandOpen;
      island.classList.toggle('open', islandOpen);
    }}

    controlBar.addEventListener('click', (e) => {{
      e.stopPropagation();
      toggleIsland();
    }});

    document.addEventListener('click', (e) => {{
      if (islandOpen && !e.target.closest('.control-island') && !e.target.closest('.control-bar')) {{
        toggleIsland();
      }}
    }});

    // Calendar generation
    (function buildCal() {{
      const now = new Date();
      const year = now.getFullYear();
      const month = now.getMonth();
      const today = now.getDate();
      const monthNames = ['January','February','March','April','May','June','July','August','September','October','November','December'];
      document.querySelector('#calHeader').textContent = monthNames[month] + ' ' + year;

      const firstDay = new Date(year, month, 1).getDay();
      const daysInMonth = new Date(year, month + 1, 0).getDate();
      const daysInPrev = new Date(year, month, 0).getDate();
      const dayNames = ['Su','Mo','Tu','We','Th','Fr','Sa'];
      let html = dayNames.map(d => '<span class="cal-day-name">' + d + '</span>').join('');

      const startDay = (firstDay === 0) ? 6 : firstDay - 1;
      for (let i = startDay - 1; i >= 0; i--) {{
        html += '<span class="cal-day other-month">' + (daysInPrev - i) + '</span>';
      }}
      for (let d = 1; d <= daysInMonth; d++) {{
        const cls = d === today ? 'cal-day today' : 'cal-day';
        html += '<span class="' + cls + '">' + d + '</span>';
      }}
      const totalCells = startDay + daysInMonth;
      const remaining = (7 - (totalCells % 7)) % 7;
      for (let i = 1; i <= remaining; i++) {{
        html += '<span class="cal-day other-month">' + i + '</span>';
      }}
      document.querySelector('#calGrid').innerHTML = html;
    }})();

    // Dock expansion — search surface grows from within the dock
    const dock = document.querySelector('#dock');
    const searchInput = document.querySelector('#searchInput');
    let expanded = false;

    // Auto-expand textarea
    searchInput.addEventListener('input', () => {{
      searchInput.style.height = 'auto';
      searchInput.style.height = Math.min(searchInput.scrollHeight, 120) + 'px';
    }});

    // Enter = submit, Shift+Enter = new line
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
      if (e.key === 'Escape' && expanded) toggleDock();
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
    let bar_buttons = render_control_bar_buttons();
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
