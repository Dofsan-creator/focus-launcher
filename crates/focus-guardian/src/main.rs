//! Service Windows `focus-guardian`. `install` et `uninstall` demandent l'élévation.
#![cfg_attr(not(windows), allow(unused))]

#[cfg(windows)]
mod win;

#[cfg(windows)]
fn main() {
    if let Err(err) = win::run() {
        eprintln!("focus-guardian: {err}");
        std::process::exit(1);
    }
}

#[cfg(not(windows))]
fn main() {}
