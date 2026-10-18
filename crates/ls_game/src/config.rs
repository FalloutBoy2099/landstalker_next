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

[audio]
enabled = true
master = 1.0
music = 0.6
sfx = 0.8
; The songs: vgm (the game's own sound driver recorded, played by the emulated YM2612 + PSG: data/audio/music_XX.vgm),
; ogg (rendered once from the engine: music_XX.ogg), or auto (vgm when the file exists).
music_format = auto

[input]
up = ArrowUp            ; north-west on the screen
down = ArrowDown        ; south-east
left = ArrowLeft        ; south-west
right = ArrowRight      ; north-east
jump = Space, ControlLeft
action = ShiftLeft, ShiftRight       ; lift, put down, throw, talk, open
attack = X, Enter                    ; the sword
menu = Tab                           ; the item menu (use: action; close: menu or attack)
ekeeke = E                           ; use an EkeEke at once
save = F5                            ; save the game (landstalker.sav)
load = F9                            ; load it
camera = C                           ; the camera follows Nigel / stays where it is
previous_room = O                    ; (playtest) previous room of the index
next_room = P
reload_physics = R                   ; reload data/physics.yaml
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
camera =
previous_room = LeftTrigger
next_room = RightTrigger
reload_physics =
; Quit with View (Select) and Menu (Start) held together (nothing else quits by accident)
quit = Select+Start
; Left stick: pushed further than this (0..1) it walks; the D-pad always walks.
deadzone = 0.5

[game]
; The language of the texts: data/i18n/<language>/ (en; the dictionaries that are missing fall back to English). LS_LANG overrides it.
language = en

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
"#;

#[derive(Resource, Clone)]
pub struct Config {
    pub window: (u32, u32),
    pub audio: AudioSettings,
    pub keys: HashMap<String, Vec<KeyCode>>,
    /// per action: the buttons that act alone, and the chords (all pressed together)
    pub pad_buttons: HashMap<String, Vec<Vec<GamepadButton>>>,
    pub pad_enabled: bool,
    pub pad_deadzone: f32,
    pub playtest_panel: bool,
    pub occlusion_rule: String,
    pub log_script: bool,
    pub sprites_mode: String,
    /// the language of the texts (data/i18n/<language>/)
    pub language: String,
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
        for name in ["up", "down", "left", "right", "jump", "attack", "action", "menu", "ekeeke", "save", "load", "camera", "previous_room", "next_room", "reload_physics", "quit"] {
            let list: Vec<KeyCode> = get(&format!("input.{}", name)).split(',').filter_map(|k| parse_key(k.trim())).collect();
            keys.insert(name.to_string(), list);
        }
        let mut pad_buttons = HashMap::new();
        for name in ["jump", "attack", "action", "menu", "camera", "previous_room", "next_room", "reload_physics", "quit"] {
            // "A, B+Start": A alone, or B and Start together
            let list: Vec<Vec<GamepadButton>> = get(&format!("gamepad.{}", name))
                .split(',')
                .map(|chord| chord.split('+').filter_map(|k| parse_button(k.trim())).collect::<Vec<_>>())
                .filter(|c| !c.is_empty())
                .collect();
            pad_buttons.insert(name.to_string(), list);
        }
        Config {
            window: (get("video.window_width").parse().unwrap_or(1700), get("video.window_height").parse().unwrap_or(960)),
            audio: AudioSettings { enabled: as_bool(&get("audio.enabled")), master: num("audio.master", 1.0).clamp(0.0, 1.0), music: num("audio.music", 0.6).clamp(0.0, 1.0), sfx: num("audio.sfx", 0.8).clamp(0.0, 1.0), music_format: match get("audio.music_format").to_lowercase().as_str() { f @ ("vgm" | "ogg") => f.to_string(), _ => "auto".to_string() } },
            keys,
            pad_buttons,
            pad_enabled: as_bool(&get("gamepad.enabled")),
            pad_deadzone: num("gamepad.deadzone", 0.5).clamp(0.1, 0.95),
            playtest_panel: as_bool(&get("debug.playtest_panel")),
            occlusion_rule: get("debug.occlusion_rule").to_lowercase(),
            log_script: as_bool(&get("debug.log_script")),
            sprites_mode: get("data.sprites").to_lowercase(),
            language: get("game.language").to_lowercase(),
        }
    }

    /// Reads landstalker.ini (writes the default one if there is none).
    pub fn load(project_dir: &Path) -> Config {
        let candidates: [PathBuf; 2] = [PathBuf::from("landstalker.ini"), project_dir.join("landstalker.ini")];
        for p in &candidates {
            if let Ok(text) = std::fs::read_to_string(p) {
                println!("config: {}", p.display());
                return Config::parse(&text);
            }
        }
        let path = &candidates[1];
        match std::fs::write(path, DEFAULT_INI) {
            Ok(()) => println!("config: wrote {}", path.display()),
            Err(e) => println!("config: cannot write {}: {}", path.display(), e),
        }
        Config::default()
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
pub fn read_controls(config: Res<Config>, keys: Res<ButtonInput<KeyCode>>, pads: Query<&Gamepad>, mut ctl: ResMut<Controls>, mut held: Local<[bool; 4]>) {
    let key = |name: &str| -> Btn {
        let list = config.keys.get(name).map(|v| v.as_slice()).unwrap_or(&[]);
        Btn { pressed: list.iter().any(|k| keys.pressed(*k)), just: list.iter().any(|k| keys.just_pressed(*k)) }
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
    let ekeeke = key("ekeeke");
    let save = key("save");
    let load = key("load");
    let mut camera = key("camera");
    let mut quit = key("quit");
    let mut previous_room = key("previous_room");
    let mut next_room = key("next_room");
    let mut reload_physics = key("reload_physics");
    for (b, n) in [(&mut jump, "jump"), (&mut attack, "attack"), (&mut action, "action"), (&mut menu, "menu"), (&mut camera, "camera"), (&mut quit, "quit"), (&mut previous_room, "previous_room"), (&mut next_room, "next_room"), (&mut reload_physics, "reload_physics")] {
        let p = pad_btn(n);
        b.pressed |= p.pressed;
        b.just |= p.just;
    }
    if config.pad_enabled {
        // D-pad and left stick: up = north-west on the screen (like the arrow keys)
        for pad in &pads {
            let (x, y) = (pad.get(GamepadAxis::LeftStickX).unwrap_or(0.0), pad.get(GamepadAxis::LeftStickY).unwrap_or(0.0));
            let dz = config.pad_deadzone;
            // the stick is read as the four directions (no diagonals in this game): the larger axis wins
            let stick = if x.abs() >= y.abs() { [false, false, x < -dz, x > dz] } else { [y > dz, y < -dz, false, false] };
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
    *ctl = Controls { dir, jump, attack, action, menu, ekeeke, save, load, camera, previous_room, next_room, reload_physics, quit };
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::gamepad::GamepadInput;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Config::default()).insert_resource(Controls::default()).insert_resource(ButtonInput::<KeyCode>::default()).add_systems(Update, read_controls);
        app
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
