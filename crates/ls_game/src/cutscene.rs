//! Cutscenes: what the story does by itself. A cutscene action (data/cutscenes/story/*.yaml, one file per area of the game) is a list of operations: texts in the
//! dialogue box, story flags, sounds, tile swaps, characters appearing and disappearing, going to another room behind a fade, effects
//! (fades, flashes), Friday the fairy, the player's controls taken away... Port of RoomView::TriggerAction / ContinueCutscene /
//! StartFx / UpdateFaint / ApplyPendingWarp and of Cutscenes.h of the C++ engine.
//!
//! The scripts of the characters ask for an action (`cutscene(e, n)` in Lua), so do the conversations; `Cutscene` runs the
//! operations until one has to wait (a text, a fade, a pause) and goes on when it is over.

use crate::audio::{SfxQueue, Sound};
use crate::combat::Combat;
use crate::dialogue::format_text;
use crate::game::GameState;
use crate::scripting::Scripting;
use crate::shops::Dialogue;
use crate::{Arrival, GameFrozen, Player, RoomRequest, RoomRes, SwapState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::track::Track;
use serde_json::Value;
use std::collections::VecDeque;
use std::path::Path;

// -------------------------------------------------------------------------------------------------------------------
// the data
// -------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Text,
    Flag,
    Sound,
    Swap,
    If,
    Warp,
    Lock,
    Hostile,
    Visible,
    Hide,
    Block,
    Fx,
    Wait,
    Friday,
    Credits,
    IntroString,
}

#[derive(Clone, Debug)]
pub struct Op {
    pub kind: Kind,
    /// a cutscene text, a flag, a sound, a swap trigger, a room, an entity (-1: the one that started the action)
    pub value: i32,
    pub set: bool,
    /// IF: the condition is the answer to the yes / no question just asked (else a story flag)
    pub on_answer: bool,
    /// TEXT: frames (1/60 s) before the box goes away by itself
    pub wait: i32,
    pub x: i32,
    pub y: i32,
    /// WARP: arrival direction (0 NE, 1 SE, 2 SW, 3 NW), -1 unchanged
    pub dir: i32,
    pub cells: Vec<(i32, i32)>,
    pub name: String,
    pub then_ops: Vec<Op>,
    pub else_ops: Vec<Op>,
}

impl Op {
    fn new(kind: Kind) -> Op {
        Op { kind, value: 0, set: true, on_answer: false, wait: 0, x: 0, y: 0, dir: -1, cells: Vec::new(), name: String::new(), then_ops: Vec::new(), else_ops: Vec::new() }
    }
}

#[derive(Resource, Default)]
pub struct Cutscenes {
    root: Value,
    intro_strings: Value,
    credits: Vec<CreditLine>,
}

/// A line of the credits (data/i18n/<language>/credit.txt).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreditLine {
    /// a group title, underlined: `[CAST]`
    Title(String),
    /// a role, under a title: `*Chief map designer*`
    Role(String),
    /// a name (any other line)
    Name(String),
    /// an empty line
    Blank,
}

/// The credits file: `[Title]` starts a group (three empty lines come before it, unless it is the first), `*Role*` is a role, an empty line is an empty line,
/// anything else is a name; lines starting with `;` are comments.
pub fn parse_credits(text: &str) -> Vec<CreditLine> {
    let mut lines = Vec::new();
    for raw in text.lines() {
        let line = raw.trim_end();
        if line.trim_start().starts_with(';') {
            continue;
        }
        let t = line.trim();
        if t.is_empty() {
            lines.push(CreditLine::Blank);
        } else if let Some(title) = t.strip_prefix('[').and_then(|r| r.strip_suffix(']')) {
            // (a title right under another one, like STAFF over MAP DESIGN, only gets one)
            if !lines.is_empty() {
                let gap = if matches!(lines.last(), Some(CreditLine::Title(_))) { 1 } else { 3 };
                lines.extend(std::iter::repeat(CreditLine::Blank).take(gap));
            }
            lines.push(CreditLine::Title(title.trim().to_string()));
        } else if let Some(role) = t.strip_prefix('*').and_then(|r| r.strip_suffix('*')) {
            lines.push(CreditLine::Role(role.trim().to_string()));
        } else {
            lines.push(CreditLine::Name(t.to_string()));
        }
    }
    lines
}

fn int(v: Option<&Value>, default: i32) -> i32 {
    v.and_then(|v| v.as_i64()).map(|v| v as i32).unwrap_or(default)
}

impl Cutscenes {
    pub fn load(data: &Path) -> Cutscenes {
        let dir = data.join("cutscenes");
        let mut c = Cutscenes { intro_strings: crate::yamldata::read(&data.join("scripts/intro/intro_strings.yaml")), ..Default::default() };
        c.credits = parse_credits(&crate::i18n::credit_text(data));
        c
    }

    /// The language changed (title screen): the credits are read again.
    pub fn reload_language(&mut self, data: &Path) {
        self.credits = parse_credits(&crate::i18n::credit_text(data));
    }

    /// The button presses of a playback sequence (common/cutscenes.lua: `playback_seq`): (pad bits, frames).
    pub fn playback(&self, n: i32, script: &Scripting) -> Option<Vec<(u8, u32)>> {
        let steps = script.story_ops("PLAYBACKS", n)?;
        Some(steps.as_array()?.iter().filter_map(|p| Some((p.get("buttons")?.as_u64()? as u8, p.get("frames")?.as_u64()? as u32))).collect())
    }

    /// Cutscenes from the text of a YAML tree {actions: {...}, scripts: {...}} (the tests).
    pub fn from_yaml(text: &str) -> Cutscenes {
        Cutscenes { root: serde_yaml::from_str::<serde_yaml::Value>(text).map(crate::yamldata::convert).unwrap_or_default(), ..Default::default() }
    }

    /// The operations of an action, with the calls into other actions expanded.
    pub fn action(&self, id: i32, script: Option<&Scripting>) -> Vec<Op> {
        let mut ops = Vec::new();
        self.expand(id, &mut ops, 0, script);
        ops
    }

