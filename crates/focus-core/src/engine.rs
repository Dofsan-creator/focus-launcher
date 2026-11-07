//! Classement de chaque processus et service. La liste blanche gagne toujours.

use crate::model::{EntryKind, Store};
use crate::rules::{Rule, RuleAction, RuleFile, RuleType};
use crate::whitelist;
use serde::{Deserialize, Serialize};

/// Contexte de la session à préparer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionContext {
    /// Type de l'entrée lancée.
    pub kind: EntryKind,
    /// Boutique de l'entrée.
    pub store: Store,
    /// Steam Input est utilisé : Steam doit rester ouvert, quel que soit le jeu.
    pub steam_input_active: bool,
    /// Garder Discord et les applications sociales intouchées.
    pub keep_social: bool,
}

/// Résultat du classement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "class")]
pub enum Classification {
    /// Jamais touché.
    Protected {
        /// Motif.
        why: ProtectionReason,
    },
    /// Une règle s'applique.
    Actionable {
        /// Règle retenue.
        rule: Rule,
    },
    /// Inconnu : proposé à l'utilisateur, jamais coupé automatiquement.
    Uncertain,
}

/// Pourquoi un élément est protégé.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProtectionReason {
    /// Liste blanche compilée.
    Whitelist,
    /// Ajouté par l'utilisateur.
    UserList,
    /// Client de boutique nécessaire.
    StoreClient,
    /// Logiciel de création.
    Creative,
    /// Application sociale.
    Social,
}

/// Classe un processus. L'ordre des tests est la garantie de sécurité.
pub fn classify_process(
    name: &str,
    file: &RuleFile,
    profile_rules: &[Rule],
    ctx: &SessionContext,
) -> Classification {
    let key = whitelist::normalize(name);
    if whitelist::is_protected_process(&key) {
        return protected(ProtectionReason::Whitelist);
    }
    if in_user_list(&key, file) {
        return protected(ProtectionReason::UserList);
    }
    if let Some(store) = whitelist::store_of_process(&key) {
        let steam_needed = ctx.steam_input_active && store == Store::Steam;
        if store == ctx.store || steam_needed {
            return protected(ProtectionReason::StoreClient);
        }
    }
    if whitelist::is_creative_process(&key) {
        return protected(ProtectionReason::Creative);
    }
    if whitelist::is_social_process(&key) && (ctx.keep_social || ctx.kind == EntryKind::Software) {
        return protected(ProtectionReason::Social);
    }
    match find_rule(&key, RuleType::Process, profile_rules) {
        Some(rule) => match restrict_for_kind(rule, &key, ctx) {
            Some(rule) => Classification::Actionable { rule },
            None => protected(ProtectionReason::Creative),
        },
        None => Classification::Uncertain,
    }
}

/// Classe un service par nom interne.
pub fn classify_service(
    name: &str,
    file: &RuleFile,
    profile_rules: &[Rule],
    ctx: &SessionContext,
) -> Classification {
    let key = name.trim().to_lowercase();
    if whitelist::is_protected_service(&key) {
        return protected(ProtectionReason::Whitelist);
    }
    if in_user_list(&key, file) {
        return protected(ProtectionReason::UserList);
    }
    // Profil Logiciel : arrêt de services désactivé par défaut (v4.1, section 3).
    if ctx.kind != EntryKind::Game {
        return Classification::Uncertain;
    }
    match find_rule(&key, RuleType::Service, profile_rules) {
        Some(rule) => Classification::Actionable { rule: rule.clone() },
        None => Classification::Uncertain,
    }
}

fn protected(why: ProtectionReason) -> Classification {
    Classification::Protected { why }
}

fn in_user_list(key: &str, file: &RuleFile) -> bool {
    file.never_touch
        .iter()
        .any(|name| whitelist::normalize(name) == key)
}

fn find_rule<'a>(key: &str, rule_type: RuleType, rules: &'a [Rule]) -> Option<&'a Rule> {
    rules
        .iter()
        .find(|rule| rule.rule_type == rule_type && whitelist::normalize(&rule.name) == key)
}

