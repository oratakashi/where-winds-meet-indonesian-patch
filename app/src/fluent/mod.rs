//! A small WinUI 3 (Fluent) look-alike on top of plain GPUI: just the controls the window
//! needs. Colours come from [`Theme`]; text uses Segoe UI Variable and icons are glyphs of the
//! system icon font (Segoe Fluent Icons / Segoe MDL2 Assets), so no assets are bundled.
pub mod theme;

pub use theme::{Theme, theme};

use gpui::prelude::*;
use gpui::{
    App, ClickEvent, Div, ElementId, FontWeight, Hsla, Role, SharedString, Stateful, Toggled,
    Window, div, px,
};
use std::rc::Rc;

/// Segoe Fluent Icons code points.
pub mod glyph {
    pub const HOME: &str = "\u{E80F}";
    pub const SETTINGS: &str = "\u{E713}";
    pub const HISTORY: &str = "\u{E81C}";
    pub const ADD: &str = "\u{E710}";
    pub const PAUSE: &str = "\u{E769}";
    pub const ERROR: &str = "\u{EA39}";
    pub const SYNC: &str = "\u{E895}";
    pub const WARNING: &str = "\u{E7BA}";
    pub const ACCEPT: &str = "\u{E73E}";
    pub const REFRESH: &str = "\u{E72C}";
    pub const FOLDER_OPEN: &str = "\u{E8B7}";
    pub const DELETE: &str = "\u{E74D}";
    pub const ERROR_BADGE: &str = "\u{EA39}";
    pub const CANCEL: &str = "\u{E711}";
    pub const INFO: &str = "\u{E946}";
}

pub type Handler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Body text: 14/20. Caption: 12/16.
pub const BODY: f32 = 14.;
pub const CAPTION: f32 = 12.;

pub fn icon(glyph: &'static str, size: f32, color: Hsla, cx: &App) -> Div {
    div()
        .flex_none()
        .font_family(theme(cx).icon_font.clone())
        .text_size(px(size))
        .line_height(px(size))
        .text_color(color)
        .child(glyph)
}

pub fn caption(text: impl Into<SharedString>, cx: &App) -> Div {
    div()
        .text_size(px(CAPTION))
        .line_height(px(16.))
        .text_color(theme(cx).muted_fg)
        .child(text.into())
}

pub fn page_title(text: &'static str, cx: &App) -> Stateful<Div> {
    div()
        .id(text)
        .mb(px(4.))
        .font_family(theme(cx).display_font.clone())
        .text_size(px(28.))
        .line_height(px(36.))
        .font_weight(FontWeight::SEMIBOLD)
        .role(Role::Heading)
        .child(text)
}

pub fn subtitle(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(px(BODY))
        .line_height(px(20.))
        .font_weight(FontWeight::SEMIBOLD)
        .child(text.into())
}

/// A layer card (`CardBackgroundFillColorDefault` + `CardStrokeColorDefault`).
pub fn card(cx: &App) -> Div {
    let t = theme(cx);
    div()
        .flex()
        .flex_col()
        .bg(t.card)
        .border_1()
        .border_color(t.border)
        .rounded(px(8.))
        .overflow_hidden()
}

pub fn divider(cx: &App) -> Div {
    div().h(px(1.)).w_full().flex_none().bg(theme(cx).border)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    Accent,
    Standard,
    Subtle,
}

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    glyph: Option<&'static str>,
    style: ButtonStyle,
    small: bool,
    wide: bool,
    disabled: bool,
    on_click: Option<Handler>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Button {
            id: id.into(),
            label: label.into(),
            glyph: None,
            style: ButtonStyle::Standard,
            small: false,
            wide: false,
            disabled: false,
            on_click: None,
        }
    }
    pub fn icon(mut self, glyph: &'static str) -> Self {
        self.glyph = Some(glyph);
        self
    }
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }
    pub fn small(mut self) -> Self {
        self.small = true;
        self
    }
    /// Share the row's width with the other wide buttons (dialog footers).
    pub fn wide(mut self) -> Self {
        self.wide = true;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx).clone();
        let transparent = gpui::transparent_black();
        let (bg, hover, pressed, fg, border) = match self.style {
            ButtonStyle::Accent => (t.accent, t.accent_hover, t.accent_pressed, t.accent_fg, None),
            ButtonStyle::Standard => {
                (t.control, t.control_hover, t.control_pressed, t.foreground, Some(t.control_border))
            }
            ButtonStyle::Subtle => {
                (transparent, t.subtle_hover, t.subtle_pressed, t.foreground, None)
            }
        };
        let label = self.label.clone();
        let mut el = div()
            .id(self.id)
            .flex()
            .when(self.wide, |el| el.flex_1())
            .when(!self.wide, |el| el.flex_none())
            .items_center()
            .justify_center()
            .gap(px(8.))
            .h(px(if self.small { 28. } else { 32. }))
            .px(px(if self.small { 10. } else { 14. }))
            .rounded(px(4.))
            .border_1()
            .border_color(border.unwrap_or(transparent))
            .bg(bg)
            .text_size(px(BODY))
            .line_height(px(20.))
            .text_color(fg)
            .whitespace_nowrap()
            .role(Role::Button)
            .aria_label(label.clone());
        if let Some(g) = self.glyph {
            el = el.child(icon(g, 14., fg, cx));
        }
        el = el.child(label);
        if self.disabled {
            return el.opacity(0.45);
        }
        let focus = t.focus;
        el = el
            .focusable()
            .tab_stop(true)
            .hover(move |s| s.bg(hover))
            .active(move |s| s.bg(pressed))
            .focus_visible(move |s| s.border_color(focus));
        if let Some(f) = self.on_click {
            el = el.on_click(move |e, w, cx| f(e, w, cx));
        }
        el
    }
}

