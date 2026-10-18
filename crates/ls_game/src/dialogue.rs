//! What the characters say (data/text/dialogue.json). Port of Dialogue.h of the C++ engine.
//!
//! A character is a tree of nodes that mirror the helpers of the original scripts:
//!   messages          boxes to show
//!   progress          the last entry whose story progress >= min wins
//!   set_flag_on_talk  the first talk sets the flag and shows `clear`, later talks show `set`
//!   check_flag        shows `set` or `clear` depending on a flag (does not change it)
//!   yesno             a prompt, then `yes` or `no` depending on the player's answer

use bevy::prelude::Resource;
use serde_json::Value;
use std::path::Path;

/// One box of a conversation, and the item received when it is dismissed (-1: none).
#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    pub text: String,
    pub give_item: i32,
}

#[derive(Clone, Debug, Default)]
pub struct Conversation {
    pub pages: Vec<Page>,
    /// the follow-ups if it ends on a yes / no question
    pub yes: Option<Value>,
    pub no: Option<Value>,
    pub asks: bool,
    /// items given as soon as the conversation starts
    pub give_now: Vec<i32>,
    /// script actions (cutscenes) to run once the boxes are dismissed
    pub actions: Vec<i32>,
}

/// The story as a conversation sees it.
pub trait Story {
    fn flag(&self, n: i32) -> bool;
    fn set_flag(&mut self, n: i32);
    fn progress(&self) -> i32;
    fn raise_progress(&mut self, value: i32);
}

#[derive(Default, Resource)]
pub struct Dialogues {
    root: Value,
    /// (flag, progress value): the first flag set gives the story progress
    progress_table: Vec<(i32, i32)>,
}

fn int(v: Option<&Value>) -> i32 {
    v.and_then(|v| v.as_i64()).unwrap_or(0) as i32
}

impl Dialogues {
    pub fn load(path: &Path) -> Dialogues {
        let Ok(text) = std::fs::read_to_string(path) else { return Dialogues::default() };
        let root: Value = serde_json::from_str(&text).unwrap_or_default();
        let progress_table = root.get("progress_flags").and_then(|p| p.as_array()).map(|a| a.iter().filter_map(|p| Some((p.get(0)?.as_i64()? as i32, p.get(1)?.as_i64()? as i32))).collect()).unwrap_or_default();
        Dialogues { root, progress_table }
    }

    pub fn from_json(text: &str) -> Dialogues {
        let root: Value = serde_json::from_str(text).unwrap_or_default();
        let progress_table = root.get("progress_flags").and_then(|p| p.as_array()).map(|a| a.iter().filter_map(|p| Some((p.get(0)?.as_i64()? as i32, p.get(1)?.as_i64()? as i32))).collect()).unwrap_or_default();
        Dialogues { root, progress_table }
    }

    /// Story progress: the value of the first flag of the table that is set; it only ever goes up, like the game's progress byte.
    fn progress(&self, story: &mut dyn Story) -> i32 {
        for &(flag, value) in &self.progress_table {
            if story.flag(flag) {
                story.raise_progress(value);
                break;
            }
        }
        story.progress()
    }

    pub fn has(&self, character: i32) -> bool {
        self.root.get("characters").and_then(|c| c.get(character.to_string())).is_some()
    }

    /// Starts talking to a character; may set story flags (set_flag_on_talk).
    pub fn begin(&self, character: i32, story: &mut dyn Story) -> Conversation {
        let mut c = Conversation::default();
        if let Some(node) = self.root.get("characters").and_then(|c| c.get(character.to_string())) {
            self.resolve(node, story, &mut c, 0);
        }
        c
    }

    /// Continues with a node returned in `yes` / `no`.
    pub fn follow(&self, node: Option<&Value>, story: &mut dyn Story) -> Conversation {
        let mut c = Conversation::default();
        if let Some(n) = node {
            self.resolve(n, story, &mut c, 0);
        }
        c
    }

