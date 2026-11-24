//! One file per page of the navigation pane, plus the labels and date helpers they share.
pub mod history;
pub mod home;
pub mod settings;

use crate::model::{Client, Profile};
use chrono::{Datelike, Local, TimeZone, Timelike};

pub fn profile_title(p: Profile) -> &'static str {
    match p {
        Profile::Global => "Global",
        Profile::Cn => "China (燕云十六声)",
    }
}

pub fn client_title(c: Client) -> &'static str {
    match c {
        Client::Steam => "Steam",
        Client::Launcher => t!("pages.official_launcher"),
    }
}

fn local(at: u64) -> Option<chrono::DateTime<Local>> {
    Local.timestamp_opt(at as i64, 0).single()
}

/// `HH:MM` of a Unix timestamp, local time.
pub fn clock(at: u64) -> String {
    local(at).map(|d| format!("{:02}:{:02}", d.hour(), d.minute())).unwrap_or_default()
}

/// `13 Nov 2026` of a Unix timestamp, local time.
pub fn day(at: u64) -> String {
    let months = [
        t!("date.month.01"),
        t!("date.month.02"),
        t!("date.month.03"),
        t!("date.month.04"),
        t!("date.month.05"),
        t!("date.month.06"),
        t!("date.month.07"),
        t!("date.month.08"),
        t!("date.month.09"),
        t!("date.month.10"),
        t!("date.month.11"),
        t!("date.month.12"),
    ];
    local(at)
        .map(|d| format!("{} {} {}", d.day(), months[d.month0() as usize % 12], d.year()))
        .unwrap_or_default()
}
