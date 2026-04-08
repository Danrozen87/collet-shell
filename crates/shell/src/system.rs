//! System state queries — battery, Wi-Fi, Bluetooth.
//!
//! On Linux: queries UPower, NetworkManager, BlueZ via D-Bus.
//! On macOS: returns stub data for preview development.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub battery: BatteryState,
    pub wifi: WifiState,
    pub bluetooth: BluetoothState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryState {
    pub percentage: u8,
    pub charging: bool,
    pub level: BatteryLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BatteryLevel {
    Empty,   // 0-10%
    Low,     // 11-25%
    Medium,  // 26-50%
    High,    // 51-75%
    Full,    // 76-100%
}

impl BatteryLevel {
    pub fn from_percentage(pct: u8) -> Self {
        match pct {
            0..=10 => Self::Empty,
            11..=25 => Self::Low,
            26..=50 => Self::Medium,
            51..=75 => Self::High,
            _ => Self::Full,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiState {
    pub connected: bool,
    pub ssid: Option<String>,
    pub signal: SignalLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SignalLevel {
    None,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothState {
    pub enabled: bool,
    pub connected_devices: Vec<String>,
}

/// Returns current system state.
/// TODO: Linux — query UPower, NetworkManager, BlueZ via D-Bus (zbus).
/// For now returns stub data suitable for preview development.
pub fn query() -> SystemState {
    SystemState {
        battery: BatteryState {
            percentage: 72,
            charging: false,
            level: BatteryLevel::High,
        },
        wifi: WifiState {
            connected: true,
            ssid: Some("Collet-Home".into()),
            signal: SignalLevel::High,
        },
        bluetooth: BluetoothState {
            enabled: false,
            connected_devices: vec![],
        },
    }
}
