//! Lecteur minimal du format texte VDF de Steam.

use std::collections::BTreeMap;

/// Une valeur VDF.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// Chaîne.
    Text(String),
    /// Bloc de clés (en minuscules).
    Block(BTreeMap<String, Value>),
}

impl Value {
    /// Lit une sous-clé d'un bloc.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Block(map) => map.get(&key.to_lowercase()),
            Value::Text(_) => None,
        }
    }

    /// Lit la valeur comme texte.
    pub fn text(&self) -> Option<&str> {
        match self {
            Value::Text(text) => Some(text),
            Value::Block(_) => None,
        }
    }
}

/// Analyse un document VDF. Renvoie `None` si la structure est cassée.
pub fn parse(input: &str) -> Option<Value> {
    let tokens = tokenize(input)?;
    let mut position = 0;
    let block = parse_block(&tokens, &mut position, 0)?;
    Some(Value::Block(block))
}

#[derive(Debug, PartialEq)]
enum Token {
    Text(String),
    Open,
    Close,
}

fn tokenize(input: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(Token::Open),
            '}' => tokens.push(Token::Close),
            '/' if chars.peek() == Some(&'/') => {
                for next in chars.by_ref() {
                    if next == '\n' {
                        break;
                    }
                }
            }
            '"' => {
                let mut text = String::new();
                loop {
                    match chars.next()? {
                        '"' => break,
                        '\\' => match chars.next()? {
                            'n' => text.push('\n'),
                            't' => text.push('\t'),
                            other => text.push(other),
                        },
                        other => text.push(other),
                    }
                }
                tokens.push(Token::Text(text));
            }
            _ => {}
        }
    }
    Some(tokens)
}

fn parse_block(
    tokens: &[Token],
    position: &mut usize,
    depth: usize,
) -> Option<BTreeMap<String, Value>> {
    let mut map = BTreeMap::new();
    while let Some(token) = tokens.get(*position) {
        *position += 1;
        let key = match token {
            Token::Text(key) => key.to_lowercase(),
            Token::Close if depth > 0 => return Some(map),
            _ => return None,
        };
        let value = match tokens.get(*position)? {
            Token::Text(text) => {
                *position += 1;
                Value::Text(text.clone())
            }
            Token::Open => {
                *position += 1;
                Value::Block(parse_block(tokens, position, depth + 1)?)
            }
            Token::Close => return None,
        };
        map.insert(key, value);
    }
    (depth == 0).then_some(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_should_read_nested_blocks_and_escaped_paths() {
        let doc =
            parse("\"libraryfolders\"\n{\n \"0\"\n {\n \"path\" \"D:\\\\Games\\\\Steam\"\n }\n}\n");
        let path = doc
            .as_ref()
            .and_then(|v| v.get("LibraryFolders"))
            .and_then(|v| v.get("0"))
            .and_then(|v| v.get("path"))
            .and_then(Value::text);
        assert_eq!(path, Some("D:\\Games\\Steam"));
    }

    #[test]
    fn parse_should_reject_an_unclosed_block() {
        assert_eq!(parse("\"a\" { \"b\" \"c\""), None);
    }

    #[test]
    fn parse_should_reject_an_unclosed_string() {
        assert_eq!(parse("\"a"), None);
    }
}
