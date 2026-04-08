//! IPC message handler — dispatches web frontend requests to system operations.
//!
//! ## Compositor key bindings (collet-cosmic-comp, separate repo)
//!
//! The following shortcuts should be bound in the compositor and trigger
//! shell IPC actions via `webview.evaluate_script()`:
//!
//! | Key               | Shell IPC action  | JS function       |
//! |-------------------|-------------------|--------------------|
//! | Super             | `toggle_dock`     | `toggleDock()`     |
//! | Super+Escape      | `toggle_island`   | `toggleIsland()`   |
//! | XF86AudioRaise    | `show_osd`        | `showOSD('volume', pct)` |
//! | XF86AudioLower    | `show_osd`        | `showOSD('volume', pct)` |
//! | XF86MonBrightUp   | `show_osd`        | `showOSD('brightness', pct)` |
//! | XF86MonBrightDown | `show_osd`        | `showOSD('brightness', pct)` |
//! | Super+L           | `lock`            | lock screen surface |

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// An IPC message from the web frontend.
#[derive(Debug, Deserialize)]
pub struct Request {
    pub action: String,
    pub payload: serde_json::Value,
}

/// An IPC response back to the web frontend.
#[derive(Debug, Serialize)]
pub struct Response {
    pub ok: bool,
    pub data: serde_json::Value,
}

/// Handle an IPC request from the web frontend.
/// This is the bridge between the UI and Linux.
pub fn handle(body: &str) -> String {
    let request: Request = match serde_json::from_str(body) {
        Ok(r) => r,
        Err(e) => {
            return serde_json::to_string(&Response {
                ok: false,
                data: serde_json::json!({"error": e.to_string()}),
            })
            .unwrap_or_default();
        }
    };

    let response = match request.action.as_str() {
        "launch_app" => launch_app(&request.payload),
        "search_apps" => search_apps(&request.payload),
        "system_info" => system_info(),
        "power" => power_action(&request.payload),
        "lock" => lock_session(),
        "unlock" => unlock_attempt(&request.payload),
        "system_state" => system_state(),
        "toggle_island" => toggle_island(),
        "toggle_dock" => toggle_dock(),
        "show_osd" => show_osd(&request.payload),
        _ => Response {
            ok: false,
            data: serde_json::json!({"error": "unknown action"}),
        },
    };

    serde_json::to_string(&response).unwrap_or_default()
}

fn launch_app(payload: &serde_json::Value) -> Response {
    let app_id = payload["app_id"].as_str().unwrap_or("");
    eprintln!("[collet-shell] Launch: {app_id}");

    // Try to find the .desktop file and extract Exec
    if let Some(exec) = find_desktop_exec(app_id) {
        // Strip field codes (%u, %U, %f, %F, etc.) from Exec line
        let cmd = exec.split_whitespace()
            .filter(|s| !s.starts_with('%'))
            .collect::<Vec<_>>()
            .join(" ");
        eprintln!("[collet-shell] Exec: {cmd}");
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            let _ = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .spawn();
        }
        Response {
            ok: true,
            data: serde_json::json!({"launched": app_id, "exec": cmd}),
        }
    } else {
        Response {
            ok: true,
            data: serde_json::json!({"launched": app_id, "exec": null}),
        }
    }
}

/// Search installed applications by query string.
/// Returns matching app names, descriptions, and IDs from .desktop files.
fn search_apps(payload: &serde_json::Value) -> Response {
    let query = payload["query"].as_str().unwrap_or("").to_lowercase();
    if query.is_empty() {
        return Response {
            ok: true,
            data: serde_json::json!({"results": []}),
        };
    }

    let apps = discover_apps();
    let results: Vec<_> = apps
        .iter()
        .filter(|a| {
            a.name.to_lowercase().contains(&query)
                || a.description.to_lowercase().contains(&query)
        })
        .take(8)
        .map(|a| {
            serde_json::json!({
                "id": a.id,
                "name": a.name,
                "description": a.description,
                "icon": a.icon,
            })
        })
        .collect();

    Response {
        ok: true,
        data: serde_json::json!({"results": results}),
    }
}

struct AppEntry {
    id: String,
    name: String,
    description: String,
    icon: String,
}

/// Discover installed applications from .desktop files.
fn discover_apps() -> Vec<AppEntry> {
    let mut apps = Vec::new();
    let dirs = desktop_dirs();

    for dir in &dirs {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                continue;
            }
            if let Some(app) = parse_desktop_file(&path) {
                apps.push(app);
            }
        }
    }

    // Deduplicate by name
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps.dedup_by(|a, b| a.name == b.name);
    apps
}