/// WinUI `ToggleSwitch` (without its label; the setting row carries the text).
#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    label: SharedString,
    on: bool,
    on_change: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, on: bool) -> Self {
        Toggle { id: id.into(), label: label.into(), on, on_change: None }
    }
    pub fn on_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx).clone();
        let on = self.on;
        let focus = t.focus;
        let hover = if on { t.accent_hover } else { t.subtle_hover };
        let mut el = div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .w(px(40.))
            .h(px(20.))
            .px(px(3.))
            .rounded_full()
            .border_1()
            .border_color(if on { t.accent } else { t.switch_border })
            .bg(if on { t.accent } else { gpui::transparent_black() })
            .when(on, |el| el.justify_end())
            .focusable()
            .tab_stop(true)
            .role(Role::Switch)
            .aria_label(self.label)
            .aria_toggled(if on { Toggled::True } else { Toggled::False })
            .hover(move |s| s.bg(hover))
            .focus_visible(move |s| s.border_2().border_color(focus))
            .child(
                div()
                    .size(px(12.))
                    .rounded_full()
                    .bg(if on { t.accent_fg } else { t.switch_knob }),
            );
        if let Some(f) = self.on_change {
            el = el.on_click(move |_, w, cx| f(!on, w, cx));
        }
        el
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Success,
    Warning,
    Info,
    Muted,
    Error,
}

/// Status pill (WinUI `InfoBadge`-like, with text).
pub fn badge(label: impl Into<SharedString>, tone: Tone, cx: &App) -> Div {
    let t = theme(cx);
    let (bg, fg) = match tone {
        Tone::Success => (t.success_bg, t.success_fg),
        Tone::Warning => (t.warning_bg, t.warning_fg),
        Tone::Info => (t.info_bg, t.info_fg),
        Tone::Muted => (t.muted_bg, t.muted_fg),
        Tone::Error => (t.destructive, if t.dark { gpui::black() } else { gpui::white() }),
    };
    div()
        .flex_none()
        .px(px(8.))
        .py(px(1.))
        .rounded_full()
        .bg(bg)
        .text_color(fg)
        .text_size(px(CAPTION))
        .line_height(px(16.))
        .font_weight(FontWeight::MEDIUM)
        .child(label.into())
}

/// WinUI `InfoBar`, error severity.
pub fn error_bar(title: &'static str, message: SharedString, on_close: Handler, cx: &App) -> Stateful<Div> {
    let t = theme(cx).clone();
    div()
        .id("error-bar")
        .flex()
        .items_start()
        .gap(px(12.))
        .px(px(16.))
        .py(px(12.))
        .rounded(px(4.))
        .border_1()
        .border_color(t.border)
        .bg(t.destructive_bg)
        .role(Role::Alert)
        .child(div().pt(px(2.)).child(icon(glyph::ERROR_BADGE, 16., t.destructive, cx)))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .gap(px(2.))
                .child(subtitle(title))
                .child(div().text_size(px(BODY)).line_height(px(20.)).child(message)),
        )
        .child(
            Button::new("error-bar-close", "")
                .icon(glyph::CANCEL)
                .style(ButtonStyle::Subtle)
                .small()
                .on_click(move |e, w, cx| on_close(e, w, cx)),
        )
}

/// WinUI `InfoBar`, informational severity, with one action button.
pub fn info_bar(
    id: &'static str,
    title: SharedString,
    message: SharedString,
    action: SharedString,
    on_action: Handler,
    cx: &App,
) -> Stateful<Div> {
    let t = theme(cx).clone();
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(12.))
        .px(px(16.))
        .py(px(12.))
        .rounded(px(4.))
        .border_1()
        .border_color(t.border)
        .bg(t.info_bg)
        .role(Role::Status)
        .child(icon(glyph::INFO, 16., t.info, cx))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .gap(px(2.))
                .child(subtitle(title))
                .child(div().text_size(px(BODY)).line_height(px(20.)).child(message)),
        )
        .child(
            Button::new((id, 1usize), action)
                .style(ButtonStyle::Accent)
                .on_click(move |e, w, cx| on_action(e, w, cx)),
        )
}