/// Applique les limites du type d'entrée : un logiciel ne ferme ni ne suspend
/// rien, une application sociale n'est jamais suspendue.
fn restrict_for_kind(rule: &Rule, key: &str, ctx: &SessionContext) -> Option<Rule> {
    let touches_state = matches!(rule.action, RuleAction::Close | RuleAction::Suspend);
    if ctx.kind == EntryKind::Other {
        return None;
    }
    if ctx.kind == EntryKind::Software && touches_state {
        return None;
    }
    if whitelist::is_social_process(key) && rule.action == RuleAction::Suspend {
        return None;
    }
    Some(rule.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{Profile, Risk};

    fn rule(name: &str, rule_type: RuleType, action: RuleAction) -> Rule {
        Rule {
            rule_type,
            name: name.to_owned(),
            action,
            restore_after_game: true,
            allow_force: false,
            risk: Risk::Low,
            reason: "test".to_owned(),
        }
    }

    fn file_with(rules: Vec<Rule>) -> RuleFile {
        RuleFile {
            version: 1,
            profiles: vec![Profile {
                name: "p".to_owned(),
                simulation_only: false,
                rules,
            }],
            never_touch: vec!["MyTool".to_owned()],
        }
    }

    fn game_ctx() -> SessionContext {
        SessionContext {
            kind: EntryKind::Game,
            store: Store::Steam,
            steam_input_active: false,
            keep_social: false,
        }
    }

    fn classify(name: &str, action: RuleAction, ctx: &SessionContext) -> Classification {
        let file = file_with(vec![rule(name, RuleType::Process, action)]);
        classify_process(name, &file, &file.profiles[0].rules, ctx)
    }

    #[test]
    fn every_whitelisted_process_should_be_refused_even_when_a_rule_targets_it() {
        for name in whitelist::protected_processes() {
            let got = classify(name, RuleAction::Close, &game_ctx());
            assert_eq!(
                got,
                Classification::Protected {
                    why: ProtectionReason::Whitelist
                },
                "{name}"
            );
        }
    }

    #[test]
    fn every_whitelisted_service_should_be_refused_even_when_a_rule_targets_it() {
        for name in whitelist::protected_services() {
            let file = file_with(vec![rule(name, RuleType::Service, RuleAction::Stop)]);
            let got = classify_service(name, &file, &file.profiles[0].rules, &game_ctx());
            assert_eq!(
                got,
                Classification::Protected {
                    why: ProtectionReason::Whitelist
                },
                "{name}"
            );
        }
    }

    #[test]
    fn unknown_process_should_be_uncertain() {
        let file = file_with(vec![]);
        let got = classify_process("mystery", &file, &[], &game_ctx());
        assert_eq!(got, Classification::Uncertain);
    }

    #[test]
    fn user_list_should_win_over_a_rule() {
        let got = classify("mytool", RuleAction::Close, &game_ctx());
        assert_eq!(
            got,
            Classification::Protected {
                why: ProtectionReason::UserList
            }
        );
    }

    #[test]
    fn steam_should_be_protected_for_a_steam_game() {
        let got = classify("steam", RuleAction::Close, &game_ctx());
        assert_eq!(
            got,
            Classification::Protected {
                why: ProtectionReason::StoreClient
            }
        );
    }

    #[test]
    fn steam_should_be_protected_for_another_store_when_steam_input_is_active() {
        let ctx = SessionContext {
            store: Store::Epic,
            steam_input_active: true,
            ..game_ctx()
        };
        let got = classify("steam", RuleAction::Close, &ctx);
        assert_eq!(
            got,
            Classification::Protected {
                why: ProtectionReason::StoreClient
            }
        );
    }

    #[test]
    fn steam_should_be_closable_for_an_epic_game_without_steam_input() {
        let ctx = SessionContext {
            store: Store::Epic,
            ..game_ctx()
        };
        assert!(matches!(
            classify("steam", RuleAction::Close, &ctx),
            Classification::Actionable { .. }
        ));
    }

    #[test]
    fn software_profile_should_refuse_closing_any_process() {
        let ctx = SessionContext {
            kind: EntryKind::Software,
            ..game_ctx()
        };
        assert!(matches!(
            classify("onedrive", RuleAction::Close, &ctx),
            Classification::Protected { .. }
        ));
    }

    #[test]
    fn software_profile_should_allow_throttling() {
        let ctx = SessionContext {
            kind: EntryKind::Software,
            ..game_ctx()
        };
        assert!(matches!(
            classify("onedrive", RuleAction::Throttle, &ctx),
            Classification::Actionable { .. }
        ));
    }

    #[test]
    fn creative_process_should_stay_protected_in_a_game_session() {
        assert!(matches!(
            classify("blender", RuleAction::Close, &game_ctx()),
            Classification::Protected {
                why: ProtectionReason::Creative
            }
        ));
    }

    #[test]
    fn social_process_should_never_be_suspended() {
        assert!(matches!(
            classify("discord", RuleAction::Suspend, &game_ctx()),
            Classification::Protected { .. }
        ));
    }

    #[test]
    fn other_kind_should_not_touch_anything() {
        let ctx = SessionContext {
            kind: EntryKind::Other,
            ..game_ctx()
        };
        assert!(matches!(
            classify("onedrive", RuleAction::Throttle, &ctx),
            Classification::Protected { .. }
        ));
    }

    #[test]
    fn software_profile_should_not_stop_services_by_default() {
        let ctx = SessionContext {
            kind: EntryKind::Software,
            ..game_ctx()
        };
        let file = file_with(vec![rule("wsearch", RuleType::Service, RuleAction::Stop)]);
        let got = classify_service("wsearch", &file, &file.profiles[0].rules, &ctx);
        assert_eq!(got, Classification::Uncertain);
    }
}
