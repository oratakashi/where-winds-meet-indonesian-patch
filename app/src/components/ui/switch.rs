use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

mod components {
    use super::*;
    clx! {SwitchLabel, span, "text-sm font-medium"}
}

pub use components::*;

// Adapted from the Rust/UI registry: the original keeps its own state and reports nothing.
// Here the parent owns the state (`checked`) and is told about clicks (`on_change`).
#[component]
pub fn Switch(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(into, optional, default = "Toggle switch".to_string())] aria_label: String,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let state = move || if checked.get() { "checked" } else { "unchecked" };

    let track_class = tw_merge!(
        "inline-flex h-5 w-10 shrink-0 cursor-pointer items-center rounded-full border border-foreground/60 px-[3px] transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-foreground disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-primary data-[state=checked]:bg-primary data-[state=unchecked]:bg-transparent hover:data-[state=unchecked]:bg-foreground/5",
        class
    );

    view! {
        <button
            data-name="Switch"
            type="button"
            role="switch"
            aria-checked=move || checked.get().to_string()
            aria-label=aria_label
            data-state=state
            class=track_class
            on:click=move |_| on_change.run(!checked.get_untracked())
        >
            <span
                data-state=state
                class="block pointer-events-none size-3 rounded-full transition-all data-[state=checked]:translate-x-5 data-[state=checked]:bg-primary-foreground data-[state=unchecked]:translate-x-0 data-[state=unchecked]:bg-foreground/70"
            />
        </button>
    }
}
