//! Exécution et restauration d'un plan, à travers le trait [`SystemOps`].
//!
//! L'ordre est la garantie : capturer, journaliser, agir, journaliser. Une
//! coupure à n'importe quel instant laisse donc un journal suffisant.

use crate::journal::{Journal, JournalEntry, JournalError, JournalStore, Original, StepState};
use crate::model::ProcessInfo;
use crate::plan::{Plan, Step};
use thiserror::Error;

/// Erreur d'une opération système.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{0}")]
pub struct OpError(pub String);

/// Opérations réelles (ou fausses, dans les tests) sur le système.
pub trait SystemOps {
    /// Le processus est-il toujours celui observé ? Protège de la réutilisation des pid.
    fn is_same_process(&self, process: &ProcessInfo) -> bool;
    /// Empêche la veille.
    fn keep_awake(&mut self) -> Result<(), OpError>;
    /// Autorise de nouveau la veille.
    fn allow_sleep(&mut self) -> Result<(), OpError>;
    /// Renvoie le schéma d'alimentation actif.
    fn active_power_scheme(&self) -> Result<String, OpError>;
    /// Active le schéma performances élevées.
    fn set_high_performance(&mut self) -> Result<(), OpError>;
    /// Active un schéma.
    fn set_power_scheme(&mut self, guid: &str) -> Result<(), OpError>;
    /// Renvoie la classe de priorité.
    fn process_priority(&self, process: &ProcessInfo) -> Result<u32, OpError>;
    /// Applique le mode efficacité et la priorité basse.
    fn throttle(&mut self, process: &ProcessInfo) -> Result<(), OpError>;
    /// Annule le mode efficacité et remet la priorité.
    fn unthrottle(&mut self, process: &ProcessInfo, class: u32) -> Result<(), OpError>;
    /// Suspend.
    fn suspend(&mut self, process: &ProcessInfo) -> Result<(), OpError>;
    /// Reprend.
    fn resume(&mut self, process: &ProcessInfo) -> Result<(), OpError>;
    /// Ligne de commande de relance.
    fn relaunch_info(&self, process: &ProcessInfo) -> Result<(String, Vec<String>), OpError>;
    /// Ferme proprement, puis de force si autorisé.
    fn close(&mut self, process: &ProcessInfo, allow_force: bool) -> Result<(), OpError>;
    /// Relance un programme.
    fn relaunch(&mut self, path: &str, args: &[String]) -> Result<(), OpError>;
    /// Arrête un service.
    fn stop_service(&mut self, name: &str) -> Result<(), OpError>;
    /// Démarre un service.
    fn start_service(&mut self, name: &str) -> Result<(), OpError>;
}

/// Erreur de session.
#[derive(Debug, Error)]
pub enum SessionError {
    /// Le journal n'a pas pu être écrit : on n'agit pas sans lui.
    #[error(transparent)]
    Journal(#[from] JournalError),
    /// Le plan interdit toute action réelle.
    #[error("simulation-only plan cannot be applied")]
    SimulationOnly,
}

/// Bilan d'une application ou d'une restauration.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    /// Nombre d'étapes réussies.
    pub done: usize,
    /// Échecs, en anglais pour les journaux.
    pub failures: Vec<String>,
}

/// Applique un plan. Le journal complet existe avant la première action.
/// Un échec d'étape n'arrête pas la session : le jeu reste jouable.
pub fn apply_plan(
    plan: &Plan,
    entry_id: &str,
    started_at: i64,
    ops: &mut dyn SystemOps,
    store: &JournalStore,
) -> Result<Report, SessionError> {
    if plan.simulation_only {
        return Err(SessionError::SimulationOnly);
    }
    let mut journal = Journal {
        version: 1,
        entry_id: entry_id.to_owned(),
        started_at,
        entries: plan
            .steps
            .iter()
            .map(|step| JournalEntry {
                step: step.clone(),
                state: StepState::Planned,
                original: Original::None,
            })
            .collect(),
    };
    store.write(&journal)?;

    let mut report = Report::default();
    for index in 0..journal.entries.len() {
        let step = journal.entries[index].step.clone();
        match capture(&step, ops) {
            Ok(original) => {
                journal.entries[index].original = original;
                journal.entries[index].state = StepState::Captured;
                store.write(&journal)?;
            }
            Err(err) => {
                report.failures.push(format!("capture failed: {err}"));
                continue;
            }
        }
        match apply(&step, ops) {
            Ok(()) => {
                journal.entries[index].state = StepState::Applied;
                store.write(&journal)?;
                report.done += 1;
            }
            Err(err) => report.failures.push(format!("apply failed: {err}")),
        }
    }
    Ok(report)
}

