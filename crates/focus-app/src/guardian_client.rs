//! Client du service `focus-guardian`. La connexion vit le temps de la session :
//! sa perte, même par plantage, déclenche la restauration côté service.

#![cfg(windows)]

use focus_core::session::OpError;
use focus_guardian::{Request, Response, PIPE_NAME};
use focus_sys::ops::ServiceControl;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// Connexion au gardien.
pub struct GuardianClient {
    writer: File,
    reader: BufReader<File>,
}

impl GuardianClient {
    /// Se connecte. `None` si le service n'est pas installé ou pas démarré.
    pub fn connect() -> Option<Self> {
        let file = File::options().read(true).write(true).open(PIPE_NAME).ok()?;
        let reader = BufReader::new(file.try_clone().ok()?);
        Some(Self { writer: file, reader })
    }

    fn send(&mut self, request: &Request) -> Result<(), OpError> {
        let line = serde_json::to_string(request).map_err(|e| OpError(e.to_string()))?;
        writeln!(self.writer, "{line}").map_err(|e| OpError(e.to_string()))?;
        let mut answer = String::new();
        self.reader.read_line(&mut answer).map_err(|e| OpError(e.to_string()))?;
        match serde_json::from_str(&answer).map_err(|e| OpError(e.to_string()))? {
            Response::Ok => Ok(()),
            Response::Error { message } => Err(OpError(message)),
        }
    }
}

impl ServiceControl for GuardianClient {
    fn stop(&mut self, name: &str) -> Result<(), OpError> {
        self.send(&Request::Stop { name: name.to_owned() })
    }

    fn start(&mut self, name: &str) -> Result<(), OpError> {
        self.send(&Request::Start { name: name.to_owned() })
    }
}
