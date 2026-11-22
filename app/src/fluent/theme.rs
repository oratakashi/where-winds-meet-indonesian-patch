//! Windows 11 / WinUI 3 (Fluent) palette — the same tokens the Tailwind theme of v0.3 used.
//!
//! - accent = the user's Windows accent colour (`AccentPalette`), like WinUI's
//!   `AccentFillColorDefault` (Dark1 on light theme, Light2 on dark theme)
//! - with Mica (Windows 11 22H2+) the base layer is transparent and the cards translucent, so
//!   the wallpaper tint shows through; elsewhere (Windows 10) everything is opaque.
use crate::shell::system;
use gpui::{App, Global, Hsla, SharedString, WindowAppearance, rgb, rgba};

#[derive(Clone)]
pub struct Theme {
    pub dark: bool,
    pub mica: bool,
    pub font: SharedString,
    pub display_font: SharedString,
    pub icon_font: SharedString,

    pub background: Hsla,
    pub foreground: Hsla,
    pub muted_fg: Hsla,
    pub card: Hsla,
    pub border: Hsla,

    /// Standard (non-accent) buttons.
    pub control: Hsla,
    pub control_hover: Hsla,
    pub control_pressed: Hsla,
    pub control_border: Hsla,
    /// Subtle buttons and navigation items.
    pub subtle_hover: Hsla,
    pub subtle_pressed: Hsla,

    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub accent_pressed: Hsla,
    pub accent_fg: Hsla,
    pub focus: Hsla,

    pub destructive: Hsla,
    pub destructive_bg: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
    pub success_bg: Hsla,
    pub success_fg: Hsla,
    pub warning_bg: Hsla,
    pub warning_fg: Hsla,
    pub info_bg: Hsla,
    pub info_fg: Hsla,
    pub muted_bg: Hsla,

    /// Toggle switch when off.
    pub switch_border: Hsla,
    pub switch_knob: Hsla,
    /// The dimmed layer behind a dialog.
    pub smoke: Hsla,
    pub dialog: Hsla,
    pub dialog_footer: Hsla,
}

impl Global for Theme {}

fn c(hex: u32) -> Hsla {
    rgb(hex).into()
}

fn ca(hex_rgba: u32) -> Hsla {
    rgba(hex_rgba).into()
}

impl Theme {
    pub fn new(appearance: WindowAppearance) -> Theme {
        let dark = matches!(appearance, WindowAppearance::Dark | WindowAppearance::VibrantDark);
        let mica = system::mica_supported();
        let win11 = system::windows_build() >= 22000;
        let font: SharedString = if win11 { "Segoe UI Variable Text" } else { "Segoe UI" }.into();
        let display_font: SharedString =
            if win11 { "Segoe UI Variable Display" } else { "Segoe UI" }.into();
        // Segoe MDL2 Assets has the same code points on Windows 10.
        let icon_font: SharedString =
            if win11 { "Segoe Fluent Icons" } else { "Segoe MDL2 Assets" }.into();

        let palette = system::accent_palette();
        // AccentPalette: Light3, Light2, Light1, Base, Dark1, Dark2, Dark3.
        let (accent, accent_hover, accent_pressed) = match palette {
            Some(p) if dark => (p[1], p[2], p[2]),
            Some(p) => (p[4], p[3], p[5]),
            None if dark => (0x60cdff, 0x5bb9e6, 0x56a7cf),
            None => (0x005fb8, 0x1a6fc0, 0x3380c7),
        };
        let mut accent_pressed = c(accent_pressed);
        accent_pressed.a = 0.9;

        let mut t = if dark {
            Theme {
                dark,
                mica,
                font,
                display_font,
                icon_font,
                background: c(0x202020),
                foreground: c(0xffffff),
                muted_fg: ca(0xffffffb3),
                card: c(0x2b2b2b),
                border: ca(0xffffff14),
                control: ca(0xffffff0f),
                control_hover: ca(0xffffff17),
                control_pressed: ca(0xffffff0a),
                control_border: ca(0xffffff17),
                subtle_hover: ca(0xffffff0f),
                subtle_pressed: ca(0xffffff0a),
                accent: c(accent),
                accent_hover: c(accent_hover),
                accent_pressed,
                accent_fg: c(0x000000),
                focus: c(0xffffff),
                destructive: c(0xff99a4),
                destructive_bg: c(0x442726),
                success: c(0x6ccb5f),
                warning: c(0xfce100),
                info: c(0x60cdff),
                success_bg: c(0x0e3a0e),
                success_fg: c(0x6ccb5f),
                warning_bg: c(0x433519),
                warning_fg: c(0xfce100),
                info_bg: c(0x0b2f4d),
                info_fg: c(0x60cdff),
                muted_bg: c(0x323232),
                switch_border: ca(0xffffff99),
                switch_knob: ca(0xffffffb3),
                smoke: ca(0x0000004d),
                dialog: c(0x2b2b2b),
                dialog_footer: c(0x202020),
            }
        } else {
            Theme {
                dark,
                mica,
                font,
                display_font,
                icon_font,
                background: c(0xf3f3f3),
                foreground: c(0x1a1a1a),
                muted_fg: c(0x5d5d5d),
                card: c(0xfbfbfb),
                border: ca(0x00000014),
                control: c(0xffffff),
                control_hover: c(0xf6f6f6),
                control_pressed: c(0xf5f5f5),
                control_border: ca(0x0000001a),
                subtle_hover: ca(0x0000000d),
                subtle_pressed: ca(0x00000008),
                accent: c(accent),
                accent_hover: c(accent_hover),
                accent_pressed,
                accent_fg: c(0xffffff),
                focus: c(0x1a1a1a),
                destructive: c(0xc42b1c),
                destructive_bg: c(0xfde7e9),
                success: c(0x0f7b0f),
                warning: c(0x9d5d00),
                info: c(0x005fb8),
                success_bg: c(0xdff6dd),
                success_fg: c(0x0e700e),
                warning_bg: c(0xfff4ce),
                warning_fg: c(0x6d5200),
                info_bg: c(0xe0efff),
                info_fg: c(0x0f548c),
                muted_bg: c(0xececec),
                switch_border: ca(0x00000099),
                switch_knob: ca(0x000000b3),
                smoke: ca(0x0000004d),
                dialog: c(0xffffff),
                dialog_footer: c(0xf3f3f3),
            }
        };
        if mica {
            // Transparent base, translucent cards (same values as `html.mica` in v0.3).
            t.background = ca(0x00000000);
            if dark {
                t.card = ca(0xffffff0d);
                t.control = ca(0xffffff12);
                t.control_hover = ca(0xffffff1a);
            } else {
                t.card = ca(0xffffffb3);
                t.control = ca(0xffffffbf);
                t.control_hover = ca(0xffffff8c);
            }
        }
        t
    }
}

pub fn theme(cx: &App) -> &Theme {
    cx.global::<Theme>()
}
