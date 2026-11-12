//! Icônes dessinées pour Focus Launcher : traits simples sur une grille de 24.
//! Aucune icône de constructeur ni de bibliothèque tierce (règle d'originalité).

use leptos::prelude::*;

fn markup(name: &str, level: u8) -> String {
    let dim = |on: bool| if on { "1" } else { "0.28" };
    match name {
        "play" => r#"<path d="M8 5.5v13l11-6.5z" fill="currentColor" stroke="none"/>"#.into(),
        "more" => r#"<circle cx="5.5" cy="12" r="1.7" fill="currentColor" stroke="none"/><circle cx="12" cy="12" r="1.7" fill="currentColor" stroke="none"/><circle cx="18.5" cy="12" r="1.7" fill="currentColor" stroke="none"/>"#.into(),
        "settings" => r#"<circle cx="12" cy="12" r="2.8"/><circle cx="12" cy="12" r="6.4"/><path d="M12 2.6v2.6M12 18.8v2.6M2.6 12h2.6M18.8 12h2.6M5.4 5.4l1.8 1.8M16.8 16.8l1.8 1.8M5.4 18.6l1.8-1.8M16.8 7.2l1.8-1.8"/>"#.into(),
        "power" => r#"<path d="M12 3.2v8.3"/><path d="M6.4 6.7a7.8 7.8 0 1 0 11.2 0"/>"#.into(),
        "search" => r#"<circle cx="10.8" cy="10.8" r="6.2"/><path d="M15.4 15.4l5 5"/>"#.into(),
        "store" => r#"<path d="M5 8.5h14l-1.2 10.8a1.6 1.6 0 0 1-1.6 1.4H7.8a1.6 1.6 0 0 1-1.6-1.4z"/><path d="M9 8.5V6.8a3 3 0 0 1 6 0v1.7"/>"#.into(),
        "add" => r#"<path d="M12 5v14M5 12h14"/>"#.into(),
        "star" => r#"<path d="M12 4l2.4 5 5.4.6-4 3.7 1.1 5.3L12 16l-4.9 2.6 1.1-5.3-4-3.7 5.4-.6z"/>"#.into(),
        "star-full" => r#"<path d="M12 4l2.4 5 5.4.6-4 3.7 1.1 5.3L12 16l-4.9 2.6 1.1-5.3-4-3.7 5.4-.6z" fill="currentColor"/>"#.into(),
        "trash" => r#"<path d="M5 7h14M10 7V5h4v2M7 7l1 12.5h8L17 7"/>"#.into(),
        "lock" => r#"<rect x="6" y="11" width="12" height="9" rx="2"/><path d="M9 11V8.2a3 3 0 0 1 6 0V11"/>"#.into(),
        "check" => r#"<path d="M5 12.5l4.5 4.5L19 7.5"/>"#.into(),
        "close" => r#"<path d="M6.5 6.5l11 11M17.5 6.5l-11 11"/>"#.into(),
        "back" => r#"<path d="M14.5 6L8.5 12l6 6"/>"#.into(),
        "next" => r#"<path d="M9.5 6l6 6-6 6"/>"#.into(),
        "volume" => r#"<path d="M5 10v4h3l4.2 3.6V6.4L8 10z"/><path d="M15.6 9.4a3.6 3.6 0 0 1 0 5.2M17.9 7.1a7 7 0 0 1 0 9.8"/>"#.into(),
        "moon" => r#"<path d="M19 14.6A7.6 7.6 0 0 1 9.4 5a7.6 7.6 0 1 0 9.6 9.6z"/>"#.into(),
        "refresh" => r#"<path d="M19 12a7 7 0 1 1-2.2-5.1"/><path d="M19.2 4.2v4.2H15"/>"#.into(),
        "library" => r#"<rect x="4" y="4" width="7" height="7" rx="1.6"/><rect x="13" y="4" width="7" height="7" rx="1.6"/><rect x="4" y="13" width="7" height="7" rx="1.6"/><rect x="13" y="13" width="7" height="7" rx="1.6"/>"#.into(),
        "display" => r#"<rect x="3" y="5" width="18" height="12" rx="2"/><path d="M9 20h6M12 17v3"/>"#.into(),
        "bolt" => r#"<path d="M13 3L5.5 13.5h6L10.5 21l8-10.5h-6z"/>"#.into(),
        "globe" => r#"<circle cx="12" cy="12" r="8.2"/><path d="M3.8 12h16.4M12 3.8c2.6 2.4 2.6 14 0 16.4M12 3.8c-2.6 2.4-2.6 14 0 16.4"/>"#.into(),
        "info" => r#"<circle cx="12" cy="12" r="8.2"/><path d="M12 11v5M12 7.8v.4"/>"#.into(),
        "ethernet" => r#"<rect x="5.5" y="9" width="13" height="9.5" rx="1.6"/><path d="M9 9V6h6v3M9.5 13v2.2M12 13v2.2M14.5 13v2.2"/>"#.into(),
        "bluetooth" => r#"<path d="M7.2 7.6l9.6 8.6-4.8 4.3V3.5l4.8 4.3-9.6 8.6"/>"#.into(),
        "gamepad" => r#"<path d="M7.2 8h9.6a4 4 0 0 1 4 4.4l-.5 3.7a2.2 2.2 0 0 1-3.9 1L15 15.2H9l-1.4 1.9a2.2 2.2 0 0 1-3.9-1l-.5-3.7A4 4 0 0 1 7.2 8z"/><path d="M7.6 10.8v3M6.1 12.3h3"/><circle cx="15.6" cy="11.4" r="1" fill="currentColor" stroke="none"/><circle cx="17.4" cy="13.2" r="1" fill="currentColor" stroke="none"/>"#.into(),
        "wifi" => format!(
            r#"<circle cx="12" cy="18.6" r="1.4" fill="currentColor" stroke="none" opacity="{}"/><path d="M8.6 15.2a4.8 4.8 0 0 1 6.8 0" opacity="{}"/><path d="M5.6 12.2a9 9 0 0 1 12.8 0" opacity="{}"/><path d="M2.7 9.2a13.2 13.2 0 0 1 18.6 0" opacity="{}"/>"#,
            dim(level >= 1), dim(level >= 2), dim(level >= 3), dim(level >= 4)
        ),
        "battery" => {
            let width = f32::from(level.min(3)) / 3.0 * 12.0;
            format!(
                r#"<rect x="3" y="8" width="16" height="8" rx="2"/><path d="M21 11v2"/><rect x="5" y="10" width="{width:.1}" height="4" rx="1" fill="currentColor" stroke="none"/>"#
            )
        }
        "minus" => r#"<path d="M5 12h14"/>"#.into(),
        "pointer" => r#"<path d="M6 3.5l12.5 7.6-5.6 1.3 3.3 6.1-2.4 1.3-3.3-6.1-4.2 3.9z"/>"#.into(),
        "download" => r#"<path d="M12 4v10.5M7.5 10.2l4.5 4.5 4.5-4.5"/><path d="M5 19.5h14"/>"#.into(),
        "offline" => r#"<path d="M4 4l16 16"/><path d="M8.6 15.2a4.8 4.8 0 0 1 6.8 0M5.6 12.2a9 9 0 0 1 4.4-2.4M14.6 9.9a9 9 0 0 1 3.8 2.3"/>"#.into(),
        _ => String::new(),
    }
}

/// Icône. `level` règle les icônes à paliers (Wi-Fi, batterie).
#[component]
pub fn Icon(name: &'static str, #[prop(optional)] level: u8) -> impl IntoView {
    view! {
        <svg class="icon" viewBox="0 0 24 24" aria-hidden="true" fill="none" stroke="currentColor"
            stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"
            inner_html=markup(name, level)></svg>
    }
}

