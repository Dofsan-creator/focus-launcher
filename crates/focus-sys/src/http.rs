//! Téléchargement d'images par la pile réseau de Windows, vers des hôtes fixés.
//! Les appels bloquent : ils tournent dans un fil de fond, jamais dans la fenêtre.

use thiserror::Error;
use windows::core::HSTRING;
use windows::Foundation::Uri;
use windows::Storage::Streams::DataReader;
use windows::Web::Http::HttpClient;

/// Échec d'un téléchargement : l'entrée garde son icône.
#[derive(Debug, Error)]
pub enum HttpError {
    /// Adresse hors de la liste autorisée, ou non chiffrée.
    #[error("address not allowed: {0}")]
    NotAllowed(String),
    /// Réponse trop grande pour une image.
    #[error("response too large")]
    TooLarge,
    /// Erreur de Windows.
    #[error(transparent)]
    Windows(#[from] windows::core::Error),
}

/// L'adresse est-elle en HTTPS vers un hôte de la liste ?
pub fn is_allowed(scheme: &str, host: &str, allowed_hosts: &[&str]) -> bool {
    scheme.eq_ignore_ascii_case("https")
        && allowed_hosts
            .iter()
            .any(|allowed| host.eq_ignore_ascii_case(allowed))
}

/// Télécharge une ressource en mémoire.
pub fn download(url: &str, allowed_hosts: &[&str], max_bytes: u32) -> Result<Vec<u8>, HttpError> {
    let uri = Uri::CreateUri(&HSTRING::from(url))?;
    let scheme = uri.SchemeName()?.to_string_lossy();
    let host = uri.Host()?.to_string_lossy();
    if !is_allowed(&scheme, &host, allowed_hosts) {
        return Err(HttpError::NotAllowed(host));
    }
    let client = HttpClient::new()?;
    let response = client.GetAsync(&uri)?.get()?;
    response.EnsureSuccessStatusCode()?;
    let buffer = response.Content()?.ReadAsBufferAsync()?.get()?;
    let length = buffer.Length()?;
    if length > max_bytes {
        return Err(HttpError::TooLarge);
    }
    let reader = DataReader::FromBuffer(&buffer)?;
    let mut bytes = vec![0_u8; length as usize];
    reader.ReadBytes(&mut bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOSTS: &[&str] = &["cdn1.epicgames.com"];

    #[test]
    fn only_https_to_listed_hosts_should_pass() {
        assert!(is_allowed("https", "cdn1.epicgames.com", HOSTS));
        assert!(is_allowed("HTTPS", "CDN1.EpicGames.com", HOSTS));
        assert!(!is_allowed("http", "cdn1.epicgames.com", HOSTS));
        assert!(!is_allowed("https", "evil.example", HOSTS));
        assert!(!is_allowed("https", "cdn1.epicgames.com.evil.example", HOSTS));
    }

    #[test]
    fn unlisted_address_should_be_refused_before_any_request() {
        let result = download("https://evil.example/x.jpg", HOSTS, 1024);
        assert!(matches!(result, Err(HttpError::NotAllowed(_))));
    }
}
