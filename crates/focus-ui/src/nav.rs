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
    let Ok(nodes) = root.query_selector_all("button.f:not([disabled]):not([data-skip])") else {
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
    if let Some(moved) = settings_move(&elements[current], direction) {
        return moved;
    }
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

/// Paramètres : le menu et la rubrique restent deux colonnes. On se déplace d'abord
/// dans sa colonne ; à son bord, on rejoint la rubrique active du menu (et non celle
/// qui est à la même hauteur) ou le premier réglage. `None` : navigation ordinaire.
fn settings_move(current: &HtmlElement, direction: Direction) -> Option<bool> {
    let in_body = current.closest(".settings-body").ok().flatten().is_some();
    let in_nav = current.closest(".settings-nav").ok().flatten().is_some();
    let trapped = current.closest(".dialog, .osk, .panel").ok().flatten().is_some();
    if trapped || (!in_body && !in_nav) {
        return None;
    }
    let document = document()?;
    let rtl = document
        .document_element()
        .and_then(|root| root.get_attribute("dir"))
        .is_some_and(|dir| dir == "rtl");
    let (toward_nav, toward_body) = if rtl {
        (Direction::Right, Direction::Left)
    } else {
        (Direction::Left, Direction::Right)
    };
    let pick = |selector: &str| -> Option<HtmlElement> {
        document.query_selector(selector).ok().flatten()?.dyn_into::<HtmlElement>().ok()
    };
    // D'abord dans la même colonne (rangées de langues ou de niveaux comprises).
    let column = if in_body { ".settings-body" } else { ".settings-nav" };
    let here = rect_of(current);
    let horizontal = matches!(direction, Direction::Left | Direction::Right);
    let elements: Vec<HtmlElement> = focusables()
        .into_iter()
        .filter(|element| element.closest(column).ok().flatten().is_some())
        // Gauche et droite restent sur la même rangée (niveaux, langues).
        .filter(|element| {
            let rect = rect_of(element);
            !horizontal || (rect.y < here.y + here.h && here.y < rect.y + rect.h)
        })
        .collect();
    if let Some(current) = active(&elements) {
        let rects: Vec<Rect> = elements.iter().map(rect_of).collect();
        if let Some(index) = spatial::next(&rects, current, direction) {
            focus(&elements[index]);
            return Some(true);
        }
    }
    // Au bord de la colonne : on passe à l'autre.
    let target = if in_body && direction == toward_nav {
        pick(".settings-nav .nav-item.active")
    } else if in_nav && direction == toward_body {
        pick(".settings-body button.f:not([disabled]):not([data-skip])")
    } else {
        None
    };
    if let Some(target) = target {
        focus(&target);
        return Some(true);
    }
    // Vers le bord extérieur : rien à gauche du menu, rien à droite de la rubrique.
    Some(false)
}
