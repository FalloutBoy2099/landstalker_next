//! landstalker.ini: the keys, the game pad, the sound, the window and the debug options.
//! The file is looked for in the current folder, then in the project folder; if there is none, a commented one is
//! written (see DEFAULT_INI) so that every option can be found and changed in it.

use crate::audio::AudioSettings;
use bevy::input::gamepad::{Gamepad, GamepadAxis, GamepadButton};
use bevy::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const DEFAULT_INI: &str = r#"; landstalker_next: configuration
; Lines starting with ; or # are comments. Values can be changed while the game is not running.
; Keys are the names of Bevy's KeyCode without the "Key" prefix for letters (A, B ... Z), Digit0..Digit9, ArrowUp, Space,
; ShiftLeft, ControlLeft, Enter, Escape, Tab, F1..F12 ... Several keys can be given, separated by commas.
; Game pad buttons: South, East, West, North, LeftTrigger, RightTrigger, LeftTrigger2, RightTrigger2, Select, Start,
; DPadUp, DPadDown, DPadLeft, DPadRight (South = A on an Xbox pad).

[video]
; Size of the game window (the game itself is drawn at 3x the original size).
window_width = 1700
window_height = 960

; Volume of a single sound effect (0 = mute, 1 = normal, up to 2): only the effects written here change, the others are at the normal volume. The name is the one
; of the original's sound (SND_SwordHit: sword_hit) or its number in hexadecimal (52). Example:
; [sfx]
; sword_hit = 0.6
; door = 0.8

[effects]
; The graphic effects over the picture (what they work on and how each kind of place is lit: data/effects.yaml; false turns one off, all off is the
; picture of the original).
; A dark oval on the ground under Nigel and the objects that are above it (jumps, platforms, flying enemies).
object_shadows = true
; The light of the flames: the room is darker (outdoors almost not, in a cave a lot) and the flames light what is near them, and flicker.
flame_light = true
; A soft halo around the flames and the magic eyes (what glows: the bright, saturated pixels, made brighter than white): the camera is HDR and Bevy's
; bloom takes up what is over this brightness.
bloom = true
bloom_threshold = 0.8
; Its strength: 0.12 is a very light halo, just enough to make the flames live.
bloom_intensity = 0.12
; The tone mapper of the camera: none (the colours as they are), reinhard, reinhard_luminance or tony (softer highlights).
tonemapping = none
; The corners of the game area a little darker.
vignette = true
; "Wet dungeon": the shadows pushed to blue, the highlights kept warm.
color_grade = true
; How strong light, bloom, vignette and grade are, for all the kinds of place at once (1 = as data/effects.yaml says, 0 = none, 2 = twice).
intensity = 1.0