    /// The boxes of a cutscene text.
    pub fn text(&self, id: i32) -> Vec<String> {
        let key = format!("cutscenes.t{}", id);
        if crate::i18n::has(&key) {
            // "Owner: Heh heh heh!" with the lines that follow indented by two spaces, like the original's boxes
            let who = crate::i18n::speaker(&key).map(|k| crate::i18n::t(&k));
            return crate::i18n::pages(&key)
                .iter()
                .map(|s| {
                    let text = format_text(s);
                    match &who {
                        Some(name) => {
                            let mut lines = text.lines();
                            let first = lines.next().unwrap_or("");
                            let rest: Vec<String> = lines.map(|l| format!("  {}", l)).collect();
                            std::iter::once(format!("{}: {}", name, first)).chain(rest).collect::<Vec<_>>().join("
")
                        }
                        None => text,
                    }
                })
                .collect();
        }
        // texts kept in the tree itself (the tests)
        self.root.get("texts").and_then(|t| t.get(id.to_string())).and_then(|n| n.as_array()).map(|a| a.iter().filter_map(|s| s.as_str()).map(format_text).collect()).unwrap_or_default()
    }

    /// An action (`which` "STORY") or a routine ("ROUTINES"): a tree kept by the tests, else the one the Lua scripts define (common/cutscenes.lua).
    fn lookup(&self, which: &str, id: i32, script: Option<&Scripting>) -> Option<Value> {
        let key = if which == "STORY" { "actions" } else { "scripts" };
        if let Some(v) = self.root.get(key).and_then(|a| a.get(id.to_string())) {
            return Some(v.clone());
        }
        script.and_then(|s| s.story_ops(which, id))
    }

    fn expand(&self, id: i32, out: &mut Vec<Op>, depth: usize, script: Option<&Scripting>) {
        if depth > 6 {
            return;
        }
        if let Some(list) = self.lookup("STORY", id, script) {
            self.expand_list(&list, out, depth, script);
        }
    }

    fn expand_list(&self, list: &Value, out: &mut Vec<Op>, depth: usize, script: Option<&Scripting>) {
        let Some(items) = list.as_array() else { return };
        for o in items {
            if let Some(v) = o.get("text") {
                // a cutscene id that is a routine of the original: its operations, in place
                let id = int(Some(v), 0);
                if let Some(routine) = self.lookup("ROUTINES", id, script) {
                    if depth < 8 {
                        self.expand_list(&routine, out, depth + 1, script);
                        continue;
                    }
                }
                let mut op = Op::new(Kind::Text);
                op.value = id;
                op.wait = int(o.get("wait"), 0);
                out.push(op);
            } else if let Some(v) = o.get("flag") {
                let mut op = Op::new(Kind::Flag);
                op.value = int(Some(v), 0);
                op.set = o.get("set").and_then(|s| s.as_bool()).unwrap_or(true);
                out.push(op);
            } else if let Some(v) = o.get("sound") {
                let mut op = Op::new(Kind::Sound);
                op.value = int(Some(v), 0);
                out.push(op);
            } else if let Some(v) = o.get("swap") {
                let mut op = Op::new(Kind::Swap);
                op.value = int(Some(v), 0);
                out.push(op);
            } else if let Some(v) = o.get("call") {
                self.expand(int(Some(v), 0), out, depth + 1, script);
            } else if let Some(v) = o.get("warp") {
                let mut op = Op::new(Kind::Warp);
                op.value = int(Some(v), 0);
                op.x = int(o.get("x"), 0);
                op.y = int(o.get("y"), 0);
                op.dir = int(o.get("dir"), -1);
                out.push(op);
            } else if let Some(v) = o.get("lock") {
                let mut op = Op::new(Kind::Lock);
                op.set = v.as_bool().unwrap_or(false);
                out.push(op);
            } else if let Some(v) = o.get("hostile") {
                let mut op = Op::new(Kind::Hostile);
                op.value = int(Some(v), -1);
                op.set = o.get("set").and_then(|s| s.as_bool()).unwrap_or(true);
                out.push(op);
            } else if let Some(v) = o.get("visible") {
                let mut op = Op::new(Kind::Visible);
                op.value = int(Some(v), -1);
                op.set = o.get("set").and_then(|s| s.as_bool()).unwrap_or(true);
                out.push(op);
            } else if o.get("credits").is_some() {
                out.push(Op::new(Kind::Credits));
            } else if let Some(v) = o.get("intro_string") {
                let mut op = Op::new(Kind::IntroString);
                op.value = int(Some(v), 0);
                out.push(op);
            } else if let Some(v) = o.get("fx") {
                let mut op = Op::new(Kind::Fx);
                op.name = v.as_str().unwrap_or("").to_string();
                out.push(op);
            } else if let Some(v) = o.get("wait") {
                let mut op = Op::new(Kind::Wait);
                op.value = int(Some(v), 0);
                out.push(op);
            } else if let Some(v) = o.get("friday") {
                let mut op = Op::new(Kind::Friday);
                op.name = v.as_str().unwrap_or("").to_string();
                out.push(op);
            } else if let Some(v) = o.get("hide") {
                let mut op = Op::new(Kind::Hide);
                op.value = int(Some(v), -1);
                out.push(op);
            } else if let Some(v) = o.get("block") {
                let mut op = Op::new(Kind::Block);
                op.cells = v.as_array().map(|a| a.iter().filter_map(|c| Some((c.get(0)?.as_i64()? as i32, c.get(1)?.as_i64()? as i32))).collect()).unwrap_or_default();
                out.push(op);
            } else if let Some(c) = o.get("if") {
                let mut op = Op::new(Kind::If);
                match c.get("flag") {
                    Some(f) => op.value = int(Some(f), 0),
                    None => op.on_answer = true,
                }
                if let Some(t) = o.get("then") {
                    self.expand_list(t, &mut op.then_ops, depth + 1, script);
                }
                if let Some(e) = o.get("else") {
                    self.expand_list(e, &mut op.else_ops, depth + 1, script);
                }
                out.push(op);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------
// what is going on
// -------------------------------------------------------------------------------------------------------------------

/// A room to go to behind a fade.
#[derive(Clone, Debug)]
pub struct PendingWarp {
    pub room: i32,
    pub x: i32,
    pub y: i32,
    pub dir: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FxMode {
    None,
    ToLevel,
    Pulse,
}

#[derive(Resource)]
pub struct Cutscene {
    ops: VecDeque<Op>,
    /// seconds a fade, a flash or a pause still takes: nothing goes on until it is over (and the scripts stand still)
    pub busy: f32,
    busy_then_warp: bool,
    /// a text has been dismissed: the operations go on
    pub resume: bool,
    /// the answer to the yes / no question of a text
    answer: Option<bool>,
    /// the entity whose script started the action
    source: Option<usize>,
    /// actions asked for, not started yet: (action, entity)
    requests: Vec<(i32, Option<usize>)>,
    /// the story took the controls: only a recorded input (`playback`) moves Nigel
    pub input_locked: bool,
    /// seconds Nigel lies still (in bed)
    pub hold: f32,
    pub hidden: bool,
    pending_warp: Option<PendingWarp>,
    warp_now: bool,
    /// a warp is under way: when the new room has loaded, this is done
    arriving: Option<PendingWarp>,
    pub faint: Option<f32>,
    faint_warp: bool,
    // screen effects
    pub fx_level: f32,
    fx_mode: FxMode,
    fx_from: f32,
    fx_to: f32,
    fx_t: f32,
    fx_dur: f32,
    pub fx_color: [u8; 3],
    /// Friday: 0 away, 1 appearing, 2 hovering, 3 leaving; seconds in that state
    pub friday: (u8, f32),
    /// the playback being replayed: (steps, index, seconds left in the step, sequence number)
    playback: Option<(Vec<(u8, u32)>, usize, f32, i32)>,
    /// an intro caption and the seconds left
    pub caption: Option<(i32, f32)>,
    /// the credits: seconds since they began
    pub credits: Option<f32>,
    /// the intro of a new game is under way
    pub intro: bool,
    /// the entity the camera looks at while the action runs (one started by a dormant enemy: the boss that wakes up)
    pub focus: Option<usize>,
    /// a scene recorded from the original is being played (the scripts, the walk and the fades are the recording's)
    pub track: Option<TrackRun>,
    /// the recording is over: the intro ends
    track_done: bool,
    /// the room was changed by hand (the panel): the intro and any scene stop, nothing stays black or letterboxed
    pub abort: bool,
    /// the text of the black message box of a recorded scene (an item found...)
    pub box_text: Option<String>,
    /// the frame of the recording at which the box of `box_text` opened, and the frame now (the letters are typed one every 2.4 frames)
    pub box_since: f32,
    pub track_t: f32,
    /// Friday's place on the screen during a recorded scene (pixels of the 320x224 picture)
    pub friday_screen: Option<(f32, f32)>,
    /// what Nigel does in a recorded scene: the bits of the original's Player_Action (0 when nothing is recorded)
    pub track_action: u16,
    /// seconds since `track_action` last changed
    pub track_action_t: f32,
}

/// A recorded scene being played: the frames (60 Hz) it has run, and where it is in its lists of events.
pub struct TrackRun {
    track: Track,
    t: f32,
    /// the room the recording has reached (the game loads it while the screen is black)
    room: i32,
    str_i: usize,
    msg_i: usize,
    action_i: usize,
    sound_i: usize,
    /// the song the recording asked for last (none yet: silence; the rooms' own songs are not played in a recorded scene)
    song: Option<u8>,
    last: Option<(f32, f32, f32)>,
}

impl Default for Cutscene {
    fn default() -> Self {
        Cutscene {
            ops: VecDeque::new(),
            busy: 0.0,
            busy_then_warp: false,
            resume: false,
            answer: None,
            source: None,
            requests: Vec::new(),
            input_locked: false,
            hold: 0.0,
            hidden: false,
            pending_warp: None,
            warp_now: false,
            arriving: None,
            faint: None,
            faint_warp: false,
            fx_level: 0.0,
            fx_mode: FxMode::None,
            fx_from: 0.0,
            fx_to: 0.0,
            fx_t: 0.0,
            fx_dur: 0.0,
            fx_color: [0, 0, 0],
            friday: (0, 0.0),
            playback: None,
            caption: None,
            credits: None,
            intro: false,
            focus: None,
            track: None,
            track_done: false,
            abort: false,
            box_text: None,
            box_since: 0.0,
            track_t: 0.0,
            friday_screen: None,
            track_action: 0,
            track_action_t: 0.0,
        }
    }
}

impl Cutscene {
    /// Nigel cannot walk, jump or act: a pause, a fade, he lies in bed, he has fainted, the credits roll.
    pub fn blocks_player(&self) -> bool {
        self.busy > 0.0 || self.hold > 0.0 || self.faint.is_some() || self.credits.is_some()
    }

    /// Anything of the story that stops the player from doing things (and the other systems from reading his buttons).
    pub fn active(&self) -> bool {
        self.input_locked || self.blocks_player() || !self.ops.is_empty() || self.arriving.is_some() || self.pending_warp.is_some()
    }

    /// The answer to a question of a text: the operations of the branch it chose go first.
    pub fn answer_with(&mut self, yes: bool, ops: Vec<Op>) {
        self.answer = Some(yes);
        for o in ops.into_iter().rev() {
            self.ops.push_front(o);
        }
    }

    /// An action is asked for (a script, a conversation).
    pub fn request(&mut self, id: i32, source: Option<usize>) {
        self.requests.push((id, source));
    }

    /// Where the recorded input moves Nigel now, as (dx, dy) with diagonals; None when nothing is replayed.
    pub fn playback_direction(&self) -> Option<(f32, f32)> {
        let (steps, i, _, _) = self.playback.as_ref()?;
        let b = steps.get(*i)?.0;
        let (mut dx, mut dy) = (0.0, 0.0);
        if b & 1 != 0 {
            dx -= 1.0;
        }
        if b & 2 != 0 {
            dx += 1.0;
        }
        if b & 4 != 0 {
            dy += 1.0;
        }
        if b & 8 != 0 {
            dy -= 1.0;
        }
        Some((dx, dy))
    }

    /// What the scripts' PLAYBACK condition sees: the sequence replayed (0xFE once its last entry, the hold, is reached or when the story just
    /// took the controls), 9 while Nigel lies still, 0 when the player has the controls.
    pub fn playback_state(&self) -> i32 {
        if let Some((steps, i, _, seq)) = &self.playback {
            if let Some(s) = steps.get(*i) {
                return if s.1 == 254 { 0xFE } else { *seq };
            }
        }
        if self.hold > 0.0 {
            return 9;
        }
        if self.input_locked {
            0xFE
        } else {
            0
        }
    }

    /// A recorded scene is under way.
    pub fn tracking(&self) -> bool {
        self.track.is_some()
    }

    /// The black bars of the scenes of the intro (the original shows 160 of its 224 lines).
    pub fn letterbox(&self) -> bool {
        self.intro
    }

    /// A new game played from a recording of the original (tools/emuhawk): the story's own scripts stand still.
    pub fn begin_intro_track(&mut self, track: Track, first_room: i32) {
        self.begin_intro();
        self.caption = None;
        // test aid: LS_TRACK_AT=frame starts the recording that far in (the room is loaded by the first update)
        let t0 = std::env::var("LS_TRACK_AT").ok().and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);
        let (str_i, msg_i, action_i, sound_i) = (track.strs.partition_point(|s| s.0 <= t0), track.msgs.partition_point(|s| s.0 <= t0), track.actions.partition_point(|s| s.0 <= t0), track.sounds.partition_point(|s| s.0 <= t0));
        self.track = Some(TrackRun { track, t: t0, room: first_room, str_i, msg_i, action_i, sound_i, song: None, last: None });
    }

    /// A new game without the intro: the adventure starts in room 148, where the intro ends.
    pub fn begin_adventure(&mut self) {
        *self = Cutscene::default();
        self.arriving = Some(PendingWarp { room: 148, x: 32, y: 38, dir: 2 });
    }

    /// A new game: the intro scenes (room 139 and on) run by themselves, the controls are the story's.
    pub fn begin_intro(&mut self) {
        *self = Cutscene::default();
        self.intro = true;
        self.input_locked = true;
        self.caption = Some((1, 2.0));
    }

    fn start_fx(&mut self, name: &str) -> f32 {
        let to_level = |s: &mut Cutscene, target: f32, dur: f32, color: [u8; 3]| {
            if (s.fx_level - target).abs() < 0.01 {
                return 0.0;
            }
            s.fx_mode = FxMode::ToLevel;
            s.fx_from = s.fx_level;
            s.fx_to = target;
            s.fx_t = 0.0;
            s.fx_dur = dur;
            s.fx_color = color;
            dur
        };
        let pulse = |s: &mut Cutscene, dur: f32, color: [u8; 3]| {
            s.fx_mode = FxMode::Pulse;
            s.fx_t = 0.0;
            s.fx_dur = dur;
            s.fx_color = color;
            dur
        };
        match name {
            "fade_out" => to_level(self, 1.0, 0.4, [0, 0, 0]),
            "fade_in" => {
                let c = self.fx_color;
                to_level(self, 0.0, 0.4, c)
            }
            "black_flash" => pulse(self, 0.4, [0, 0, 0]),
            "white_flash" => pulse(self, 0.4, [255, 255, 255]),
            "red_flash" => pulse(self, 0.5, [255, 30, 30]),
            "warp_pad" | "tree_warp" | "nole_warp" => pulse(self, 0.9, [255, 255, 255]),
            _ => 0.0,
        }
    }

    fn update_fx(&mut self, dt: f32) {
        if self.fx_mode == FxMode::None {
            return;
        }
        self.fx_t += dt;
        let k = if self.fx_dur > 0.0 { (self.fx_t / self.fx_dur).min(1.0) } else { 1.0 };
        if self.fx_mode == FxMode::ToLevel {
            self.fx_level = self.fx_from + (self.fx_to - self.fx_from) * k;
            if k >= 1.0 {
                self.fx_level = self.fx_to;
                self.fx_mode = FxMode::None;
            }
        } else {
            // up, then down
            self.fx_level = if k < 0.5 { k * 2.0 } else { (1.0 - k) * 2.0 };
            if k >= 1.0 {
                self.fx_level = 0.0;
                self.fx_mode = FxMode::None;
            }
        }
    }

    fn fade_out_then_warp(&mut self, warp: PendingWarp) {
        self.pending_warp = Some(warp);
        self.busy = self.start_fx("fade_out");
        if self.busy > 0.0 {
            self.busy_then_warp = true;
        } else {
            self.warp_now = true;
        }
    }
}

/// What the operations act on.
struct Env<'a> {
    game: &'a mut GameState,
    script: &'a Scripting,
    sfx: &'a mut SfxQueue,
    dlg: &'a mut Dialogue,
    frozen: &'a mut GameFrozen,
    combat: &'a mut Combat,
    room: &'a mut RoomRes,
    swap: Option<&'a mut SwapState>,
    cs: &'a Cutscenes,
}

impl Cutscene {
    /// A story action starts. Action 0 is the faint.
    fn trigger(&mut self, id: i32, source: Option<usize>, env: &mut Env) {
        if id == 0 {
            if self.faint.is_none() && !env.combat.dead {
                self.faint = Some(0.0);
            }
            return;
        }
        let mut ops = env.cs.action(id, Some(env.script));
        if self.track.is_some() {
            // a recorded scene has its own rooms, fades, captions, sounds, characters and Friday: only the talking is left (the story flags the actions set
            // would hide characters the recording shows: the original's flags are all clear when the adventure starts)
            ops.retain(|o| matches!(o.kind, Kind::Text | Kind::Wait));
        }
        if std::env::var("LS_LOG_CUTSCENE").is_ok() {
            println!("[cutscene] action {} ({} operations) asked by entity {:?} (busy {:.2}, box open {}, queued {})", id, ops.len(), source, self.busy, env.dlg.open, self.ops.len());
        }
        if ops.is_empty() {
            return;
        }
        self.source = source;
        // an action started by a boss (or any dormant enemy) looks at it
        self.focus = source.filter(|s| env.script.entities().get(*s).is_some_and(|e| e.dormant));
        self.ops.extend(ops);
        self.answer = None;
        if !env.dlg.open && self.busy <= 0.0 {
            self.run(env);
        }
    }

    /// Runs the operations until one has to wait.
    fn run(&mut self, env: &mut Env) {
        while let Some(op) = self.ops.pop_front() {
            match op.kind {
                Kind::Flag => {
                    if op.set {
                        env.script.set_flag(op.value);
                    } else {
                        env.script.clear_flag(op.value);
                    }
                }
                Kind::Sound => env.sfx.play(op.value as u8),
                Kind::Swap => {
                    // the first swap of this trigger whose flag is not set yet happens
                    let flags: Vec<(i64, i64)> = env.room.room.file.swap_flags.iter().filter_map(|s| Some((s.get("trigger")?.as_i64()?, s.get("flag")?.as_i64()?))).collect();
                    if let Some((_, flag)) = flags.into_iter().find(|(t, f)| *t == op.value as i64 && !env.script.flag(*f as i32)) {
                        env.script.set_flag(flag as i32);
                    }
                }
                Kind::Text => {
                    let pages = env.cs.text(op.value);
                    if pages.is_empty() {
                        continue;
                    }
                    // a text followed by a branch on the answer ends with a yes / no question
                    let branch = match self.ops.front() {
                        Some(next) if next.kind == Kind::If && next.on_answer && self.answer.is_none() => {
                            let b = self.ops.pop_front().unwrap();
                            Some((b.then_ops, b.else_ops))
                        }
                        _ => None,
                    };
                    if std::env::var("LS_LOG_CUTSCENE").is_ok() {
                        println!("[cutscene] text {} ({} pages, wait {})", op.value, pages.len(), op.wait);
                    }
                    // in a recorded scene every box goes by itself (a wait of 0 would wait for a button for ever)
                    let wait = if self.track.is_some() { (op.wait as f32).max(1.0) } else { op.wait as f32 };
                    env.dlg.start_cutscene(pages, wait / 60.0, branch);
                    env.frozen.0 = true;
                    return; // goes on when the last box is dismissed (or the question answered)
                }
                Kind::Hostile | Kind::Visible | Kind::Hide => {
                    let idx = if op.value < 0 { self.source.map_or(-1, |s| s as i32) } else { op.value };
                    if idx < 0 {
                        continue;
                    }
                    let idx = idx as usize;
                    match op.kind {
                        Kind::Hostile => env.script.set_hostile(idx, op.set),
                        Kind::Visible => {
                            let was = env.script.set_visible(idx, op.set);
                            if op.set && !was && !self.intro {
                                // appears with a rumble and sparks
                                env.sfx.play(0x59);
                                env.combat.shake = (0.7, 0.7, 2.5);
                                env.combat.burst_at(env.script, idx);
                            }
                        }
                        _ => {
                            env.script.hide_entity(idx);
                            env.game.collected.insert((env.room.room.file.id, idx)); // gone for good
                        }
                    }
                }
                Kind::Block => {
                    for (x, y) in &op.cells {
                        let off = env.room.room.file.map.cell_offset;
                        let (hx, hy) = (x - off, y - off);
                        if let Some(mut c) = env.room.room.cell(hx, hy) {
                            c.flags |= 0x4;
                            env.room.room.set_cell(hx, hy, c);
                            // the map that the swaps start from too, or the next swap would open it again
                            if let Some(swap) = env.swap.as_deref_mut() {
                                let (gx, gy) = (hx - env.room.room.file.map.grid.origin[0], hy - env.room.room.file.map.grid.origin[1]);
                                let width = env.room.room.tmx.width;
                                if gx >= 0 && gy >= 0 {
                                    if let Some(v) = env.room.room.tmx.layers.get("cell_flags").and_then(|l| l.get(gy as usize * width + gx as usize)).copied() {
                                        if let Some(b) = swap.base.get_mut("cell_flags").and_then(|l| l.get_mut(gy as usize * width + gx as usize)) {
                                            *b = v;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Kind::Lock => {
                    self.input_locked = op.set;
                    if !op.set {
                        self.intro = false;
                        self.hold = 0.0;
                        self.hidden = false;
                        self.friday = (0, 0.0);
                        self.playback = None;
                        env.script.set_intro(false);
                    }
                }
                Kind::Warp => {
                    // fade out first; the room changes when the screen is black, then the new room fades in
                    let room = if op.value < 0 { env.room.room.file.id } else { op.value };
                    self.fade_out_then_warp(PendingWarp { room, x: op.x, y: op.y, dir: op.dir });
                    return;
                }
                Kind::Fx => {
                    let dur = self.start_fx(&op.name);
                    if dur > 0.0 {
                        self.busy = dur;
                        return;
                    }
                }
                Kind::IntroString => {
                    let frames = env.cs.intro_strings.get(op.value.to_string()).and_then(|s| s.get("frames")).and_then(|f| f.as_f64()).unwrap_or(0.0) as f32;
                    if frames > 0.0 {
                        self.caption = Some((op.value, frames / 60.0));
                    }
                }
                Kind::Credits => {
                    self.ops.clear();
                    self.credits = Some(0.0);
                    self.input_locked = true;
                    return;
                }
                Kind::Wait => {
                    if op.value > 0 {
                        self.busy = op.value as f32 / 60.0;
                        return;
                    }
                }
                Kind::Friday => match op.name.as_str() {
                    "appear" => self.friday = (1, 0.0),
                    "hover" => {
                        if self.friday.0 == 0 {
                            self.friday = (2, 0.0);
                        }
                    }
                    "leave" => {
                        if self.friday.0 != 0 {
                            self.friday = (3, 0.0);
                        }
                    }
                    _ => self.friday = (0, 0.0),
                },
                Kind::If => {
                    // a branch on a flag, or on an answer nobody asked for (taken as "no")
                    let yes = if op.on_answer { self.answer.take().unwrap_or(false) } else { env.script.flag(op.value) };
                    let list = if yes { &op.then_ops } else { &op.else_ops };
                    for o in list.iter().rev() {
                        self.ops.push_front(o.clone());
                    }
                }
            }
        }
    }
}

/// The story goes on: actions asked for are started, the timers run, a text that was dismissed lets the operations go on, a warp
/// changes the room.
#[allow(clippy::too_many_arguments)]
pub fn cutscene_update(
    time: Res<Time>,
    mut cut: ResMut<Cutscene>,
    cs: Res<Cutscenes>,
    (mut game, script, mut sfx): (ResMut<GameState>, NonSend<Scripting>, ResMut<SfxQueue>),
    (mut dlg, mut frozen, mut combat): (ResMut<Dialogue>, ResMut<GameFrozen>, ResMut<Combat>),
    (mut room, swap): (ResMut<RoomRes>, Option<ResMut<SwapState>>),
    (mut request, mut arrival, mut sound): (ResMut<RoomRequest>, ResMut<Arrival>, ResMut<Sound>),
    _ctl: Res<crate::config::Controls>,
) {
    let dt = time.delta_secs().min(0.05);
    let cut = &mut *cut;
    let mut swap = swap;
    let mut env = Env { game: &mut game, script: &script, sfx: &mut sfx, dlg: &mut dlg, frozen: &mut frozen, combat: &mut combat, room: &mut room, swap: swap.as_deref_mut(), cs: &cs };
    if cut.intro {
        if !cut.tracking() {
            sound.room_song = Some(0x2B); // the intro's music
        }
        env.script.set_intro(true);
    }
    // a room chosen by hand ends the scene in progress
    if cut.abort {
        *cut = Cutscene::default();
        env.script.set_intro(false);
        env.dlg.close_all();
        env.frozen.0 = false;
        return;
    }
    // the end of the intro (it cannot be skipped): the adventure starts in room 148
    if cut.intro && cut.track_done && cut.arriving.is_none() && cut.pending_warp.is_none() {
        let faint = cut.faint_warp;
        *cut = Cutscene::default();
        cut.faint_warp = faint;
        env.script.set_intro(false);
        env.dlg.close_all();
        env.frozen.0 = false;
        request.0 = Some(148);
        arrival.0 = Some(Vec2::new(32.5, 38.5));
        cut.arriving = Some(PendingWarp { room: 148, x: 32, y: 38, dir: 2 });
        return;
    }
    // the story's own timers (they run while the game stands still behind a text)
    cut.update_fx(dt);
    cut.friday.1 += dt;
    if cut.friday.0 == 1 && cut.friday.1 >= 0.5 {
        cut.friday.0 = 2;
    }
    if cut.friday.0 == 3 && cut.friday.1 >= 0.8 {
        cut.friday.0 = 0;
    }
    cut.hold = (cut.hold - dt).max(0.0);
    if let Some((n, t)) = cut.caption.as_mut() {
        *t -= dt;
        if *t <= 0.0 {
            let _ = n;
            cut.caption = None;
        }
    }
    if let Some(t) = cut.credits.as_mut() {
        *t += dt;
    }
    // the recorded input
    if let Some((steps, i, left, _)) = cut.playback.as_mut() {
        if *i < steps.len() && steps[*i].1 != 254 {
            *left -= dt;
            if *left <= 0.0 {
                *i += 1;
                if *i < steps.len() {
                    *left += steps[*i].1 as f32 / 60.0;
                }
            }
        }
    }
    if env.script.take_pause() {
        cut.hold = cut.hold.max(254.0 / 60.0);
    }
    for n in env.script.take_playbacks() {
        if let Some(steps) = cs.playback(n, env.script) {
            let first = steps.first().map_or(0.0, |s| s.1 as f32 / 60.0);
            cut.playback = Some((steps.clone(), 0, first, n));
        }
    }
    env.script.set_playback_state(cut.playback_state());
    // faint: the death pictures for a moment, then the story goes on where he fell
    if let Some(t) = cut.faint.as_mut() {
        *t += dt;
        if *t >= 2.5 {
            cut.faint = None;
            if env.room.room.file.id == 426 && !env.script.flag(4) {
                cut.faint_warp = true;
                cut.fade_out_then_warp(PendingWarp { room: 593, x: 16, y: 15, dir: 2 });
            }
        }
    }
    // actions asked for
    for (source, id) in env.script.take_cutscenes() {
        cut.requests.push((id, usize::try_from(source).ok()));
    }
    for (id, source) in std::mem::take(&mut cut.requests) {
        cut.trigger(id, source, &mut env);
    }
    // a pause, a fade is over
    if cut.busy > 0.0 {
        cut.busy -= dt;
        if cut.busy <= 0.0 {
            cut.busy = 0.0;
            if cut.busy_then_warp {
                cut.busy_then_warp = false;
                cut.warp_now = true;
            } else {
                cut.resume = true;
            }
        }
    }
    // the screen is black: the room changes
    if cut.warp_now {
        cut.warp_now = false;
        if let Some(w) = cut.pending_warp.take() {
            request.0 = Some(w.room);
            arrival.0 = Some(Vec2::new(w.x as f32 + 0.5, w.y as f32 + 0.5));
            cut.arriving = Some(w);
        }
    }
    if cut.resume && !env.dlg.open && cut.busy <= 0.0 {
        cut.resume = false;
        if cut.arriving.is_none() {
            cut.run(&mut env);
        }
    }
    if cut.ops.is_empty() && !env.dlg.open {
        cut.focus = None;
    }
}

/// Plays the recorded scene: puts the player and the entities where the original had them, fades, captions.
pub fn track_update(
    time: Res<Time>,
    mut cut: ResMut<Cutscene>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    (mut request, mut arrival): (ResMut<RoomRequest>, ResMut<Arrival>),
    mut players: Query<&mut Player>,
    (mut sfx, mut sound): (ResMut<SfxQueue>, ResMut<Sound>),
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    mut shots: Local<(Vec<f32>, usize)>,
) {
    let dt = time.delta_secs().min(0.05);
    let cut = &mut *cut;
    let Some(run) = cut.track.as_mut() else { return };
    // test aid: LS_TRACK_SHOTS=f1,f2,... takes a picture of the window when the recording reaches each frame, into LS_TRACK_SHOT_DIR (p_<frame>.png), then quits
    if shots.0.is_empty() && shots.1 == 0 {
        shots.0 = std::env::var("LS_TRACK_SHOTS").unwrap_or_default().split(',').filter_map(|v| v.trim().parse().ok()).collect();
        shots.1 = usize::from(shots.0.is_empty()) * usize::MAX / 2;
    }
    if shots.1 < shots.0.len() && run.t >= shots.0[shots.1] {
        let dir = std::env::var("LS_TRACK_SHOT_DIR").unwrap_or_else(|_| ".".into());
        let path = std::path::PathBuf::from(dir).join(format!("p_{}.png", shots.0[shots.1] as i32));
        commands.spawn(bevy::render::view::screenshot::Screenshot::primary_window()).observe(bevy::render::view::screenshot::save_to_disk(path));
        shots.1 += 1;
    } else if shots.1 >= shots.0.len() && !shots.0.is_empty() && shots.1 < usize::MAX / 4 && run.t > shots.0[shots.0.len() - 1] + 30.0 {
        exit.write(AppExit::Success);
    }
    // the recording runs on while a conversation is on: the original's pages last their own number of frames (the `wait` of the
    // operations) and what its characters do meanwhile is in the recording
    run.t += dt * 60.0;
    let t = run.t;
    cut.fx_color = [0, 0, 0];
    cut.fx_level = run.track.fade_at(t);
    cut.friday_screen = run.track.friday_at(t);
    cut.track_t = t;
    let action = run.track.pose_at(t);
    if action != cut.track_action {
        cut.track_action = action;
        cut.track_action_t = 0.0;
    }
    cut.track_action_t += dt;
    sound.room_song = run.song;
    // the sounds the original asked for: songs and sound effects
    while run.sound_i < run.track.sounds.len() && run.track.sounds[run.sound_i].0 <= t {
        let id = run.track.sounds[run.sound_i].1;
        // 0x1xx: the same sound asked for with a flag of the driver (0x159: the rumble of the boulder)
        let low = (id & 0xFF) as u8;
        if low < 0x40 && id < 0x100 {
            run.song = Some(low);
        } else if low >= 0x40 {
            sfx.play(low);
        }
        run.sound_i += 1;
    }
    if let Some((frame, next)) = run.track.handover {
        if t >= frame {
            if next == 148 {
                // the end of the intro: the adventure starts in room 148 (the skip's way)
                cut.track = None;
                cut.track_done = true;
                cut.box_text = None;
                return;
            }
            // the recording stops here: the game's own scripts play the rest in this room
            let s = run.track.at(0, frame);
            request.0 = Some(next);
            arrival.0 = s.map(|s| Vec2::new(s.x, s.y));
            cut.arriving = Some(PendingWarp { room: next, x: s.map_or(0, |s| s.x as i32), y: s.map_or(0, |s| s.y as i32), dir: -1 });
            cut.fx_level = 1.0;
            cut.track = None;
            cut.box_text = None;
            return;
        }
    } else if t > run.track.length + 20.0 {
        cut.track = None;
        cut.track_done = true;
        return;
    }
    // the room changes behind the black
    if let Some(r) = run.track.room_at(t) {
        if r != run.room {
            run.room = r;
            let s = run.track.at(0, t);
            request.0 = Some(r);
            arrival.0 = s.map(|s| Vec2::new(s.x, s.y));
        }
    }
    while run.str_i < run.track.strs.len() && run.track.strs[run.str_i].0 <= t {
        let (_, id, frames) = run.track.strs[run.str_i];
        cut.caption = if id != 0 { Some((id, 1.0e9 + frames)) } else { None }; // the original counts the frames down only while the screen is lit: the recording says when it goes
        run.str_i += 1;
    }
    let mut asked = Vec::new();
    while run.action_i < run.track.actions.len() && run.track.actions[run.action_i].0 <= t {
        asked.push(run.track.actions[run.action_i].1);
        run.action_i += 1;
    }
    while run.msg_i < run.track.msgs.len() && run.track.msgs[run.msg_i].0 <= t {
        let text = run.track.msgs[run.msg_i].1.clone();
        cut.box_text = if text.is_empty() { None } else { Some(box_text_of(&text)) }; // the track holds the key of the text (and the page: key#n)
        cut.box_since = run.track.msgs[run.msg_i].0;
        run.msg_i += 1;
    }
    if room.room.file.id != run.room {
        cut.requests.extend(asked.into_iter().map(|id| (id, None)));
        return; // not loaded yet: the entities are the old room's
    }
    if let (Ok(mut p), Some(s)) = (players.single_mut(), run.track.at(0, t)) {
        // what Nigel does comes from the original's Player_Action (walking bits 0-3, fall 4, jump 5): carried by a platform he does not walk
        let recorded = !run.track.poses.is_empty();
        let moved = if recorded { action & 0x0F != 0 } else { run.last.is_some_and(|(x, y, _)| (x - s.x).abs() + (y - s.y).abs() > 0.001) };
        let airborne = if recorded { action & 0x30 != 0 } else { run.last.is_some_and(|(_, _, z)| (z - s.z).abs() > 0.001) };
        p.x = s.x;
        p.y = s.y;
        p.z = s.z;
        p.vz = 0.0;
        p.facing = [1u8, 2, 3, 0][(s.rot & 3) as usize];
        p.moving = moved;
        p.grounded = !airborne;
        if moved {
            p.anim_time += dt;
        }
        cut.hidden = s.hidden;
        run.last = Some((s.x, s.y, s.z));
    }
    for slot in 1..run.track.slots.len() {
        if let Some(s) = run.track.at(slot, t) {
            let before = run.track.at(slot, t - 2.0).filter(|b| (b.x - s.x).abs() + (b.y - s.y).abs() < 1.5);
            let moved = before.is_some_and(|b| (b.x - s.x).abs() + (b.y - s.y).abs() > 0.001);
            script.place_entity(slot - 1, s.x, s.y, s.z, s.rot as i32, !s.hidden, moved);
        }
    }
    cut.requests.extend(asked.into_iter().map(|id| (id, None)));
}

/// The new room is there: Nigel faces the way the story wants, and the screen fades in.
pub fn cutscene_arrived(mut cut: ResMut<Cutscene>, mut players: Query<&mut Player, Added<Player>>) {
    let Ok(mut p) = players.single_mut() else { return };
    let Some(w) = cut.arriving.take() else { return };
    if w.dir >= 0 {
        // an entity's direction -> Nigel's facing
        p.facing = [1u8, 2, 3, 0][(w.dir & 3) as usize];
    }
    if cut.faint_warp {
        // he is in bed, under the blanket, and Friday hovers beside him
        cut.faint_warp = false;
        cut.hidden = true;
        cut.hold = 254.0 / 60.0;
        cut.friday = (2, 0.0);
    } else {
        cut.hidden = false;
    }
    cut.fx_level = cut.fx_level.max(1.0); // still black: the new room fades in
    cut.busy = cut.start_fx("fade_in");
    if cut.busy <= 0.0 {
        cut.resume = true;
    }
}

// -------------------------------------------------------------------------------------------------------------------
// what is seen
// -------------------------------------------------------------------------------------------------------------------

/// The picture of the game is the original's 320x224 (scaled): what lies beyond it stays black (rooms have characters and scenery that the
/// original never showed there: room 183 has three people waiting below the screen).
pub fn viewport_mask(mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?.clone();
    let screen = ctx.content_rect();
    let area = egui::Rect::from_min_size(screen.min, egui::vec2(screen.width() - crate::playtest::PANEL_W, screen.height()));
    let (o, k) = crate::picture_origin(area);
    let (left, right, bottom) = (o.x, o.x + 320.0 * k, o.y + 224.0 * k);
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("viewport_mask")));
    let black = egui::Color32::BLACK;
    if bottom < area.bottom() {
        painter.rect_filled(egui::Rect::from_min_max(egui::pos2(area.left(), bottom), area.max), 0.0, black);
    }
    if left > area.left() {
        painter.rect_filled(egui::Rect::from_min_max(area.min, egui::pos2(left, area.bottom())), 0.0, black);
    }
    if right < area.right() {
        painter.rect_filled(egui::Rect::from_min_max(egui::pos2(right, area.top()), area.max), 0.0, black);
    }
    Ok(())
}

/// Fades and flashes over the game area.
pub fn fx_overlay(mut contexts: EguiContexts, cut: Res<Cutscene>) -> Result {
    if cut.fx_level <= 0.001 && !cut.letterbox() {
        return Ok(());
    }
    let ctx = contexts.ctx_mut()?.clone();
    let screen = ctx.content_rect();
    let area = egui::Rect::from_min_size(screen.min, egui::vec2(screen.width() - crate::playtest::PANEL_W, screen.height()));
    if cut.letterbox() {
        // the scenes of the intro show 160 of the 224 lines of the screen (lines 24 to 184), the rest is black
        let (_, k) = crate::picture_origin(area);
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("letterbox")));
        painter.rect_filled(egui::Rect::from_min_max(area.min, egui::pos2(area.right(), area.top() + 24.0 * k)), 0.0, egui::Color32::BLACK);
        painter.rect_filled(egui::Rect::from_min_max(egui::pos2(area.left(), area.top() + 184.0 * k), area.max), 0.0, egui::Color32::BLACK);
    }
    let c = cut.fx_color;
    egui::Area::new(egui::Id::new("fx")).order(egui::Order::Foreground).fixed_pos(area.min).interactable(false).show(&ctx, |ui| {
        ui.painter().rect_filled(area, 0.0, egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], (cut.fx_level.clamp(0.0, 1.0) * 255.0) as u8));
    });
    Ok(())
}

/// The captions of the intro (the text of the intro screens), drawn with the intro font.
pub fn caption_overlay(mut contexts: EguiContexts, cut: Res<Cutscene>, cs: Res<Cutscenes>, mut font: Local<Option<Option<egui::TextureHandle>>>) -> Result {
    let Some((n, _)) = cut.caption else { return Ok(()) };
    let ctx = contexts.ctx_mut()?.clone();
    let tex = font.get_or_insert_with(|| {
        let path = std::path::Path::new(&std::env::var("BEVY_ASSET_ROOT").unwrap_or_default()).join("data/graphics/fonts/introfont.png");
        let mut img = image::open(path).ok()?.to_rgba8();
        // the original's colours on the screen: light grey letters with a black edge (the picture of the font has white and blue)
        for px in img.pixels_mut() {
            if px.0 == [255, 255, 255, 255] {
                px.0 = [204, 204, 204, 255];
            } else if px.0 == [30, 40, 110, 255] {
                px.0 = [0, 0, 0, 255];
            }
        }
        let size = [img.width() as usize, img.height() as usize];
        Some(ctx.load_texture("introfont", egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw()), egui::TextureOptions::NEAREST))
    });
    let Some(tex) = tex.as_ref() else { return Ok(()) };
    let Some(s) = cs.intro_strings.get(n.to_string()) else { return Ok(()) };
    let screen = ctx.content_rect();
    let area = egui::Rect::from_min_size(screen.min, egui::vec2(screen.width() - crate::playtest::PANEL_W, screen.height()));
    let (origin, k) = crate::picture_origin(area);
    let top = origin.y;
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("caption")));
    for (key, xk, yk) in [("line1", "x1", "y1"), ("line2", "x2", "y2")] {
        let (sx, sy) = (int(s.get(xk), 0), int(s.get(yk), 0));
        let glyphs: Vec<i32> = s.get(key).and_then(|l| l.as_array()).map(|a| a.iter().map(|g| int(Some(g), 0)).collect()).unwrap_or_default();
        for (i, g) in glyphs.iter().enumerate() {
            if *g <= 0 {
                continue;
            }
            let x = origin.x + ((sx - 0x80) as f32 + i as f32 * 8.0) * k;
            let y = top + (sy - 0x80) as f32 * k;
            let (u0, u1) = (*g as f32 * 8.0 / tex.size()[0] as f32, (*g as f32 * 8.0 + 8.0) / tex.size()[0] as f32);
            painter.image(tex.id(), egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(8.0 * k, 16.0 * k)), egui::Rect::from_min_max(egui::pos2(u0, 0.0), egui::pos2(u1, 1.0)), egui::Color32::WHITE);
        }
    }
    Ok(())
}

/// The text of a box of a recorded scene: `key` or `key#page` of the dictionaries, with the speaker before it when the key has one.
fn box_text_of(spec: &str) -> String {
    let (key, page) = spec.split_once('#').map_or((spec, 0), |(k, p)| (k, p.parse().unwrap_or(0)));
    let pages = crate::i18n::pages(key);
    let text = format_text(pages.get(page).map_or("", |s| s.as_str()));
    match crate::i18n::speaker(key).map(|k| crate::i18n::t(&k)) {
        Some(name) => {
            let mut lines = text.lines();
            let first = lines.next().unwrap_or("");
            std::iter::once(format!("{}: {}", name, first)).chain(lines.map(|l| format!("  {}", l))).collect::<Vec<_>>().join("
")
        }
        None => text,
    }
}

/// The message box of the recorded scenes: the same window as every other text of the game (shops::text_box).
pub fn scene_box_overlay(mut contexts: EguiContexts, cut: Res<Cutscene>, config: Res<crate::config::Config>) -> Result {
    let Some(text) = cut.box_text.as_ref() else { return Ok(()) };
    let ctx = contexts.ctx_mut()?.clone();
    // typed one letter a frame (a box of the story: the blips of the original)
    let typed: String = if cut.box_since > 0.0 { text.chars().take((((cut.track_t - cut.box_since).max(0.0)) / 1.0) as usize + 1).collect() } else { text.clone() };
    crate::shops::text_box(&ctx, &config.ui, "scene_box", text, &typed, None);
    Ok(())
}

/// The credits: black, the lines scroll up.
pub fn credits_overlay(mut contexts: EguiContexts, mut cut: ResMut<Cutscene>, cs: Res<Cutscenes>) -> Result {
    let Some(t) = cut.credits else { return Ok(()) };
    let ctx = contexts.ctx_mut()?.clone();
    let screen = ctx.content_rect();
    let area = egui::Rect::from_min_size(screen.min, egui::vec2(screen.width() - crate::playtest::PANEL_W, screen.height()));
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("credits")));
    painter.rect_filled(area, 0.0, egui::Color32::BLACK);
    // a centred list: titles underlined, roles in grey, names in white, empty lines are space
    const STEP: f32 = 38.0;
    let mut y = area.bottom() - t * 40.0;
    let mut last = y;
    for line in &cs.credits {
        last = y;
        let visible = y > area.top() - 40.0 && y < area.bottom() + 40.0;
        if visible {
            let centre = egui::pos2(area.center().x, y);
            match line {
                CreditLine::Title(text) => {
                    let rect = painter.text(centre, egui::Align2::CENTER_TOP, text, egui::FontId::monospace(26.0), egui::Color32::WHITE);
                    painter.line_segment([egui::pos2(rect.left(), rect.bottom() + 3.0), egui::pos2(rect.right(), rect.bottom() + 3.0)], egui::Stroke::new(2.0, egui::Color32::WHITE));
                }
                CreditLine::Role(text) => {
                    painter.text(centre, egui::Align2::CENTER_TOP, text, egui::FontId::monospace(20.0), egui::Color32::from_gray(170));
                }
                CreditLine::Name(text) => {
                    painter.text(centre, egui::Align2::CENTER_TOP, text, egui::FontId::monospace(24.0), egui::Color32::WHITE);
                }
                CreditLine::Blank => {}
            }
        }
        y += STEP;
    }
    if last < area.top() - 40.0 {
        cut.credits = None;
        cut.input_locked = false;
    }
    Ok(())
}

