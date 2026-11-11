//! La politique de sécurité de production doit laisser démarrer l'interface.
//!
//! Trunk charge le WebAssembly avec `fetch`, que `connect-src` contrôle : sans
//! `'self'`, WebView2 bloque le fichier et la fenêtre reste noire. `cargo tauri
//! dev` n'applique pas cette politique, et un navigateur non plus : seule la
//! version publiée le montre, d'où ce test.

use serde_json::Value;
use std::path::PathBuf;

fn csp() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tauri.conf.json");
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let config: Value = serde_json::from_str(&text).unwrap_or_default();
    config["app"]["security"]["csp"]
        .as_str()
        .unwrap_or_default()
        .to_owned()
}

/// Sources d'une directive, vide si elle manque.
fn sources(csp: &str, directive: &str) -> Vec<String> {
    csp.split(';')
        .find_map(|part| {
            let mut words = part.split_whitespace();
            (words.next() == Some(directive)).then(|| words.map(str::to_owned).collect())
        })
        .unwrap_or_default()
}

fn allows(csp: &str, directive: &str, source: &str) -> bool {
    sources(csp, directive).iter().any(|found| found == source)
}

#[test]
fn interface_should_be_allowed_to_fetch_its_webassembly() {
    let csp = csp();
    assert!(allows(&csp, "connect-src", "'self'"), "{csp}");
}

#[test]
fn webassembly_should_be_allowed_to_compile() {
    let csp = csp();
    assert!(allows(&csp, "script-src", "'wasm-unsafe-eval'"), "{csp}");
}

#[test]
fn interface_should_keep_the_ipc_channel() {
    let csp = csp();
    assert!(allows(&csp, "connect-src", "ipc:"), "{csp}");
    assert!(allows(&csp, "connect-src", "http://ipc.localhost"), "{csp}");
}

#[test]
fn policy_should_not_open_scripts_or_connections_to_the_network() {
    let csp = csp();
    for directive in ["default-src", "script-src", "connect-src"] {
        for wide in ["*", "http:", "https:", "'unsafe-eval'"] {
            assert!(!allows(&csp, directive, wide), "{directive} {wide}: {csp}");
        }
    }
}