[audio]
enabled = true
master = 1.0
music = 0.6
sfx = 0.8
; The songs: vgm (the game's own sound driver recorded, played by the emulated YM2612 + PSG: data/audio/music_XX.vgm),
; ogg (rendered once from the engine: music_XX.ogg), or auto (vgm when the file exists).
music_format = auto
; The fanfare of a chest is played over the music of the room, which goes on (false: the music stops for the fanfare, like the original's).
chest_keeps_music = true

[input]
; auto: the letters are the keys that are labelled so on your keyboard (AZERTY, QWERTZ...), the other keys (digits, arrows, Shift...) are by place;
; physical: every key is the one at that place on a QWERTY keyboard (A is the key to the right of Tab, whatever it says).
layout = auto
up = ArrowUp            ; north-west on the screen
down = ArrowDown        ; south-east
left = ArrowLeft        ; south-west
right = ArrowRight      ; north-east
jump = Space, ControlLeft
action = ShiftLeft, ShiftRight       ; lift, put down, throw, talk, open
attack = X, Enter                    ; the sword
menu = Tab                           ; the inventory (use: action); the same key again closes it
equipment = 5                        ; the equipment page (sword, armour, boots, ring); the same key again closes it; quit closes either
quick_previous = 1                   ; the quick item (the one shown at the top of the screen; the EkeEke, then the cures you carry): the previous one
quick_use = 2                        ; use it at once
quick_next = 3                       ; the next one
use_key = 4                          ; use a Key in front of a locked door (if you carry one)
ekeeke = R                           ; use an EkeEke at once: press it, then press it again to confirm
save = F5                            ; save the game (landstalker.sav)
load = F9                            ; load it
camera = C                           ; the camera follows Nigel / stays where it is
previous_room = O                    ; (playtest) previous room of the index
next_room = P
reload_physics = F6                  ; reload data/physics.yaml
start_tas = F7                       ; (debug) start recording a whole route (the TAS: room after room, with the flags)
stop_tas = F8                        ; (debug) stop it: the route is written (data/tas)
quit = Escape

[gamepad]
; Made for an Xbox Series X|S controller (also fine for any XInput / SDL game controller):
;   left stick or D-pad ... walk (the four directions of the isometric view)
;   A (South) ............. jump        X (West) ........... the sword
;   B (East) .............. action: lift, put down, throw, open a chest, talk
;   Y (North) ............. item menu (up / down: D-pad, B: use, Y or X: close)
;   Y (North) ............. camera       LB / RB ............ previous / next room (playtest)
;   Menu (Start) .......... quit (when pressed together with View)
enabled = true
jump = South, RightTrigger2
attack = West
action = East
menu = North
quick_previous =
quick_use =
quick_next =
use_key =
equipment = LeftTrigger2
start_tas =
stop_tas =
camera =
previous_room = LeftTrigger
next_room = RightTrigger
reload_physics =
; Quit with View (Select) and Menu (Start) held together (nothing else quits by accident)
quit = Select+Start
; Left stick: pushed further than this (0..1) it walks; the D-pad always walks.
deadzone = 0.5

[game]
; The language of the texts: data/i18n/<language>/ (en, fr, es, it, de, jp; a dictionary that is missing falls back to English). `auto` takes the language of the
; system (English when it is not one of these); the title screen changes it and writes the choice here. LS_LANG overrides it.
language = auto
; Direction keys: false = one fixed direction per key (up north-west, right north-east, down south-east, left south-west). true = like the original's pad: two
; keys together are a diagonal of the screen, one key goes along the axis Nigel already faces (and the stick has diagonals).
original_directions = false
; Help to leave a room: pushed against a wall with a door a little to the side (two cells at most), Nigel slides along the wall to it. The original's help is
; narrower (it only pushes him when one corner of him is free).
assist_exits = true

[ui]
; Size (in pixels) of the text of the messages.
font_size = 24
; The boxes of text (the dialogues, their questions, the story's and the scenes': all the same window, the size of the original's story boxes):
; the size of their text (pixels); where they sit: bottom, middle or top; their distance from the edge of the picture; their width (pixels, 0: the
; width of the original's box).
dialog_font_size = 40
dialog_position = bottom
dialog_margin = 24
dialog_width = 0
; The room between the text of the boxes (the dialogues, their questions and the story's: all the same window) and their border (pixels), and their
; background colour: "red, green, blue" or "red, green, blue, opacity" (each 0 to 255; no # here, it starts a comment).
dialog_padding = 20
dialog_background = 0, 0, 0
; The background of the item and equipment menus: "red, green, blue, opacity" (0 to 255: the lower the opacity, the more of the game shows through).
menu_background = 14, 16, 44, 235
; The bar at the top (hearts, EkeEke, gold): its scale (1.0 = normal) and the size of its text (pixels).
hud_scale = 1.0
hud_font_size = 20

[data]
; Where the sprites are read from: auto (a loose file of data/sprites if there is one, else the sprite's own .lsa archive),
; files (loose YAML + PNG only), or lsa (the archives only: data/sprites/sprite_NNN.lsa, data/sprites/entities/entity_NNN.lsa).
; Make the archives with: lsctl lsa-pack-sprites data/sprites
sprites = auto

[debug]
; The playtest panel on the right of the window (room switch, notes, captures).
playtest_panel = true
; Occlusion rule: original (the console's), cpp, objects, mine.
occlusion_rule = original
; Print the flags and sounds the scripts set.
log_script = false
; The room the game starts in, straight away (no title screen): to test (Massan is 592). 0: the title screen and the story's start.
start_room = 0
; Every blow of the sword kills what it hits (also in the debug panel).
one_hit_kill = false
; The TAS (data/tas, recorded with the start_tas / stop_tas keys): `route` plays the recorded route from the room the game starts in, room after room; `off` plays nothing.
tas = off
; The game starts in this room with the flags, the items and the health its TAS started with: to begin at a level of the adventure (0: not used).
tas_start_room = 0
"#;

#[derive(Resource, Clone)]
pub struct Config {
    pub window: (u32, u32),
    pub effects: EffectsSettings,
    pub audio: AudioSettings,
    pub keys: HashMap<String, Vec<KeyCode>>,
    /// per action: the buttons that act alone, and the chords (all pressed together)
    pub pad_buttons: HashMap<String, Vec<Vec<GamepadButton>>>,
    pub pad_enabled: bool,
    pub assist_exits: bool,
    /// the letters of the ini are the keys labelled so (the layout of the keyboard), not the places of a QWERTY keyboard
    pub logical_letters: bool,
    pub pad_deadzone: f32,
    pub playtest_panel: bool,
    pub occlusion_rule: String,
    pub log_script: bool,
    pub start_room: i32,
    pub one_hit_kill: bool,
    pub tas_mode: String,
    pub tas_start_room: i32,
    pub sprites_mode: String,
    /// the direction keys read like the original's pad (two keys = a diagonal of the screen, one key = along the axis he faces) instead of one fixed direction each
    pub original_directions: bool,
    /// the language of the texts (data/i18n/<language>/)
    pub language: String,
    pub ui: UiSettings,
    /// the landstalker.ini that was read (where the language chosen on the title screen is written)
    pub ini_path: Option<PathBuf>,
}

/// The graphic effects (the [effects] section of landstalker.ini).
#[derive(Clone, Debug)]
pub struct EffectsSettings {
    pub object_shadows: bool,
    pub flame_light: bool,
    pub bloom: bool,
    pub bloom_threshold: f32,
    /// the strength of Bevy's bloom (0.15: a very light halo)
    pub bloom_intensity: f32,
    pub vignette: bool,
    pub color_grade: bool,
    pub intensity: f32,
    /// the tone mapper of the camera: none, reinhard, reinhard_luminance or tony
    pub tonemapping: String,
}

/// The sizes and places of the texts and boxes (the [ui] section of landstalker.ini).
#[derive(Clone, Debug)]
pub struct UiSettings {
    pub font_size: f32,
    pub dialog_font_size: f32,
    pub story_font_scale: f32,
    /// "bottom", "middle" or "top"
    pub dialog_position: String,
    pub dialog_margin: f32,
    pub dialog_width: f32,
    pub dialog_padding: f32,
    /// red, green, blue, opacity
    pub dialog_background: [u8; 4],
    pub menu_background: [u8; 4],
    pub hud_scale: f32,
    pub hud_font_size: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config::parse(DEFAULT_INI)
    }
}

/// `name = value` pairs of an ini file, as "section.name".
fn parse_ini(text: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let mut section = String::new();
    for raw in text.lines() {
        // comments: a ; or # (outside the value of a key name, there are no ; in values)
        let line = raw.split(|c| c == ';' || c == '#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if let Some(s) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = s.trim().to_lowercase();
        } else if let Some((k, v)) = line.split_once('=') {
            out.insert(format!("{}.{}", section, k.trim().to_lowercase()), v.trim().to_string());
        }
    }
    out
}

/// "red, green, blue" or "red, green, blue, opacity" (0 to 255 each); `default` when it does not read.
fn parse_color(v: &str, default: [u8; 4]) -> [u8; 4] {
    let n: Vec<u8> = v.split(',').filter_map(|p| p.trim().parse::<u8>().ok()).collect();
    match n.as_slice() {
        [r, g, b] => [*r, *g, *b, 255],
        [r, g, b, a] => [*r, *g, *b, *a],
        _ => default,
    }
}

impl UiSettings {
    /// Where line `line` of a story box (of `lines` lines) is written: the padding from the border, and the lines closer together when the box is too small.
    pub fn story_text_at(&self, rect: bevy_egui::egui::Rect, k: f32, line: usize, lines: usize) -> bevy_egui::egui::Pos2 {
        let pitch = (16.0 * k).min((rect.height() - 2.0 * self.dialog_padding) / lines.max(3) as f32);
        bevy_egui::egui::pos2(rect.left() + self.dialog_padding, rect.top() + self.dialog_padding + pitch * line as f32)
    }
}

fn as_bool(v: &str) -> bool {
    matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on")
}

impl Config {
    pub fn parse(text: &str) -> Config {
        let defaults = if text == DEFAULT_INI { HashMap::new() } else { parse_ini(DEFAULT_INI) };
        let mut ini = defaults;
        ini.extend(parse_ini(text));
        let get = |k: &str| ini.get(k).cloned().unwrap_or_default();
        let num = |k: &str, d: f32| get(k).parse::<f32>().unwrap_or(d);
        let mut keys = HashMap::new();
        for name in ["up", "down", "left", "right", "jump", "attack", "action", "menu", "quick_previous", "quick_use", "quick_next", "use_key", "ekeeke", "equipment", "start_tas", "stop_tas", "save", "load", "camera", "previous_room", "next_room", "reload_physics", "quit"] {
            let list: Vec<KeyCode> = get(&format!("input.{}", name)).split(',').filter_map(|k| parse_key(k.trim())).collect();
            keys.insert(name.to_string(), list);
        }
        let mut pad_buttons = HashMap::new();
        for name in ["jump", "attack", "action", "menu", "quick_previous", "quick_use", "quick_next", "use_key", "equipment", "start_tas", "stop_tas", "camera", "previous_room", "next_room", "reload_physics", "quit"] {
            // "A, B+Start": A alone, or B and Start together
            let list: Vec<Vec<GamepadButton>> = get(&format!("gamepad.{}", name))
                .split(',')
                .map(|chord| chord.split('+').filter_map(|k| parse_button(k.trim())).collect::<Vec<_>>())
                .filter(|c| !c.is_empty())
                .collect();
            pad_buttons.insert(name.to_string(), list);
        }
        Config {
            effects: EffectsSettings {
                object_shadows: as_bool(&get("effects.object_shadows")),
                flame_light: as_bool(&get("effects.flame_light")),
                bloom: as_bool(&get("effects.bloom")),
                bloom_threshold: num("effects.bloom_threshold", 0.8).clamp(0.2, 2.0),
                bloom_intensity: num("effects.bloom_intensity", 0.12).clamp(0.0, 1.0),
                vignette: as_bool(&get("effects.vignette")),
                color_grade: as_bool(&get("effects.color_grade")),
                intensity: num("effects.intensity", 1.0).clamp(0.0, 2.0),
                tonemapping: get("effects.tonemapping").to_lowercase(),
            },
            window: (get("video.window_width").parse().unwrap_or(1700), get("video.window_height").parse().unwrap_or(960)),
            audio: AudioSettings { enabled: as_bool(&get("audio.enabled")), master: num("audio.master", 1.0).clamp(0.0, 1.0), music: num("audio.music", 0.6).clamp(0.0, 1.0), sfx: num("audio.sfx", 0.8).clamp(0.0, 1.0), music_format: match get("audio.music_format").to_lowercase().as_str() { f @ ("vgm" | "ogg") => f.to_string(), _ => "auto".to_string() }, chest_keeps_music: !matches!(get("audio.chest_keeps_music").to_lowercase().as_str(), "false" | "0" | "no" | "off"), sfx_volumes: ini.iter().filter_map(|(k, v)| Some((crate::audio::sfx_id(k.strip_prefix("sfx.")?)?, v.trim().parse::<f32>().ok()?.clamp(0.0, 2.0)))).collect() },
            keys,
            pad_buttons,
            pad_enabled: as_bool(&get("gamepad.enabled")),
            assist_exits: !matches!(get("game.assist_exits").to_lowercase().as_str(), "false" | "0" | "no" | "off"),
            logical_letters: get("input.layout").to_lowercase() != "physical" && std::env::var("LS_KEYBOARD").map_or(true, |v| v != "physical"),
            pad_deadzone: num("gamepad.deadzone", 0.5).clamp(0.1, 0.95),
            playtest_panel: as_bool(&get("debug.playtest_panel")),
            occlusion_rule: get("debug.occlusion_rule").to_lowercase(),
            log_script: as_bool(&get("debug.log_script")),
            start_room: get("debug.start_room").trim().parse().unwrap_or(0),
            one_hit_kill: as_bool(&get("debug.one_hit_kill")),
            tas_mode: get("debug.tas").to_lowercase(),
            tas_start_room: get("debug.tas_start_room").trim().parse().unwrap_or(0),
            sprites_mode: get("data.sprites").to_lowercase(),
            original_directions: get("game.original_directions").to_lowercase() == "true",
            language: resolve_language(&get("game.language")),
            ini_path: None,
            ui: UiSettings {
                font_size: num("ui.font_size", 24.0).clamp(8.0, 96.0),
                dialog_font_size: num("ui.dialog_font_size", 40.0).clamp(8.0, 120.0),
                story_font_scale: num("ui.story_font_scale", 1.0).clamp(0.5, 3.0),
                dialog_position: match get("ui.dialog_position").to_lowercase().as_str() { p @ ("middle" | "top") => p.to_string(), _ => "bottom".to_string() },
                dialog_margin: num("ui.dialog_margin", 24.0).clamp(0.0, 600.0),
                dialog_width: num("ui.dialog_width", 0.0).clamp(0.0, 3000.0),
                dialog_padding: num("ui.dialog_padding", 20.0).clamp(0.0, 100.0),
                dialog_background: parse_color(&get("ui.dialog_background"), [0, 0, 0, 255]),
                menu_background: parse_color(&get("ui.menu_background"), [14, 16, 44, 235]),
                hud_scale: num("ui.hud_scale", 1.0).clamp(0.5, 4.0),
                hud_font_size: num("ui.hud_font_size", 20.0).clamp(8.0, 96.0),
            },
        }
    }

    /// Reads landstalker.ini (writes the default one if there is none).
    pub fn load(project_dir: &Path) -> Config {
        let candidates: [PathBuf; 2] = [PathBuf::from("landstalker.ini"), project_dir.join("landstalker.ini")];
        for p in &candidates {
            if let Ok(text) = std::fs::read_to_string(p) {
                println!("config: {}", p.display());
                let mut c = Config::parse(&text);
                c.ini_path = Some(p.clone());
                return c;
            }
        }
        let path = &candidates[1];
        match std::fs::write(path, DEFAULT_INI) {
            Ok(()) => println!("config: wrote {}", path.display()),
            Err(e) => println!("config: cannot write {}: {}", path.display(), e),
        }
        let mut c = Config::default();
        c.ini_path = Some(path.clone());
        c
    }

    /// Writes the language chosen on the title screen into landstalker.ini (the `language =` line of [game]).
    /// Writes the switches and strengths of [effects] into landstalker.ini (the lines that are there are changed, the others are added under [effects]).
    pub fn save_effects(&self) -> bool {
        let Some(path) = &self.ini_path else { return false };
        let Ok(text) = std::fs::read_to_string(path) else { return false };
        let fx = &self.effects;
        let pairs: Vec<(&str, String)> = vec![
            ("object_shadows", fx.object_shadows.to_string()),
            ("flame_light", fx.flame_light.to_string()),
            ("bloom", fx.bloom.to_string()),
            ("bloom_threshold", format!("{:.2}", fx.bloom_threshold)),
            ("bloom_intensity", format!("{:.2}", fx.bloom_intensity)),
            ("tonemapping", fx.tonemapping.clone()),
            ("vignette", fx.vignette.to_string()),
            ("color_grade", fx.color_grade.to_string()),
            ("intensity", format!("{:.2}", fx.intensity)),
        ];
        let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
        let Some(start) = lines.iter().position(|l| l.trim().eq_ignore_ascii_case("[effects]")) else { return false };
        let end = lines.iter().skip(start + 1).position(|l| l.trim().starts_with('[')).map_or(lines.len(), |i| start + 1 + i);
        for (key, value) in pairs {
            let found = (start + 1..end.min(lines.len())).find(|&i| {
                let t = lines[i].trim_start();
                !t.starts_with(';') && t.contains('=') && t.split('=').next().map(|k| k.trim() == key).unwrap_or(false)
            });
            match found {
                Some(i) => lines[i] = format!("{} = {}", key, value),
                None => lines.insert(start + 1, format!("{} = {}", key, value)),
            }
        }
        std::fs::write(path, lines.join("\n") + "\n").is_ok()
    }

    pub fn save_language(&self, language: &str) {
        let Some(path) = &self.ini_path else { return };
        let Ok(text) = std::fs::read_to_string(path) else { return };
        let mut done = false;
        let mut lines: Vec<String> = text
            .lines()
            .map(|l| {
                if !done && l.trim_start().starts_with("language") && l.contains('=') && !l.trim_start().starts_with(';') {
                    done = true;
                    format!("language = {}", language)
                } else {
                    l.to_string()
                }
            })
            .collect();
        if !done {
            // an ini written before the option existed: the line goes under [game] (which is added when there is none)
            match lines.iter().position(|l| l.trim().eq_ignore_ascii_case("[game]")) {
                Some(i) => lines.insert(i + 1, format!("language = {}", language)),
                None => lines.extend([String::new(), "[game]".to_string(), format!("language = {}", language)]),
            }
        }
        let _ = std::fs::write(path, lines.join("\n") + "\n");
    }
}

/// The languages of the game: the code of data/i18n/<code>/ and its name, in that language.
pub const LANGUAGES: [(&str, &str); 6] = [("en", "English"), ("fr", "Français"), ("es", "Español"), ("it", "Italiano"), ("de", "Deutsch"), ("jp", "Nihongo")];

/// The language of the system as one of ours (en when it is none of them).
pub fn system_language() -> String {
    let locale = sys_locale::get_locale().unwrap_or_default().to_lowercase();
    let code = locale.split(['-', '_']).next().unwrap_or("");
    match code {
        "fr" | "es" | "it" | "de" => code.to_string(),
        "ja" | "jp" => "jp".to_string(),
        _ => "en".to_string(),
    }
}

/// `auto` (or nothing) is the language of the system; otherwise the code written in the ini.
fn resolve_language(value: &str) -> String {
    let v = value.trim().to_lowercase();
    if v.is_empty() || v == "auto" {
        system_language()
    } else {
        v
    }
}

pub fn parse_key(name: &str) -> Option<KeyCode> {
    use KeyCode::*;
    let n = name.trim();
    if n.len() == 1 {
        let c = n.chars().next()?.to_ascii_uppercase();
        let letters = [KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ];
        let digits = [Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9];
        return match c {
            'A'..='Z' => Some(letters[(c as u8 - b'A') as usize]),
            '0'..='9' => Some(digits[(c as u8 - b'0') as usize]),
            _ => None,
        };
    }
    Some(match n.to_lowercase().as_str() {
        "arrowup" | "up" => ArrowUp,
        "arrowdown" | "down" => ArrowDown,
        "arrowleft" | "left" => ArrowLeft,
        "arrowright" | "right" => ArrowRight,
        "space" => Space,
        "enter" | "return" => Enter,
        "escape" | "esc" => Escape,
        "tab" => Tab,
        "backspace" => Backspace,
        "shiftleft" | "lshift" => ShiftLeft,
        "shiftright" | "rshift" => ShiftRight,
        "controlleft" | "lctrl" | "ctrl" => ControlLeft,
        "controlright" | "rctrl" => ControlRight,
        "altleft" | "lalt" | "alt" => AltLeft,
        "altright" | "ralt" => AltRight,
        "comma" => Comma,
        "period" => Period,
        "f1" => F1,
        "f2" => F2,
        "f3" => F3,
        "f4" => F4,
        "f5" => F5,
        "f6" => F6,
        "f7" => F7,
        "f8" => F8,
        "f9" => F9,
        "f10" => F10,
        "f11" => F11,
        "f12" => F12,
        "pageup" => PageUp,
        "pagedown" => PageDown,
        "home" => Home,
        "end" => End,
        "insert" => Insert,
        "delete" => Delete,
        s if s.starts_with("key") && s.len() == 4 => return parse_key(&s[3..]),
        s if s.starts_with("digit") && s.len() == 6 => return parse_key(&s[5..]),
        _ => return None,
    })
}

pub fn parse_button(name: &str) -> Option<GamepadButton> {
    Some(match name.trim().to_lowercase().as_str() {
        "south" | "a" => GamepadButton::South,
        "east" | "b" => GamepadButton::East,
        "west" | "x" => GamepadButton::West,
        "north" | "y" => GamepadButton::North,
        "lefttrigger" | "lb" => GamepadButton::LeftTrigger,
        "righttrigger" | "rb" => GamepadButton::RightTrigger,
        "lefttrigger2" | "lt" => GamepadButton::LeftTrigger2,
        "righttrigger2" | "rt" => GamepadButton::RightTrigger2,
        "select" | "back" => GamepadButton::Select,
        "start" => GamepadButton::Start,
        "dpadup" => GamepadButton::DPadUp,
        "dpaddown" => GamepadButton::DPadDown,
        "dpadleft" => GamepadButton::DPadLeft,
        "dpadright" => GamepadButton::DPadRight,
        _ => return None,
    })
}

/// A button as the game reads it: held now, pressed this frame.
#[derive(Clone, Copy, Default)]
pub struct Btn {
    pub pressed: bool,
    pub just: bool,
}

/// What the player asks, from the keyboard and the game pad together (see landstalker.ini).
#[derive(Resource, Default)]
pub struct Controls {
    /// up (NW), down (SE), left (SW), right (NE)
    pub dir: [Btn; 4],
    pub jump: Btn,
    pub attack: Btn,
    pub action: Btn,
    pub menu: Btn,
    pub quick_previous: Btn,
    pub quick_use: Btn,
    pub quick_next: Btn,
    pub use_key: Btn,
    pub equipment: Btn,
    pub start_tas: Btn,
    pub stop_tas: Btn,
    pub ekeeke: Btn,
    pub save: Btn,
    pub load: Btn,
    pub camera: Btn,
    pub previous_room: Btn,
    pub next_room: Btn,
    pub reload_physics: Btn,
    pub quit: Btn,
}

/// Reads the keyboard and the game pads into `Controls`.
/// The letter a key of the ini stands for.
pub(crate) fn letter_of(k: KeyCode) -> Option<char> {
    use KeyCode::*;
    const LETTERS: [KeyCode; 26] = [KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ];
    LETTERS.iter().position(|l| *l == k).map(|i| (b'a' + i as u8) as char)
}

pub fn read_controls(config: Res<Config>, keys: Res<ButtonInput<KeyCode>>, logical: Res<ButtonInput<bevy::input::keyboard::Key>>, pads: Query<&Gamepad>, mut ctl: ResMut<Controls>, mut held: Local<[bool; 4]>) {
    let key = |name: &str| -> Btn {
        let list = config.keys.get(name).map(|v| v.as_slice()).unwrap_or(&[]);
        // a letter is the key labelled so (the layout of the keyboard) unless the ini says `layout = physical`
        let typed = |c: char, f: &dyn Fn(&bevy::input::keyboard::Key) -> bool| f(&bevy::input::keyboard::Key::Character(c.to_string().into())) || f(&bevy::input::keyboard::Key::Character(c.to_ascii_uppercase().to_string().into()));
        let down = |k: &KeyCode| match letter_of(*k).filter(|_| config.logical_letters) {
            Some(c) => typed(c, &|key| logical.pressed(key.clone())),
            None => keys.pressed(*k),
        };
        let just = |k: &KeyCode| match letter_of(*k).filter(|_| config.logical_letters) {
            Some(c) => typed(c, &|key| logical.just_pressed(key.clone())),
            None => keys.just_pressed(*k),
        };
        Btn { pressed: list.iter().any(down), just: list.iter().any(just) }
    };
    let mut dir = [key("up"), key("down"), key("left"), key("right")];
    let pad_btn = |name: &str| -> Btn {
        if !config.pad_enabled {
            return Btn::default();
        }
        let list = config.pad_buttons.get(name).map(|v| v.as_slice()).unwrap_or(&[]);
        // a chord is pressed when all its buttons are; it was just pressed when the last one went down this frame
        let pressed = |p: &Gamepad, c: &Vec<GamepadButton>| c.iter().all(|b| p.pressed(*b));
        let just = |p: &Gamepad, c: &Vec<GamepadButton>| pressed(p, c) && c.iter().any(|b| p.just_pressed(*b));
        Btn { pressed: pads.iter().any(|p| list.iter().any(|c| pressed(p, c))), just: pads.iter().any(|p| list.iter().any(|c| just(p, c))) }
    };
    let mut jump = key("jump");
    let mut attack = key("attack");
    let mut action = key("action");
    let mut menu = key("menu");
    let mut quick_previous = key("quick_previous");
    let mut quick_use = key("quick_use");
    let mut quick_next = key("quick_next");
    let mut use_key = key("use_key");
    let mut equipment = key("equipment");
    let mut start_tas = key("start_tas");
    let mut stop_tas = key("stop_tas");
    let ekeeke = key("ekeeke");
    let save = key("save");
    let load = key("load");
    let mut camera = key("camera");
    let mut quit = key("quit");
    let mut previous_room = key("previous_room");
    let mut next_room = key("next_room");
    let mut reload_physics = key("reload_physics");
    for (b, n) in [(&mut jump, "jump"), (&mut attack, "attack"), (&mut action, "action"), (&mut menu, "menu"), (&mut quick_previous, "quick_previous"), (&mut quick_use, "quick_use"), (&mut quick_next, "quick_next"), (&mut use_key, "use_key"), (&mut equipment, "equipment"), (&mut start_tas, "start_tas"), (&mut stop_tas, "stop_tas"), (&mut camera, "camera"), (&mut quit, "quit"), (&mut previous_room, "previous_room"), (&mut next_room, "next_room"), (&mut reload_physics, "reload_physics")] {
        let p = pad_btn(n);
        b.pressed |= p.pressed;
        b.just |= p.just;
    }
    if config.pad_enabled {
        // D-pad and left stick: up = north-west on the screen (like the arrow keys)
        for pad in &pads {
            let (x, y) = (pad.get(GamepadAxis::LeftStickX).unwrap_or(0.0), pad.get(GamepadAxis::LeftStickY).unwrap_or(0.0));
            let dz = config.pad_deadzone;
            // the stick is read like the original's pad: the four directions, and the diagonals between them (two keys together are the screen diagonals the
            // game walks along); an axis counts when it is at least 0.4 of the other
            let (ax, ay) = (x.abs(), y.abs());
            let stick = if config.original_directions {
                [y > dz && ay >= 0.4 * ax, y < -dz && ay >= 0.4 * ax, x < -dz && ax >= 0.4 * ay, x > dz && ax >= 0.4 * ay]
            } else if ax >= ay {
                // (no diagonals: the larger axis wins)
                [false, false, x < -dz, x > dz]
            } else {
                [y > dz, y < -dz, false, false]
            };
            let dpad = [GamepadButton::DPadUp, GamepadButton::DPadDown, GamepadButton::DPadLeft, GamepadButton::DPadRight];
            for i in 0..4 {
                if stick[i] || pad.pressed(dpad[i]) {
                    dir[i].pressed = true;
                }
            }
        }
    }
    for i in 0..4 {
        // "just" for keys comes from the keyboard; for the pad it is the change of the held state
        dir[i].just |= dir[i].pressed && !held[i];
        held[i] = dir[i].pressed;
    }
    *ctl = Controls { dir, jump, attack, action, menu, quick_previous, quick_use, quick_next, use_key, equipment, start_tas, stop_tas, ekeeke, save, load, camera, previous_room, next_room, reload_physics, quit };
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::gamepad::GamepadInput;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Config::default()).insert_resource(Controls::default()).insert_resource(ButtonInput::<KeyCode>::default()).insert_resource(ButtonInput::<bevy::input::keyboard::Key>::default()).add_systems(Update, read_controls);
        app
    }

    #[test]
    fn the_letters_are_the_keys_labelled_so_unless_the_layout_is_physical() {
        use bevy::input::keyboard::Key;
        // attack = X: the key that says X (an AZERTY keyboard has it where a QWERTY one has X too, but Q / A / Z / W / M move)
        let mut app = app();
        app.world_mut().resource_mut::<ButtonInput<Key>>().press(Key::Character("x".into()));
        app.update();
        assert!(app.world().resource::<Controls>().attack.pressed);
        let mut app = super::tests::app();
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::KeyX);
        app.update();
        assert!(app.world().resource::<Controls>().attack.pressed == false, "the place of the key does not matter in `auto`");
        let mut app = super::tests::app();
        app.world_mut().resource_mut::<Config>().logical_letters = false;
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::KeyX);
        app.update();
        assert!(app.world().resource::<Controls>().attack.pressed, "`physical`: the place is what counts");
    }

    #[test]
    fn only_the_sound_effects_that_are_written_change_their_volume() {
        let c = Config::parse("[sfx]
sword_hit = 0.5
0x44 = 0
nope = 3
Door = 3
");
        assert_eq!(c.audio.sfx_volumes.get(&0x52), Some(&0.5));
        assert_eq!(c.audio.sfx_volumes.get(&0x44), Some(&0.0));
        assert_eq!(c.audio.sfx_volumes.get(&0x55), Some(&2.0), "up to twice the normal volume");
        assert_eq!(c.audio.sfx_volumes.len(), 3);
        assert!(Config::default().audio.sfx_volumes.is_empty());
    }

    #[test]
    fn the_ini_gives_the_keys_and_the_xbox_buttons() {
        let c = Config::default();
        assert_eq!(c.keys["jump"], vec![KeyCode::Space, KeyCode::ControlLeft]);
        assert_eq!(c.keys["previous_room"], vec![KeyCode::KeyO]);
        assert_eq!(c.pad_buttons["jump"], vec![vec![GamepadButton::South], vec![GamepadButton::RightTrigger2]]);
        assert_eq!(c.pad_buttons["attack"], vec![vec![GamepadButton::West]]);
        assert_eq!(c.pad_buttons["action"], vec![vec![GamepadButton::East]]);
        assert_eq!(c.pad_buttons["quit"], vec![vec![GamepadButton::Select, GamepadButton::Start]]);
        assert!(c.pad_buttons["reload_physics"].is_empty());
        // a changed key wins, the others keep their default
        let c = Config::parse("[input]\njump = Z\n");
        assert_eq!(c.keys["jump"], vec![KeyCode::KeyZ]);
        assert_eq!(c.keys["action"], vec![KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    }

    #[test]
    fn the_pad_walks_jumps_and_acts() {
        let mut app = app();
        let pad = app.world_mut().spawn(Gamepad::default()).id();
        // left stick to the left = south-west; A = jump
        {
            let mut e = app.world_mut().entity_mut(pad);
            let mut g = e.get_mut::<Gamepad>().unwrap();
            g.analog_mut().set(GamepadInput::Axis(GamepadAxis::LeftStickX), -0.9);
            g.digital_mut().press(GamepadButton::South);
        }
        app.update();
        let ctl = app.world().resource::<Controls>();
        assert!(ctl.dir[2].pressed && ctl.dir[2].just && !ctl.dir[0].pressed && !ctl.dir[3].pressed);
        assert!(ctl.jump.pressed && ctl.jump.just);
        // next frame: held, no longer "just"
        {
            let mut e = app.world_mut().entity_mut(pad);
            e.get_mut::<Gamepad>().unwrap().digital_mut().clear();
        }
        app.update();
        let ctl = app.world().resource::<Controls>();
        assert!(ctl.dir[2].pressed && !ctl.dir[2].just);
        assert!(ctl.jump.pressed && !ctl.jump.just);
        // a stick pushed up = north-west; a small push does nothing
        {
            let mut e = app.world_mut().entity_mut(pad);
            let mut g = e.get_mut::<Gamepad>().unwrap();
            g.analog_mut().set(GamepadInput::Axis(GamepadAxis::LeftStickX), 0.2);
            g.analog_mut().set(GamepadInput::Axis(GamepadAxis::LeftStickY), 0.9);
        }
        app.update();
        let ctl = app.world().resource::<Controls>();
        assert!(ctl.dir[0].pressed && !ctl.dir[2].pressed);
        // X = the sword; View alone does not quit, View + Menu together do
        {
            let mut e = app.world_mut().entity_mut(pad);
            let mut g = e.get_mut::<Gamepad>().unwrap();
            g.analog_mut().set(GamepadInput::Axis(GamepadAxis::LeftStickY), 0.0);
            g.digital_mut().clear();
            g.digital_mut().press(GamepadButton::West);
            g.digital_mut().press(GamepadButton::Select);
        }
        app.update();
        let ctl = app.world().resource::<Controls>();
        assert!(ctl.attack.just && !ctl.quit.pressed);
        {
            let mut e = app.world_mut().entity_mut(pad);
            let mut g = e.get_mut::<Gamepad>().unwrap();
            g.digital_mut().clear();
            g.digital_mut().press(GamepadButton::Start);
        }
        app.update();
        assert!(app.world().resource::<Controls>().quit.just);
    }

    #[test]
    fn the_keyboard_still_works() {
        let mut app = app();
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::ArrowRight);
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::ShiftLeft);
        app.update();
        let ctl = app.world().resource::<Controls>();
        assert!(ctl.dir[3].pressed && ctl.dir[3].just && ctl.action.just);
    }
}
