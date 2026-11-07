//! Construction du plan de session. Fonction pure : c'est aussi le mode simulation.

use crate::engine::{classify_process, classify_service, Classification, SessionContext};
use crate::model::{ProcessInfo, ServiceState, Snapshot};
use crate::rules::{Profile, RuleAction, RuleFile};
use serde::{Deserialize, Serialize};

/// Une action de session. Chacune a son inverse dans [`crate::session`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "step")]
pub enum Step {
    /// Empêcher la veille.
    KeepAwake,
    /// Passer en performances élevées.
    HighPerformancePower,
    /// Mode efficacité et priorité basse.
    Throttle {
        /// Processus visé.
        process: ProcessInfo,
    },
    /// Suspension.
    Suspend {
        /// Processus visé.
        process: ProcessInfo,
    },
    /// Fermeture propre.
    Close {
        /// Processus visé.
        process: ProcessInfo,
        /// Arrêt forcé autorisé après délai.
        allow_force: bool,
        /// Relancer après la session.
        relaunch: bool,
    },
    /// Arrêt temporaire d'un service.
    StopService {
        /// Nom interne.
        name: String,
    },
}

/// Élément qui attend la décision de l'utilisateur.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending {
    /// Nom.
    pub name: String,
    /// Raison donnée par la règle.
    pub reason: String,
}

/// Plan d'une session.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    /// Étapes, dans l'ordre d'application.
    pub steps: Vec<Step>,
    /// Règles `askUser` rencontrées : jamais appliquées sans accord.
    pub ask_user: Vec<Pending>,
    /// Vrai si le profil interdit toute action réelle.
    pub simulation_only: bool,
}

/// Options globales de la session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanOptions {
    /// Empêcher la veille.
    pub keep_awake: bool,
    /// Alimentation hautes performances.
    pub high_performance: bool,
}

/// Construit le plan sans rien exécuter.
pub fn build_plan(
    snapshot: &Snapshot,
    file: &RuleFile,
    profile: &Profile,
    ctx: &SessionContext,
    options: PlanOptions,
) -> Plan {
    let mut plan = Plan {
        simulation_only: profile.simulation_only,
        ..Plan::default()
    };
    if options.keep_awake {
        plan.steps.push(Step::KeepAwake);
    }
    if options.high_performance {
        plan.steps.push(Step::HighPerformancePower);
    }
    for process in &snapshot.processes {
        if let Classification::Actionable { rule } =
            classify_process(&process.name, file, &profile.rules, ctx)
        {
            let process = process.clone();
            match rule.action {
                RuleAction::Throttle => plan.steps.push(Step::Throttle { process }),
                RuleAction::Suspend => plan.steps.push(Step::Suspend { process }),
                RuleAction::Close => plan.steps.push(Step::Close {
                    process,
                    allow_force: rule.allow_force,
                    relaunch: rule.restore_after_game,
                }),
                RuleAction::AskUser => plan.ask_user.push(Pending {
                    name: process.name,
                    reason: rule.reason,
                }),
                // Refusé par la validation du fichier de règles.
                RuleAction::Stop => {}
            }
        }
    }
    for service in &snapshot.services {
        if service.state != ServiceState::Running {
            continue;
        }
        if let Classification::Actionable { rule } =
            classify_service(&service.name, file, &profile.rules, ctx)
        {
            match rule.action {
                RuleAction::Stop => plan.steps.push(Step::StopService {
                    name: service.name.clone(),
                }),
                RuleAction::AskUser => plan.ask_user.push(Pending {
                    name: service.name.clone(),
                    reason: rule.reason,
                }),
                _ => {}
            }
        }
    }
    plan
}

/// Ce plan a-t-il besoin du service élevé ?
pub fn needs_guardian(plan: &Plan) -> bool {
    plan.steps
        .iter()
        .any(|step| matches!(step, Step::StopService { .. }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{EntryKind, ServiceInfo, Store};
    use crate::rules::{Risk, Rule, RuleType};

    fn proc(name: &str, pid: u32) -> ProcessInfo {
        ProcessInfo {
            pid,
            start_time: 1,
            name: name.to_owned(),
            path: None,
            memory: 0,
        }
    }

    fn rule(name: &str, rule_type: RuleType, action: RuleAction) -> Rule {
        Rule {
            rule_type,
            name: name.to_owned(),
            action,
            restore_after_game: true,
            allow_force: false,
            risk: Risk::Low,
            reason: "because".to_owned(),
        }
    }

    fn profile(rules: Vec<Rule>) -> (RuleFile, Profile) {
        let profile = Profile {
            name: "p".to_owned(),
            simulation_only: true,
            rules,
        };
        let file = RuleFile {
            version: 1,
            profiles: vec![profile.clone()],
            never_touch: vec![],
        };
        (file, profile)
    }

    fn ctx() -> SessionContext {
        SessionContext {
            kind: EntryKind::Game,
            store: Store::Steam,
            steam_input_active: false,
            keep_social: false,
        }
    }

    #[test]
    fn plan_should_contain_no_protected_process() {
        let (file, profile) = profile(vec![rule("lsass", RuleType::Process, RuleAction::Close)]);
        let snapshot = Snapshot {
            processes: vec![proc("lsass", 10)],
            services: vec![],
        };
        let plan = build_plan(&snapshot, &file, &profile, &ctx(), PlanOptions::default());
        assert!(plan.steps.is_empty());
    }

    #[test]
    fn plan_should_only_stop_running_services() {
        let (file, profile) = profile(vec![rule("wsearch", RuleType::Service, RuleAction::Stop)]);
        let snapshot = Snapshot {
            processes: vec![],
            services: vec![ServiceInfo {
                name: "WSearch".to_owned(),
                display_name: "Windows Search".to_owned(),
                state: ServiceState::Stopped,
            }],
        };
        let plan = build_plan(&snapshot, &file, &profile, &ctx(), PlanOptions::default());
        assert!(plan.steps.is_empty());
    }

    #[test]
    fn ask_user_rule_should_never_become_a_step() {
        let (file, profile) = profile(vec![rule("chrome", RuleType::Process, RuleAction::AskUser)]);
        let snapshot = Snapshot {
            processes: vec![proc("chrome", 7)],
            services: vec![],
        };
        let plan = build_plan(&snapshot, &file, &profile, &ctx(), PlanOptions::default());
        assert!(plan.steps.is_empty() && plan.ask_user.len() == 1);
    }

    #[test]
    fn plan_without_services_should_not_need_the_guardian() {
        let plan = Plan {
            steps: vec![Step::KeepAwake],
            ..Plan::default()
        };
        assert!(!needs_guardian(&plan));
    }
}