    fn resolve(&self, n: &Value, story: &mut dyn Story, c: &mut Conversation, depth: usize) {
        if depth > 16 || !n.is_object() {
            return;
        }
        if let Some(messages) = n.get("messages").and_then(|m| m.as_array()) {
            for s in messages {
                if let Some(text) = s.as_str() {
                    // the message is the key of a text of data/i18n (a text that is not a key shows as it is)
                    c.pages.push(Page { text: format_text(&crate::i18n::t(text)), give_item: -1 });
                } else if let Some(g) = s.get("give_item") {
                    // received after the box before it; before any box: right away
                    match c.pages.last_mut() {
                        Some(p) => p.give_item = int(Some(g)),
                        None => c.give_now.push(int(Some(g))),
                    }
                } else if let Some(f) = s.get("set_flag") {
                    story.set_flag(int(Some(f)));
                } else if let Some(a) = s.get("action") {
                    c.actions.push(int(Some(a)));
                }
            }
        } else if let Some(entries) = n.get("progress").and_then(|p| p.as_array()) {
            let progress = self.progress(story);
            let mut chosen = None;
            for e in entries {
                let index = int(e.get("index"));
                let min = int(e.get("min"));
                // only the story progress is tracked
                let value = if index == 0 { progress } else { 0 };
                if value >= min {
                    chosen = e.get("node");
                }
            }
            // nothing matched (should not happen: the first entry has min 0): the first one
            if chosen.is_none() {
                chosen = entries.first().and_then(|e| e.get("node"));
            }
            if let Some(node) = chosen {
                self.resolve(node, story, c, depth + 1);
            }
        } else if let Some(f) = n.get("set_flag_on_talk") {
            let flag = int(Some(f));
            if !story.flag(flag) {
                story.set_flag(flag);
                if let Some(sub) = n.get("clear") {
                    self.resolve(sub, story, c, depth + 1);
                }
            } else if let Some(sub) = n.get("set") {
                self.resolve(sub, story, c, depth + 1);
            }
        } else if let Some(f) = n.get("check_flag") {
            let key = if story.flag(int(Some(f))) { "set" } else { "clear" };
            if let Some(sub) = n.get(key) {
                self.resolve(sub, story, c, depth + 1);
            }
        } else if let Some(yn) = n.get("yesno") {
            if let Some(prompt) = yn.get("prompt") {
                self.resolve(prompt, story, c, depth + 1);
            }
            c.asks = true;
            c.yes = yn.get("yes").cloned();
            c.no = yn.get("no").cloned();
        }
    }
}

/// Control codes seen in dialogue: {57} line break, {5F} the player's name, {5A} a number; {5B} {5E} {58} {62} {59} are box start / end /
/// prompt / pause: nothing to draw.
pub fn format_text(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(a) = rest.find('{') {
        out.push_str(&rest[..a]);
        let Some(b) = rest[a..].find('}') else {
            rest = "";
            break;
        };
        match &rest[a + 1..a + b] {
            "57" => out.push('\n'),
            "5F" => out.push_str("Nigel"),
            "5A" => out.push('?'),
            _ => {}
        }
        rest = &rest[a + b + 1..];
    }
    out.push_str(rest);
    out.chars().map(|c| if c == '\t' { ' ' } else { c }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[derive(Default)]
    struct Fake {
        flags: HashSet<i32>,
        progress: i32,
    }

    impl Story for Fake {
        fn flag(&self, n: i32) -> bool {
            self.flags.contains(&n)
        }
        fn set_flag(&mut self, n: i32) {
            self.flags.insert(n);
        }
        fn progress(&self) -> i32 {
            self.progress
        }
        fn raise_progress(&mut self, v: i32) {
            self.progress = self.progress.max(v);
        }
    }

    const JSON: &str = r#"{
      "progress_flags": [[20, 3], [16, 1]],
      "characters": {
        "1": {"progress": [
          {"index": 0, "min": 0, "node": {"messages": ["{5B}Hello {5F}!{57}Bye{5E}"]}},
          {"index": 0, "min": 1, "node": {"set_flag_on_talk": 90, "clear": {"messages": ["first", {"give_item": 7}, {"set_flag": 91}]}, "set": {"yesno": {"prompt": {"messages": ["again?"]}, "yes": {"messages": ["yes!"]}, "no": {"messages": ["no."]}}}}}
        ]}
      }
    }"#;

    #[test]
    fn what_a_character_says_follows_the_story() {
        let d = Dialogues::from_json(JSON);
        assert!(d.has(1) && !d.has(2));
        let mut s = Fake::default();
        let c = d.begin(1, &mut s);
        assert_eq!(c.pages, vec![Page { text: "Hello Nigel!\nBye".into(), give_item: -1 }]);
        // flag 16 set: the story is at 1, the second node speaks; the first talk gives an item and sets a flag
        s.flags.insert(16);
        let c = d.begin(1, &mut s);
        assert_eq!(s.progress, 1);
        assert_eq!(c.pages, vec![Page { text: "first".into(), give_item: 7 }]);
        assert!(s.flags.contains(&90) && s.flags.contains(&91));
        // the next talk asks a question, and the answers lead on
        let c = d.begin(1, &mut s);
        assert!(c.asks);
        assert_eq!(c.pages[0].text, "again?");
        let yes = d.follow(c.yes.as_ref(), &mut s);
        assert_eq!(yes.pages[0].text, "yes!");
        let no = d.follow(c.no.as_ref(), &mut s);
        assert_eq!(no.pages[0].text, "no.");
    }

    #[test]
    fn the_real_dialogues_load() {
        let data = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/text/dialogue.json");
        let d = Dialogues::load(&data);
        assert!(d.has(0));
        let c = d.begin(0, &mut Fake::default());
        assert!(c.pages[0].text.starts_with("The treasures of King Nole?"), "{:?}", c.pages);
    }
}
