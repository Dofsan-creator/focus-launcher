//! Navigation commune aux trois entrées, sur de vrais boutons HTML.

use focus_core::spatial::{self, Direction, Rect};
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

fn document() -> Option<Document> {
    web_sys::window()?.document()
}

/// Racine active : le dialogue ou le panneau ouvert piège le focus, comme attendu.
fn scope(document: &Document) -> Option<Element> {
    for selector in [".dialog", ".osk", ".panel"] {
        if let Ok(Some(found)) = document.query_selector(selector) {
            return Some(found);
        }
    }
    document.body().map(Into::into)
}

fn focusables() -> Vec<HtmlElement> {
    let Some(document) = document() else {
        return Vec::new();
    };
    let Some(root) = scope(&document) else {
        return Vec::new();
    };
    let Ok(nodes) = root.query_selector_all("button.f:not([disabled])") else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|index| nodes.item(index)?.dyn_into::<HtmlElement>().ok())
        .collect()
}

fn rect_of(element: &HtmlElement) -> Rect {
    let rect = element.get_bounding_client_rect();
    Rect {
        x: rect.x(),
        y: rect.y(),
        w: rect.width(),
        h: rect.height(),
    }
}

/// Élément actif s'il est dans la portée courante.
pub fn active(elements: &[HtmlElement]) -> Option<usize> {
    let active = document()?.active_element()?;
    elements.iter().position(|element| {
        let node: &Element = element;
        *node == active
    })
}

/// Donne le focus sans provoquer de défilement brutal.
fn focus(element: &HtmlElement) {
    let _ = element.focus();
    let options = web_sys::ScrollIntoViewOptions::new();
    options.set_block(web_sys::ScrollLogicalPosition::Nearest);
    options.set_inline(web_sys::ScrollLogicalPosition::Nearest);
    element.scroll_into_view_with_scroll_into_view_options(&options);
}

/// Place le focus sur l'élément marqué `data-auto`, sinon sur le premier.
pub fn focus_first() {
    let elements = focusables();
    let preferred = elements
        .iter()
        .find(|element| element.has_attribute("data-auto"));
    if let Some(element) = preferred.or(elements.first()) {
        focus(element);
    }
}

/// Déplace le focus. Renvoie vrai si quelque chose a bougé.
pub fn move_focus(direction: Direction) -> bool {
    let elements = focusables();
    let Some(current) = active(&elements) else {
        focus_first();
        return true;
    };
    let rects: Vec<Rect> = elements.iter().map(rect_of).collect();
    match spatial::next(&rects, current, direction) {
        Some(index) => {
            focus(&elements[index]);
            true
        }
        None => false,
    }
}

/// Valide l'élément actif.
pub fn confirm() {
    let elements = focusables();
    if let Some(index) = active(&elements) {
        elements[index].click();
    } else {
        focus_first();
    }
}

/// Indice de carte de l'élément actif, sur l'accueil.
pub fn active_card() -> Option<i32> {
    document()?
        .active_element()?
        .get_attribute("data-card")?
        .parse()
        .ok()
}
