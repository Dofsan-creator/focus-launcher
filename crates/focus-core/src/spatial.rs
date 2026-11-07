//! Navigation spatiale : trouve l'élément voisin dans une direction.
//!
//! L'interface n'a qu'un chemin pour les trois entrées : la manette et les flèches
//! déplacent le focus entre de vrais boutons, d'après leur position à l'écran.
//! Le raisonnement étant géométrique, le droite-gauche fonctionne sans cas spécial.

use serde::{Deserialize, Serialize};

/// Rectangle d'un élément à l'écran.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    /// Bord gauche.
    pub x: f64,
    /// Bord haut.
    pub y: f64,
    /// Largeur.
    pub w: f64,
    /// Hauteur.
    pub h: f64,
}

impl Rect {
    fn center(&self) -> (f64, f64) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
}

/// Direction de déplacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// Haut.
    Up,
    /// Bas.
    Down,
    /// Gauche.
    Left,
    /// Droite.
    Right,
}

/// Renvoie l'indice du meilleur voisin de `current`, s'il existe.
pub fn next(rects: &[Rect], current: usize, direction: Direction) -> Option<usize> {
    let origin = rects.get(current)?;
    let (ox, oy) = origin.center();
    rects
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != current)
        .filter_map(|(index, rect)| {
            let (x, y) = rect.center();
            let (along, across) = match direction {
                Direction::Up => (oy - y, (x - ox).abs()),
                Direction::Down => (y - oy, (x - ox).abs()),
                Direction::Left => (ox - x, (y - oy).abs()),
                Direction::Right => (x - ox, (y - oy).abs()),
            };
            // Un élément dans un cône de 60° autour de la direction, ou aligné.
            let aligned = match direction {
                Direction::Up | Direction::Down => {
                    rect.x < origin.x + origin.w && origin.x < rect.x + rect.w
                }
                Direction::Left | Direction::Right => {
                    rect.y < origin.y + origin.h && origin.y < rect.y + rect.h
                }
            };
            (along > 0.5 && (aligned || across <= along * 1.8))
                .then_some((index, along, across, aligned))
        })
        .min_by(|a, b| {
            let cost = |(_, along, across, aligned): &(usize, f64, f64, bool)| {
                along + across * if *aligned { 0.5 } else { 2.5 }
            };
            cost(a).total_cmp(&cost(b))
        })
        .map(|(index, ..)| index)
}

/// Depuis `start`, liste les éléments atteignables avec les quatre directions.
pub fn reachable(rects: &[Rect], start: usize) -> Vec<usize> {
    let mut seen = vec![false; rects.len()];
    let mut stack = vec![start];
    while let Some(index) = stack.pop() {
        if seen.get(index).copied().unwrap_or(true) {
            continue;
        }
        seen[index] = true;
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(neighbour) = next(rects, index, direction) {
                stack.push(neighbour);
            }
        }
    }
    seen.iter()
        .enumerate()
        .filter_map(|(index, on)| on.then_some(index))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: f64, y: f64) -> Rect {
        Rect {
            x,
            y,
            w: 100.0,
            h: 40.0,
        }
    }

    #[test]
    fn right_should_pick_the_next_button_of_the_row() {
        let rects = [rect(0.0, 0.0), rect(120.0, 0.0), rect(240.0, 0.0)];
        assert_eq!(next(&rects, 0, Direction::Right), Some(1));
    }

    #[test]
    fn down_should_pick_the_aligned_button() {
        let rects = [rect(0.0, 0.0), rect(500.0, 60.0), rect(0.0, 60.0)];
        assert_eq!(next(&rects, 0, Direction::Down), Some(2));
    }

    #[test]
    fn edge_should_return_none() {
        let rects = [rect(0.0, 0.0), rect(120.0, 0.0)];
        assert_eq!(next(&rects, 0, Direction::Left), None);
    }

    #[test]
    fn out_of_range_index_should_return_none() {
        assert_eq!(next(&[], 3, Direction::Up), None);
    }

    #[test]
    fn grid_should_be_fully_reachable_from_any_corner() {
        let rects: Vec<Rect> = (0..4)
            .flat_map(|r| (0..5).map(move |c| rect(f64::from(c) * 120.0, f64::from(r) * 60.0)))
            .collect();
        assert_eq!(reachable(&rects, 19).len(), 20);
    }

    #[test]
    fn staggered_layout_should_stay_reachable() {
        let rects = [
            rect(0.0, 0.0),
            rect(300.0, 0.0),
            rect(150.0, 100.0),
            rect(40.0, 200.0),
        ];
        assert_eq!(reachable(&rects, 0).len(), 4);
    }
}
