//! Localisation: every text the player reads lives in YAML dictionaries, `data/i18n/<language>/<domain>.yaml` (dialogs, cutscenes, items, shops,
//! credits, ui, messages...). The Rust code and the Lua scripts only handle keys: `domain.name` (`items.life_stock`, `ui.title.new_game`...),
//! which `t` turns into the text of the chosen language, falling back to English, then to the key itself (a missing text shows its key).
//!
//! A value is a string, a list of strings (the pages of a conversation) or a map with a `text`. `{name}` in a text is replaced by `tf`.
//! The language comes from `[game] language` of landstalker.ini or the LS_LANG variable.

use serde_yaml::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

#[derive(Default)]
pub struct Dictionary {
    /// domain -> key -> value
    domains: HashMap<String, HashMap<String, Value>>,
}

impl Dictionary {
    /// Reads every `*.yaml` of a folder: the file name is the domain.
    pub fn load(dir: &Path) -> Dictionary {
        let mut d = Dictionary::default();
        let Ok(entries) = std::fs::read_dir(dir) else { return d };
        for e in entries.flatten() {
            let path = e.path();
            if path.extension().and_then(|x| x.to_str()) != Some("yaml") {
                continue;
            }
            let Some(domain) = path.file_stem().and_then(|s| s.to_str()).map(str::to_string) else { continue };
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            match serde_yaml::from_str::<Value>(&text) {
                Ok(v @ Value::Mapping(_)) => {
                    let table = d.domains.entry(domain).or_default();
                    flatten("", &v, table);
                }
                Ok(_) => {}
                Err(err) => eprintln!("i18n: {}: {}", path.display(), err),
            }
        }
        d
    }

    pub fn from_yaml(domain: &str, yaml: &str) -> Dictionary {
        let mut d = Dictionary::default();
        if let Ok(v) = serde_yaml::from_str::<Value>(yaml) {
            flatten("", &v, d.domains.entry(domain.to_string()).or_default());
        }
        d
    }

    fn get(&self, key: &str) -> Option<&Value> {
        let (domain, name) = key.split_once('.')?;
        self.domains.get(domain)?.get(name)
    }
}

/// Nested maps become dotted keys (`title: {new_game: X}` gives `title.new_game`); a map with a `text`, a list and a string are values.
fn flatten(prefix: &str, v: &Value, out: &mut HashMap<String, Value>) {
    if let Value::Mapping(m) = v {
        let leaf = m.contains_key("text");
        if !leaf {
            for (k, child) in m {
                let Some(k) = k.as_str().map(str::to_string).or_else(|| k.as_i64().map(|n| n.to_string())) else { continue };
                let key = if prefix.is_empty() { k } else { format!("{}.{}", prefix, k) };
                flatten(&key, child, out);
            }
            return;
        }
    }
    if !prefix.is_empty() {
        out.insert(prefix.to_string(), v.clone());
    }
}

struct Languages {
    language: String,
    current: Dictionary,
    english: Dictionary,
}

static TEXTS: RwLock<Option<Languages>> = RwLock::new(None);

/// Loads the dictionaries of the language (and English as the fallback).
pub fn init(data: &Path, language: &str) {
    let language = if language.is_empty() { "en" } else { language };
    let current = Dictionary::load(&data.join("i18n").join(language));
    let english = if language == "en" { Dictionary::default() } else { Dictionary::load(&data.join("i18n").join("en")) };
    *TEXTS.write().unwrap() = Some(Languages { language: language.to_string(), current, english });
}

/// Starts with dictionaries given as text (tests): `(domain, yaml)`.
pub fn init_from(entries: &[(&str, &str)]) {
    let mut current = Dictionary::default();
    for (domain, yaml) in entries {
        let d = Dictionary::from_yaml(domain, yaml);
        for (k, v) in d.domains {
            current.domains.entry(k).or_default().extend(v);
        }
    }
    *TEXTS.write().unwrap() = Some(Languages { language: "en".into(), current, english: Dictionary::default() });
}

pub fn language() -> String {
    TEXTS.read().unwrap().as_ref().map_or_else(|| "en".to_string(), |l| l.language.clone())
}

fn lookup<T>(key: &str, f: impl Fn(&Value) -> Option<T>) -> Option<T> {
    let guard = TEXTS.read().unwrap();
    let l = guard.as_ref()?;
    l.current.get(key).and_then(&f).or_else(|| l.english.get(key).and_then(&f))
}

fn as_text(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Sequence(items) => items.iter().filter_map(|i| i.as_str()).next().map(str::to_string),
        Value::Mapping(m) => m.get("text").and_then(as_text),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn as_pages(v: &Value) -> Option<Vec<String>> {
    match v {
        Value::String(s) => Some(vec![s.clone()]),
        Value::Sequence(items) => Some(items.iter().filter_map(|i| i.as_str().map(str::to_string)).collect()),
        Value::Mapping(m) => m.get("text").and_then(as_pages),
        _ => None,
    }
}

/// Is there a text for this key (in the language or in English)?
pub fn has(key: &str) -> bool {
    lookup(key, as_text).is_some()
}

/// The text of a key; the key itself when there is none (a missing translation is seen, and a text that is not a key shows as it is).
pub fn t(key: &str) -> String {
    lookup(key, as_text).unwrap_or_else(|| key.to_string())
}

/// The pages of a key (a conversation); a single page for a plain text; the key itself when there is none.
pub fn pages(key: &str) -> Vec<String> {
    lookup(key, as_pages).unwrap_or_else(|| vec![key.to_string()])
}

/// The speaker of a key whose value is a map with `speaker`.
pub fn speaker(key: &str) -> Option<String> {
    lookup(key, |v| v.as_mapping().and_then(|m| m.get("speaker")).and_then(|s| s.as_str()).map(str::to_string))
}

/// `t` with `{name}` replaced by values.
pub fn tf(key: &str, values: &[(&str, String)]) -> String {
    let mut text = t(key);
    for (name, value) in values {
        text = text.replace(&format!("{{{}}}", name), value);
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_give_texts_and_missing_ones_show_themselves() {
        init_from(&[("ui", "title:\n  new_game: NEW GAME\nquit: QUIT\n"), ("messages", "got_gold: \"Got {gold} gold\"\n"), ("dialogs", "talk:\n  - \"First.\"\n  - \"Second.\"\nspoken:\n  speaker: Friday\n  text: Hi\n")]);
        assert_eq!(t("ui.title.new_game"), "NEW GAME");
        assert_eq!(t("ui.quit"), "QUIT");
        assert_eq!(tf("messages.got_gold", &[("gold", "30".into())]), "Got 30 gold");
        assert_eq!(pages("dialogs.talk"), vec!["First.", "Second."]);
        assert_eq!(t("dialogs.spoken"), "Hi");
        assert_eq!(speaker("dialogs.spoken").as_deref(), Some("Friday"));
        assert_eq!(t("ui.unknown"), "ui.unknown");
        assert_eq!(t("a text that is not a key"), "a text that is not a key");
        assert!(has("ui.quit") && !has("ui.nope"));
    }
}
