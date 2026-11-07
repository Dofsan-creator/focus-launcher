//! Conversion vers les chaînes UTF-16 de Windows.

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// Encode en UTF-16 terminé par zéro.
pub fn wide(text: impl AsRef<OsStr>) -> Vec<u16> {
    text.as_ref()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Décode un tampon UTF-16 terminé par zéro.
pub fn from_wide(buffer: &[u16]) -> String {
    let end = buffer.iter().position(|c| *c == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..end])
}
