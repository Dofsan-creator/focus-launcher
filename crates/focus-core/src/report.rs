//! Signalement d'un problème : le texte est nettoyé de tout ce qui nomme
//! l'utilisateur, puis placé dans l'adresse d'une fiche préremplie. Rien n'est
//! envoyé par Focus : c'est l'utilisateur qui relit et valide dans son navigateur.

/// Longueur maximale de l'adresse : au-delà, les navigateurs et GitHub la refusent.
const MAX_URL: usize = 7000;

/// Remplace `needle` sans tenir compte de la casse.
fn replace_ignoring_case(text: &str, needle: &str, with: &str) -> String {
    if needle.is_empty() {
        return text.to_owned();
    }
    let (lower, needle) = (text.to_lowercase(), needle.to_lowercase());
    // Les minuscules peuvent changer la longueur en octets : dans ce cas, on ne
    // touche à rien plutôt que de couper au mauvais endroit.
    if lower.len() != text.len() {
        return text.to_owned();
    }
    let mut out = String::with_capacity(text.len());
    let mut from = 0;
    while let Some(found) = lower[from..].find(&needle) {
        out.push_str(&text[from..from + found]);
        out.push_str(with);
        from += found + needle.len();
    }
    out.push_str(&text[from..]);
    out
}

/// Retire le dossier personnel et le nom du compte Windows d'un texte.
pub fn scrub(text: &str, profile: &str, user: &str) -> String {
    let without_profile = replace_ignoring_case(text, profile, "<dossier personnel>");
    // Un nom très court (« a », « pc ») se retrouverait dans des mots ordinaires.
    if user.chars().count() < 3 {
        return without_profile;
    }
    replace_ignoring_case(&without_profile, user, "<utilisateur>")
}

/// Dernières lignes d'avertissement et d'erreur d'un journal, les plus récentes à la fin.
pub fn problem_lines(log: &str, max: usize) -> Vec<&str> {
    let lines: Vec<&str> = log
        .lines()
        .filter(|line| line.contains(" WARN ") || line.contains("ERROR"))
        .collect();
    lines[lines.len().saturating_sub(max)..].to_vec()
}

/// Encode un caractère pour une adresse web.
fn encode_into(out: &mut String, c: char) {
    if c.is_ascii_alphanumeric() || "-_.~".contains(c) {
        out.push(c);
        return;
    }
    let mut buffer = [0_u8; 4];
    for byte in c.encode_utf8(&mut buffer).bytes() {
        out.push_str(&format!("%{byte:02X}"));
    }
}

/// Encode un texte pour une adresse web.
pub fn encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 3);
    text.chars().for_each(|c| encode_into(&mut out, c));
    out
}

/// Adresse de la fiche GitHub préremplie. Le corps est coupé, à un caractère
/// entier, pour que l'adresse reste acceptée.
pub fn issue_url(repo: &str, title: &str, body: &str) -> String {
    let mut url = format!("https://github.com/{repo}/issues/new?title={}&body=", encode(title));
    for c in body.chars() {
        let before = url.len();
        encode_into(&mut url, c);
        if url.len() > MAX_URL {
            url.truncate(before);
            break;
        }
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrub_should_hide_the_profile_folder_and_the_account_name() {
        let text = "cannot read C:\\Users\\Marie\\AppData\\x and c:\\users\\marie\\y, owner MARIE";
        let clean = scrub(text, "C:\\Users\\Marie", "Marie");
        assert_eq!(clean, "cannot read <dossier personnel>\\AppData\\x and <dossier personnel>\\y, owner <utilisateur>");
    }

    #[test]
    fn scrub_should_leave_ordinary_words_alone_for_a_very_short_name() {
        assert_eq!(scrub("a game ran", "C:\\Users\\a", "a"), "a game ran");
    }

    #[test]
    fn problem_lines_should_keep_only_the_last_warnings_and_errors() {
        let log = "1 INFO ok\n2 WARN slow\n3 ERROR broken\n4 INFO ok\n5 WARN again";
        assert_eq!(problem_lines(log, 2), ["3 ERROR broken", "5 WARN again"]);
        assert!(problem_lines("1 INFO ok", 5).is_empty());
    }

    #[test]
    fn encode_should_escape_everything_but_plain_characters() {
        assert_eq!(encode("a b&c=é\n"), "a%20b%26c%3D%C3%A9%0A");
    }

    #[test]
    fn issue_url_should_stay_short_and_never_cut_a_character() {
        let url = issue_url("owner/repo", "Titre", &"é".repeat(5000));
        assert!(url.starts_with("https://github.com/owner/repo/issues/new?title=Titre&body=%C3%A9"));
        assert!(url.len() <= MAX_URL);
        assert!(url.ends_with("%C3%A9"));
    }
}