/// Restaure dans l'ordre inverse. `Captured` est traité comme `Applied` : on ne
/// sait pas si l'action a eu lieu, et chaque inverse est sans effet si elle n'a
/// pas eu lieu. Le journal n'est effacé que si tout a réussi.
pub fn restore(ops: &mut dyn SystemOps, store: &JournalStore) -> Result<Report, SessionError> {
    let Some(mut journal) = store.read()? else {
        return Ok(Report::default());
    };
    let mut report = Report::default();
    for index in (0..journal.entries.len()).rev() {
        let entry = journal.entries[index].clone();
        if !matches!(entry.state, StepState::Captured | StepState::Applied) {
            continue;
        }
        match revert(&entry.step, &entry.original, ops) {
            Ok(()) => {
                journal.entries[index].state = StepState::Reverted;
                store.write(&journal)?;
                report.done += 1;
            }
            Err(err) => report.failures.push(format!("revert failed: {err}")),
        }
    }
    if report.failures.is_empty() {
        store.clear()?;
    }
    Ok(report)
}

/// Récupération au démarrage : un journal présent signale une session
/// interrompue. Un journal illisible est mis de côté, jamais fatal.
pub fn recover(ops: &mut dyn SystemOps, store: &JournalStore) -> Result<Report, SessionError> {
    match store.read() {
        Ok(_) => restore(ops, store),
        Err(JournalError::Corrupt(err)) => {
            store.quarantine()?;
            Ok(Report {
                done: 0,
                failures: vec![format!("corrupt journal quarantined: {err}")],
            })
        }
        Err(err) => Err(err.into()),
    }
}

fn capture(step: &Step, ops: &mut dyn SystemOps) -> Result<Original, OpError> {
    Ok(match step {
        Step::HighPerformancePower => Original::PowerScheme {
            guid: ops.active_power_scheme()?,
        },
        Step::Throttle { process } => Original::Priority {
            class: ops.process_priority(process)?,
        },
        Step::Close {
            process,
            relaunch: true,
            ..
        } => {
            let (path, args) = ops.relaunch_info(process)?;
            Original::Relaunch { path, args }
        }
        Step::StopService { .. } => Original::ServiceRunning,
        Step::KeepAwake | Step::Suspend { .. } | Step::Close { .. } => Original::None,
    })
}

fn apply(step: &Step, ops: &mut dyn SystemOps) -> Result<(), OpError> {
    match step {
        Step::KeepAwake => ops.keep_awake(),
        Step::HighPerformancePower => ops.set_high_performance(),
        Step::Throttle { process } => {
            ensure_same(process, ops)?;
            ops.throttle(process)
        }
        Step::Suspend { process } => {
            ensure_same(process, ops)?;
            ops.suspend(process)
        }
        Step::Close {
            process,
            allow_force,
            ..
        } => {
            ensure_same(process, ops)?;
            ops.close(process, *allow_force)
        }
        Step::StopService { name } => ops.stop_service(name),
    }
}

fn ensure_same(process: &ProcessInfo, ops: &dyn SystemOps) -> Result<(), OpError> {
    if ops.is_same_process(process) {
        Ok(())
    } else {
        Err(OpError(format!("process {} changed identity", process.pid)))
    }
}

