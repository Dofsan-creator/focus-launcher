//! Liste blanche compilée : aucun fichier de règles ne peut l'affaiblir.

/// Processus jamais touchés.
const PROTECTED_PROCESSES: &[&str] = &[
    // Système Windows.
    "system",
    "idle",
    "registry",
    "memory compression",
    "secure system",
    "smss",
    "csrss",
    "wininit",
    "winlogon",
    "services",
    "lsass",
    "lsaiso",
    "svchost",
    "explorer",
    "dwm",
    "fontdrvhost",
    "sihost",
    "ctfmon",
    "taskhostw",
    "runtimebroker",
    "audiodg",
    "conhost",
    "spoolsv",
    "searchhost",
    "startmenuexperiencehost",
    "shellexperiencehost",
    "textinputhost",
    "applicationframehost",
    "dllhost",
    "wudfhost",
    "consent",
    "taskmgr",
    // Sécurité.
    "msmpeng",
    "nissrv",
    "mpdefendercoreservice",
    "securityhealthservice",
    "securityhealthsystray",
    "smartscreen",
    // Pilotes graphiques, audio et périphériques.
    "nvcontainer",
    "nvdisplay.container",
    "nvsphelper64",
    "rtkauduservice64",
    "atiesrxx",
    "atieclxx",
    "amdrsserv",
    "igfxem",
    "igfxcuiservice",
    // Le launcher.
    "focus-launcher",
    "focus-guardian",
    "msedgewebview2",
    // Anticheats : y toucher peut coûter un bannissement.
    "easyanticheat",
    "easyanticheat_eos",
    "beservice",
    "vgc",
    "vgtray",
    "faceit",
    "faceitclient",
];

/// Services jamais touchés, par nom interne.
const PROTECTED_SERVICES: &[&str] = &[
    "windefend",
    "wdnissvc",
    "securityhealthservice",
    "sense",
    "wscsvc",
    "mpssvc",
    "bfe",
    "rpcss",
    "rpceptmapper",
    "dcomlaunch",
    "lsm",
    "samss",
    "eventlog",
    "plugplay",
    "power",
    "profsvc",
    "schedule",
    "themes",
    "usermanager",
    "audiosrv",
    "audioendpointbuilder",
    "dhcp",
    "dnscache",
    "nsi",
    "netprofm",
    "nlasvc",
    "lanmanworkstation",
    "wlansvc",
    "bthserv",
    "bthavctpsvc",
    "hidserv",
    "cryptsvc",
    "wininit",
    "trustedinstaller",
    "focusguardian",
    "easyanticheat",
    "easyanticheat_eos",
    "beservice",
    "vgc",
    "vgk",
];

/// Clients de boutique, protégés si le jeu choisi en dépend.
const STORE_CLIENTS: &[(&str, crate::model::Store)] = &[
    ("steam", crate::model::Store::Steam),
    ("steamwebhelper", crate::model::Store::Steam),
    ("steamservice", crate::model::Store::Steam),
    ("epicgameslauncher", crate::model::Store::Epic),
    ("epicwebhelper", crate::model::Store::Epic),
    ("galaxyclient", crate::model::Store::Gog),
    ("galaxyclient helper", crate::model::Store::Gog),
    ("eadesktop", crate::model::Store::Ea),
    ("eabackgroundservice", crate::model::Store::Ea),
    ("upc", crate::model::Store::Ubisoft),
    ("ubisoftconnect", crate::model::Store::Ubisoft),
    ("battle.net", crate::model::Store::BattleNet),
    ("agent", crate::model::Store::BattleNet),
    ("gamingservices", crate::model::Store::Xbox),
    ("xboxpcapp", crate::model::Store::Xbox),
];

/// Logiciels de création : ne jamais les fermer ni les suspendre par défaut.
const CREATIVE_PROCESSES: &[&str] = &[
    "blender",
    "maya",
    "3dsmax",
    "cinema4d",
    "houdini",
    "zbrush",
    "substancepainter",
    "unity",
    "unrealeditor",
    "resolve",
    "afterfx",
    "premiere pro",
    "photoshop",
    "illustrator",
    "lightroom",
    "acad",
    "solidworks",
    "sqlservr",
    "postgres",
    "mysqld",
    "devenv",
    "code",
];

/// Applications sociales : jamais suspendues, jamais fermées sans avertissement.
const SOCIAL_PROCESSES: &[&str] = &[
    "discord",
    "obs64",
    "obs32",
    "spotify",
    "teamspeak",
    "ts3client_win64",
    "mumble",
    "streamlabs desktop",
];

/// Met un nom d'image sous la forme de comparaison : minuscules, sans `.exe`.
pub fn normalize(name: &str) -> String {
    let lower = name.trim().to_lowercase();
    match lower.strip_suffix(".exe") {
        Some(stripped) => stripped.to_owned(),
        None => lower,
    }
}

/// Le processus est-il protégé quoi qu'il arrive ?
pub fn is_protected_process(name: &str) -> bool {
    PROTECTED_PROCESSES.contains(&normalize(name).as_str())
}

/// Le service est-il protégé quoi qu'il arrive ?
pub fn is_protected_service(name: &str) -> bool {
    PROTECTED_SERVICES.contains(&name.trim().to_lowercase().as_str())
}

/// Boutique à laquelle appartient ce processus, le cas échéant.
pub fn store_of_process(name: &str) -> Option<crate::model::Store> {
    let key = normalize(name);
    STORE_CLIENTS
        .iter()
        .find(|(client, _)| *client == key)
        .map(|(_, store)| *store)
}

/// Est-ce un logiciel de création ou de calcul ?
pub fn is_creative_process(name: &str) -> bool {
    CREATIVE_PROCESSES.contains(&normalize(name).as_str())
}

/// Est-ce une application sociale connue ?
pub fn is_social_process(name: &str) -> bool {
    SOCIAL_PROCESSES.contains(&normalize(name).as_str())
}

/// Tous les noms de processus protégés, pour les tests et l'écran Analyse.
pub fn protected_processes() -> &'static [&'static str] {
    PROTECTED_PROCESSES
}

/// Tous les noms de services protégés.
pub fn protected_services() -> &'static [&'static str] {
    PROTECTED_SERVICES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_should_ignore_case_and_extension() {
        assert_eq!(normalize("  LSASS.EXE "), "lsass");
    }

    #[test]
    fn lsass_should_be_protected_with_any_spelling() {
        assert!(is_protected_process("LsAsS.exe"));
    }

    #[test]
    fn defender_service_should_be_protected() {
        assert!(is_protected_service("WinDefend"));
    }

    #[test]
    fn unknown_process_should_not_be_protected() {
        assert!(!is_protected_process("onedrive"));
    }
}
