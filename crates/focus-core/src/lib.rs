//! Cœur de Focus Launcher : modèles et logique sans aucune dépendance à Windows.
//!
//! Tout ce qui décide (liste blanche, règles, plan, journal, restauration) vit
//! ici pour être testé avec de faux systèmes, avant de toucher une vraie machine.
#![deny(missing_docs)]

pub mod art;
pub mod engine;
pub mod journal;
pub mod model;
pub mod plan;
pub mod report;
pub mod rules;
pub mod session;
pub mod spatial;
pub mod whitelist;