fn revert(step: &Step, original: &Original, ops: &mut dyn SystemOps) -> Result<(), OpError> {
    match (step, original) {
        (Step::KeepAwake, _) => ops.allow_sleep(),
        (Step::HighPerformancePower, Original::PowerScheme { guid }) => ops.set_power_scheme(guid),
        // Après un redémarrage, le pid désigne un autre programme : ne rien faire.
        (Step::Throttle { process }, Original::Priority { class }) => {
            if ops.is_same_process(process) {
                ops.unthrottle(process, *class)
            } else {
                Ok(())
            }
        }
        (Step::Suspend { process }, _) => {
            if ops.is_same_process(process) {
                ops.resume(process)
            } else {
                Ok(())
            }
        }
        (Step::Close { .. }, Original::Relaunch { path, args }) => ops.relaunch(path, args),
        (Step::StopService { name }, Original::ServiceRunning) => ops.start_service(name),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[derive(Default)]
    struct Fake {
        log: Vec<String>,
        dead_pids: HashSet<u32>,
        fail: Option<&'static str>,
    }

    impl Fake {
        fn record(&mut self, what: String) -> Result<(), OpError> {
            if self.fail.is_some_and(|needle| what.contains(needle)) {
                return Err(OpError(what));
            }
            self.log.push(what);
            Ok(())
        }
    }

    impl SystemOps for Fake {
        fn is_same_process(&self, process: &ProcessInfo) -> bool {
            !self.dead_pids.contains(&process.pid)
        }
        fn keep_awake(&mut self) -> Result<(), OpError> {
            self.record("awake".into())
        }
        fn allow_sleep(&mut self) -> Result<(), OpError> {
            self.record("sleep".into())
        }
        fn active_power_scheme(&self) -> Result<String, OpError> {
            Ok("balanced".into())
        }
        fn set_high_performance(&mut self) -> Result<(), OpError> {
            self.record("power:high".into())
        }
        fn set_power_scheme(&mut self, guid: &str) -> Result<(), OpError> {
            self.record(format!("power:{guid}"))
        }
        fn process_priority(&self, _: &ProcessInfo) -> Result<u32, OpError> {
            Ok(32)
        }
        fn throttle(&mut self, p: &ProcessInfo) -> Result<(), OpError> {
            self.record(format!("throttle:{}", p.pid))
        }
        fn unthrottle(&mut self, p: &ProcessInfo, class: u32) -> Result<(), OpError> {
            self.record(format!("unthrottle:{}:{class}", p.pid))
        }
        fn suspend(&mut self, p: &ProcessInfo) -> Result<(), OpError> {
            self.record(format!("suspend:{}", p.pid))
        }
        fn resume(&mut self, p: &ProcessInfo) -> Result<(), OpError> {
            self.record(format!("resume:{}", p.pid))
        }
        fn relaunch_info(&self, _: &ProcessInfo) -> Result<(String, Vec<String>), OpError> {
            Ok(("C:/app.exe".into(), vec!["--bg".into()]))
        }
        fn close(&mut self, p: &ProcessInfo, _: bool) -> Result<(), OpError> {
            self.record(format!("close:{}", p.pid))
        }
        fn relaunch(&mut self, path: &str, _: &[String]) -> Result<(), OpError> {
            self.record(format!("relaunch:{path}"))
        }
        fn stop_service(&mut self, name: &str) -> Result<(), OpError> {
            self.record(format!("stop:{name}"))
        }
        fn start_service(&mut self, name: &str) -> Result<(), OpError> {
            self.record(format!("start:{name}"))
        }
    }

    fn proc(pid: u32) -> ProcessInfo {
        ProcessInfo {
            pid,
            start_time: 1,
            name: "x".into(),
            path: None,
            memory: 0,
        }
    }

    fn full_plan() -> Plan {
        Plan {
            steps: vec![
                Step::KeepAwake,
                Step::HighPerformancePower,
                Step::Throttle { process: proc(1) },
                Step::Close {
                    process: proc(2),
                    allow_force: false,
                    relaunch: true,
                },
                Step::StopService {
                    name: "WSearch".into(),
                },
            ],
            ..Plan::default()
        }
    }

    fn store(dir: &tempfile::TempDir) -> JournalStore {
        JournalStore::new(dir.path().join("session.lock.json"))
    }

    #[test]
    fn restore_should_run_in_reverse_order() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        let mut ops = Fake::default();
        apply_plan(&full_plan(), "e", 0, &mut ops, &store)?;
        ops.log.clear();
        restore(&mut ops, &store)?;
        assert_eq!(
            ops.log,
            [
                "start:WSearch",
                "relaunch:C:/app.exe",
                "unthrottle:1:32",
                "power:balanced",
                "sleep"
            ]
        );
        Ok(())
    }

    #[test]
    fn full_restore_should_delete_the_journal() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        let mut ops = Fake::default();
        apply_plan(&full_plan(), "e", 0, &mut ops, &store)?;
        restore(&mut ops, &store)?;
        assert_eq!(store.read()?, None);
        Ok(())
    }

    #[test]
    fn journal_should_exist_before_the_first_action() -> Result<(), Box<dyn std::error::Error>> {
        struct Checking {
            inner: Fake,
            path: std::path::PathBuf,
            seen: bool,
        }
        impl SystemOps for Checking {
            fn is_same_process(&self, p: &ProcessInfo) -> bool {
                self.inner.is_same_process(p)
            }
            fn keep_awake(&mut self) -> Result<(), OpError> {
                self.seen = self.path.exists();
                Ok(())
            }
            fn allow_sleep(&mut self) -> Result<(), OpError> {
                Ok(())
            }
            fn active_power_scheme(&self) -> Result<String, OpError> {
                Ok(String::new())
            }
            fn set_high_performance(&mut self) -> Result<(), OpError> {
                Ok(())
            }
            fn set_power_scheme(&mut self, _: &str) -> Result<(), OpError> {
                Ok(())
            }
            fn process_priority(&self, _: &ProcessInfo) -> Result<u32, OpError> {
                Ok(0)
            }
            fn throttle(&mut self, _: &ProcessInfo) -> Result<(), OpError> {
                Ok(())
            }
            fn unthrottle(&mut self, _: &ProcessInfo, _: u32) -> Result<(), OpError> {
                Ok(())
            }
            fn suspend(&mut self, _: &ProcessInfo) -> Result<(), OpError> {
                Ok(())
            }
            fn resume(&mut self, _: &ProcessInfo) -> Result<(), OpError> {
                Ok(())
            }
            fn relaunch_info(&self, _: &ProcessInfo) -> Result<(String, Vec<String>), OpError> {
                Ok((String::new(), vec![]))
            }
            fn close(&mut self, _: &ProcessInfo, _: bool) -> Result<(), OpError> {
                Ok(())
            }
            fn relaunch(&mut self, _: &str, _: &[String]) -> Result<(), OpError> {
                Ok(())
            }
            fn stop_service(&mut self, _: &str) -> Result<(), OpError> {
                Ok(())
            }
            fn start_service(&mut self, _: &str) -> Result<(), OpError> {
                Ok(())
            }
        }
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        let mut ops = Checking {
            inner: Fake::default(),
            path: store.path().to_owned(),
            seen: false,
        };
        apply_plan(&full_plan(), "e", 0, &mut ops, &store)?;
        assert!(ops.seen);
        Ok(())
    }

    #[test]
    fn simulation_only_plan_should_be_refused() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        let plan = Plan {
            simulation_only: true,
            ..full_plan()
        };
        let got = apply_plan(&plan, "e", 0, &mut Fake::default(), &store);
        assert!(matches!(got, Err(SessionError::SimulationOnly)));
        Ok(())
    }

    #[test]
    fn crash_recovery_should_restore_from_the_journal() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        apply_plan(&full_plan(), "e", 0, &mut Fake::default(), &store)?;
        // Nouveau processus, sans mémoire de la session précédente.
        let mut fresh = Fake::default();
        recover(&mut fresh, &store)?;
        assert!(fresh.log.contains(&"start:WSearch".to_string()));
        Ok(())
    }

    #[test]
    fn crash_between_capture_and_action_should_still_restore(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        store.write(&Journal {
            version: 1,
            entry_id: "e".into(),
            started_at: 0,
            entries: vec![JournalEntry {
                step: Step::StopService {
                    name: "WSearch".into(),
                },
                state: StepState::Captured,
                original: Original::ServiceRunning,
            }],
        })?;
        let mut ops = Fake::default();
        recover(&mut ops, &store)?;
        assert_eq!(ops.log, ["start:WSearch"]);
        Ok(())
    }

    #[test]
    fn planned_step_should_not_be_reverted() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        store.write(&Journal {
            version: 1,
            entry_id: "e".into(),
            started_at: 0,
            entries: vec![JournalEntry {
                step: Step::StopService {
                    name: "WSearch".into(),
                },
                state: StepState::Planned,
                original: Original::None,
            }],
        })?;
        let mut ops = Fake::default();
        recover(&mut ops, &store)?;
        assert!(ops.log.is_empty());
        Ok(())
    }

    #[test]
    fn reused_pid_should_not_be_touched_on_recovery() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        apply_plan(&full_plan(), "e", 0, &mut Fake::default(), &store)?;
        let mut rebooted = Fake::default();
        rebooted.dead_pids.insert(1);
        recover(&mut rebooted, &store)?;
        assert!(!rebooted
            .log
            .iter()
            .any(|line| line.starts_with("unthrottle")));
        Ok(())
    }

    #[test]
    fn failed_restore_should_keep_the_journal_and_retry_only_what_failed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        let mut ops = Fake::default();
        apply_plan(&full_plan(), "e", 0, &mut ops, &store)?;
        ops.log.clear();
        ops.fail = Some("start:WSearch");
        let report = restore(&mut ops, &store)?;
        assert_eq!(report.failures.len(), 1);
        ops.fail = None;
        ops.log.clear();
        restore(&mut ops, &store)?;
        assert_eq!(ops.log, ["start:WSearch"]);
        assert_eq!(store.read()?, None);
        Ok(())
    }

    #[test]
    fn failed_step_should_not_stop_the_session() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        let mut ops = Fake {
            fail: Some("power:high"),
            ..Fake::default()
        };
        let report = apply_plan(&full_plan(), "e", 0, &mut ops, &store)?;
        assert_eq!((report.done, report.failures.len()), (4, 1));
        Ok(())
    }

    #[test]
    fn corrupt_journal_should_be_quarantined_on_recovery() -> Result<(), Box<dyn std::error::Error>>
    {
        let dir = tempfile::tempdir()?;
        let store = store(&dir);
        std::fs::write(store.path(), "garbage")?;
        let report = recover(&mut Fake::default(), &store)?;
        assert_eq!((report.failures.len(), store.read()?), (1, None));
        Ok(())
    }
}