fn desktop_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
    ];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(format!("{home}/.local/share/applications")));
    }
    if let Ok(xdg) = std::env::var("XDG_DATA_DIRS") {
        for dir in xdg.split(':') {
            dirs.push(PathBuf::from(format!("{dir}/applications")));
        }
    }
    // Flatpak exports
    dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
    dirs
}

fn parse_desktop_file(path: &PathBuf) -> Option<AppEntry> {
    let content = std::fs::read_to_string(path).ok()?;

    // Skip NoDisplay and Hidden entries
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "NoDisplay=true" || trimmed == "Hidden=true" {
            return None;
        }
    }

    let mut name = String::new();
    let mut comment = String::new();
    let mut icon = String::new();
    let mut exec = String::new();
    let mut in_desktop_entry = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[Desktop Entry]" {
            in_desktop_entry = true;
            continue;
        }
        if trimmed.starts_with('[') && trimmed != "[Desktop Entry]" {
            if in_desktop_entry {
                break; // Only parse the [Desktop Entry] section
            }
            continue;
        }
        if !in_desktop_entry {
            continue;
        }

        if let Some(val) = trimmed.strip_prefix("Name=") {
            if name.is_empty() {
                name = val.to_string();
            }
        } else if let Some(val) = trimmed.strip_prefix("Comment=") {
            if comment.is_empty() {
                comment = val.to_string();
            }
        } else if let Some(val) = trimmed.strip_prefix("Icon=") {
            if icon.is_empty() {
                icon = val.to_string();
            }
        } else if let Some(val) = trimmed.strip_prefix("Exec=") {
            if exec.is_empty() {
                exec = val.to_string();
            }
        }
    }

    if name.is_empty() {
        return None;
    }

    let id = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    Some(AppEntry {
        id,
        name,
        description: comment,
        icon,
    })
}

fn find_desktop_exec(app_id: &str) -> Option<String> {
    let apps = discover_apps();
    apps.into_iter()
        .find(|a| a.id == app_id || a.name.to_lowercase().replace(' ', "-") == app_id)
        .and_then(|a| {
            let dirs = desktop_dirs();
            for dir in &dirs {
                let path = dir.join(format!("{}.desktop", a.id));
                if let Ok(content) = std::fs::read_to_string(&path) {
                    for line in content.lines() {
                        if let Some(val) = line.trim().strip_prefix("Exec=") {
                            return Some(val.to_string());
                        }
                    }
                }
            }
            None
        })
}

fn system_info() -> Response {
    // TODO: read battery, wifi, disk, memory
    Response {
        ok: true,
        data: serde_json::json!({
            "hostname": "collet",
            "uptime": "placeholder",
        }),
    }
}

fn power_action(payload: &serde_json::Value) -> Response {
    let action = payload["action"].as_str().unwrap_or("");
    eprintln!("[collet-shell] Power: {action}");
    // TODO: call systemctl poweroff/reboot/suspend
    Response {
        ok: true,
        data: serde_json::json!({"action": action}),
    }
}

fn lock_session() -> Response {
    eprintln!("[collet-shell] Lock session");
    // TODO: trigger ext-session-lock-v1 protocol via compositor
    Response {
        ok: true,
        data: serde_json::json!({"locked": true}),
    }
}

fn system_state() -> Response {
    // TODO: query real system state via D-Bus
    Response {
        ok: true,
        data: serde_json::json!({
            "battery": { "percentage": 72, "charging": false, "level": "High" },
            "wifi": { "connected": true, "ssid": "Collet-Home", "signal": "High" },
            "bluetooth": { "enabled": false, "connected_devices": [] }
        }),
    }
}

fn unlock_attempt(payload: &serde_json::Value) -> Response {
    let _password = payload["password"].as_str().unwrap_or("");
    // SECURITY: Stub — real implementation requires PAM integration.
    // Never log the password.
    eprintln!("[collet-shell] Unlock attempt");
    Response {
        ok: true,
        data: serde_json::json!({"unlocked": true}),
    }
}

fn toggle_island() -> Response {
    eprintln!("[collet-shell] Toggle island");
    Response {
        ok: true,
        data: serde_json::json!({"toggled": "island"}),
    }
}

fn toggle_dock() -> Response {
    eprintln!("[collet-shell] Toggle dock");
    Response {
        ok: true,
        data: serde_json::json!({"toggled": "dock"}),
    }
}

fn show_osd(payload: &serde_json::Value) -> Response {
    let osd_type = payload["type"].as_str().unwrap_or("volume");
    let pct = payload["pct"].as_f64().unwrap_or(50.0);
    eprintln!("[collet-shell] OSD: {osd_type} {pct}%");
    Response {
        ok: true,
        data: serde_json::json!({"type": osd_type, "pct": pct}),
    }
}
