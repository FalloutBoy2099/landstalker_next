//! Reading the YAML data files (data/cutscenes, data/dialogues, data/items.yaml ...) into `serde_json::Value`, the tree the game code walks.

use serde_json::{Map, Value};
use std::path::Path;

/// One YAML file as a tree (null when it is missing or does not parse: a message tells which).
pub fn read(path: &Path) -> Value {
    match std::fs::read_to_string(path) {
        Ok(text) => match serde_yaml::from_str::<serde_yaml::Value>(&text) {
            Ok(v) => convert(v),
            Err(e) => {
                println!("data: {}: {}", path.display(), e);
                Value::Null
            }
        },
        Err(_) => Value::Null,
    }
}

/// The files `<prefix><number>.yaml` of a folder, by number (as a string, like the keys of a JSON object).
pub fn numbered(dir: &Path, prefix: &str) -> Map<String, Value> {
    let mut map = Map::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return map };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let Some(number) = name.strip_prefix(prefix).and_then(|n| n.strip_suffix(".yaml")) else { continue };
        if number.parse::<i64>().is_ok() {
            map.insert(number.to_string(), read(&entry.path()));
        }
    }
    map
}

/// A mapping whose keys are numbers (`7: ...`), as (number, value).
pub fn int_keys(v: &Value) -> Vec<(i32, &Value)> {
    v.as_object().map(|m| m.iter().filter_map(|(k, v)| Some((k.parse::<i32>().ok()?, v))).collect()).unwrap_or_default()
}

/// A YAML tree as a JSON one (the keys of a mapping become strings: `7:` is "7").
pub fn convert(v: serde_yaml::Value) -> Value {
    use serde_yaml::Value as Y;
    match v {
        Y::Null => Value::Null,
        Y::Bool(b) => Value::Bool(b),
        Y::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::from(i)
            } else if let Some(u) = n.as_u64() {
                Value::from(u)
            } else {
                Value::from(n.as_f64().unwrap_or(0.0))
            }
        }
        Y::String(s) => Value::String(s),
        Y::Sequence(list) => Value::Array(list.into_iter().map(convert).collect()),
        Y::Mapping(map) => {
            let mut out = Map::new();
            for (k, v) in map {
                let key = match k {
                    Y::String(s) => s,
                    Y::Number(n) => n.to_string(),
                    Y::Bool(b) => b.to_string(),
                    other => format!("{:?}", other),
                };
                out.insert(key, convert(v));
            }
            Value::Object(out)
        }
        Y::Tagged(t) => convert(t.value),
    }
}