/// Friday the fairy, over the game: she appears from below, hovers, leaves.
#[derive(Component)]
pub struct FridaySprite;

#[allow(clippy::too_many_arguments)]
pub fn friday_sprite(
    mut commands: Commands,
    cut: Res<Cutscene>,
    time: Res<Time>,
    files: Res<crate::assets::DataFiles>,
    sprites: Res<crate::Sprites>,
    mut images: ResMut<Assets<Image>>,
    cam: Query<&Transform, (With<Camera2d>, Without<FridaySprite>)>,
    mut q: Query<(Entity, &mut Sprite, &mut Transform, &mut Visibility), With<FridaySprite>>,
    mut cache: Local<Option<(String, Handle<Image>)>>,
) {
    let Ok(cam) = cam.single() else { return };
    if q.is_empty() {
        commands.spawn((FridaySprite, Sprite::default(), Transform::from_xyz(0.0, 0.0, 50.0), Visibility::Hidden));
        return;
    }
    let (_, mut sprite, mut tf, mut vis) = q.single_mut().unwrap();
    let (state, t) = cut.friday;
    let on_track = cut.tracking() || (cut.intro && cut.friday_screen.is_some());
    if (on_track && cut.friday_screen.is_none()) || (!on_track && state == 0) {
        if *vis != Visibility::Hidden {
            *vis = Visibility::Hidden;
        }
        return;
    }
    // the picture: the walking frames of Friday (entity 97), facing the player
    let Some(pic) = crate::entity_picture(97, 2, None, &sprites.entities, "", Some(time.elapsed_secs()), None) else { return };
    if cache.as_ref().map(|c| &c.0) != Some(&pic.path) {
        let Some(img) = files.image(&pic.path) else { return };
        let (w, h) = img.dimensions();
        let handle = images.add(Image::new(bevy::render::render_resource::Extent3d { width: w, height: h, depth_or_array_layers: 1 }, bevy::render::render_resource::TextureDimension::D2, img.into_raw(), bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb, bevy::asset::RenderAssetUsages::default()));
        *cache = Some((pic.path.clone(), handle));
    }
    sprite.image = cache.as_ref().unwrap().1.clone();
    sprite.flip_x = pic.mirror;
    *vis = Visibility::Inherited;
    if let (true, Some((fx, fy))) = (on_track, cut.friday_screen) {
        // a recorded scene: where the original's sprite of Friday is (24 x 24 pixels), in pixels of its picture
        let k = crate::SCALE;
        let (sx, sy) = ((fx + 12.0) * k, (fy + 12.0) * k);
        tf.translation = Vec3::new(cam.translation.x - crate::playtest::PANEL_W / 2.0 + (sx - 640.0), cam.translation.y + (480.0 - sy), 50.0);
        tf.scale = Vec3::splat(k);
        return;
    }
    // where: above the middle of the game area (the panel covers the right of the window), in screen pixels (3 per picture pixel)
    let view_h = 960.0;
    let hover = -view_h * 0.28 + (t * 3.1).sin() * 9.0;
    let y = match state {
        1 => hover - (1.0 - (t / 0.5).min(1.0)) * view_h * 0.15,
        3 => hover - (t / 0.7).min(1.0) * view_h * 0.35,
        _ => hover,
    };
    let x = (t * 2.2).sin() * 30.0;
    tf.translation = Vec3::new(cam.translation.x - crate::playtest::PANEL_W / 2.0 + x, cam.translation.y - y, 50.0);
    tf.scale = Vec3::splat(crate::SCALE);
}

