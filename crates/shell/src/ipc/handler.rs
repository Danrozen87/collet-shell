//! IPC message handler — dispatches web frontend requests to system operations.

use serde::{Deserialize, Serialize};

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
        "system_info" => system_info(),
        "power" => power_action(&request.payload),
        _ => Response {
            ok: false,
            data: serde_json::json!({"error": "unknown action"}),
        },
    };

    serde_json::to_string(&response).unwrap_or_default()
}

fn launch_app(payload: &serde_json::Value) -> Response {
    let app_id = payload["app_id"].as_str().unwrap_or("");
    // TODO: use freedesktop-desktop-entry to find and launch
    eprintln!("[collet-shell] Launch: {app_id}");
    Response {
        ok: true,
        data: serde_json::json!({"launched": app_id}),
    }
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