#[cfg(test)]
mod tests {
    use super::*;

    const YAML: &str = r#"{
      "texts": {"1": ["{5B}Hello{57}{5F}{5E}"], "2": ["a", "b"]},
      "scripts": {"77": [{"flag": 5}]},
      "actions": {
        "9": [{"flag": 1}, {"sound": 66}, {"text": 77}, {"text": 1, "wait": 30}, {"if": {"flag": 3}, "then": [{"warp": 10, "x": 4, "y": 5, "dir": 2}], "else": [{"fx": "fade_out"}]}, {"lock": true}],
        "10": [{"call": 9}, {"hide": -1}, {"block": [[1, 2], [3, 4]]}]
      }
    }"#;

    #[test]
    fn an_action_expands_its_calls_and_routines() {
        let c = Cutscenes::from_yaml(YAML);
        let ops = c.action(9, None);
        let kinds: Vec<Kind> = ops.iter().map(|o| o.kind).collect();
        // the text 77 is a routine of the original (a flag), the text 1 waits 30 frames
        assert_eq!(kinds, vec![Kind::Flag, Kind::Sound, Kind::Flag, Kind::Text, Kind::If, Kind::Lock]);
        assert_eq!(ops[3].wait, 30);
        assert_eq!((ops[4].then_ops[0].kind, ops[4].then_ops[0].value, ops[4].then_ops[0].x, ops[4].then_ops[0].dir), (Kind::Warp, 10, 4, 2));
        assert_eq!(ops[4].else_ops[0].name, "fade_out");
        // a call expands in place
        let ten = c.action(10, None);
        assert_eq!(ten.len(), 6 + 2);
        assert_eq!(ten[7].cells, vec![(1, 2), (3, 4)]);
        assert_eq!(c.text(1), vec!["Hello\nNigel".to_string()]);
        assert!(c.text(99).is_empty());
    }

    #[test]
    fn the_fades_run_up_and_down() {
        let mut s = Cutscene::default();
        assert_eq!(s.start_fx("fade_out"), 0.4);
        s.update_fx(0.2);
        assert!((s.fx_level - 0.5).abs() < 1e-5);
        s.update_fx(0.3);
        assert_eq!(s.fx_level, 1.0);
        // already black: no need to fade out again; the fade in goes back to clear
        assert_eq!(s.start_fx("fade_out"), 0.0);
        assert_eq!(s.start_fx("fade_in"), 0.4);
        s.update_fx(0.5);
        assert_eq!(s.fx_level, 0.0);
        // a flash goes up and down
        assert_eq!(s.start_fx("white_flash"), 0.4);
        s.update_fx(0.2);
        assert!((s.fx_level - 1.0).abs() < 1e-5);
        s.update_fx(0.3);
        assert_eq!(s.fx_level, 0.0);
    }

    #[test]
    fn the_real_actions_load() {
        let _lock = crate::i18n::TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let data = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");
        crate::i18n::init(&data, "en");
        let c = Cutscenes::load(&data);
        let script = Scripting::new(&data).expect("the scripts load");
        assert!(!c.action(29, Some(&script)).is_empty(), "action 29 (Nigel wakes up in Massan)");
        assert!(!c.text(5).is_empty());
        assert!(c.playback(0, &script).map_or(false, |p| !p.is_empty()));
        assert!(!c.credits.is_empty());
    }
}
