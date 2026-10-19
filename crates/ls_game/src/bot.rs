//! A test player: LS_BOT="148,149,151,..." walks Nigel through the rooms of the list, in order, by pressing the keys like a person would (it finds its way
//! over the cells of the room with a breadth-first search: down any step, up one step with a jump, to the warp that leads to the next room or to the pit that
//! falls into it), reads the boxes (the action key) and writes what it does in the console. To check a route of the story: LS_NEWGAME=1 LS_BOT=....

use crate::game::Message;
use crate::scripting::Scripting;
use crate::shops::Dialogue;
use crate::{GameFrozen, Player, RoomRes};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Default)]
pub struct BotState {
    route: Vec<i32>,
    started: bool,
    room: i32,
    path: Vec<(i32, i32)>,
    avoid: HashSet<(i32, i32)>,
    last_pos: (f32, f32),
    still: f32,
    log_t: f32,
    press_t: f32,
    replans: u32,
    /// the bot pressed a direction during the last frame (standing still then means blocked)
    intent: bool,
    done: bool,
    bumps: Bumps,
    /// the puzzle of the crate: 0 none, 1 going to the crate, 2 carrying it to its place, 3 put down
    puzzle: u8,
    crate_cell: (i32, i32),
    drop: Option<((i32, i32), (i32, i32), (i32, i32))>,
}

/// the cells of the warps that lead somewhere else than `next`: the bot must not step on them
fn other_warps(room: &RoomRes, next: i32) -> HashSet<(i32, i32)> {
    let mut g = HashSet::new();
    for w in room.room.file.warps.iter().filter(|w| w.target_room != next) {
        for dx in 0..w.area[2].max(1) {
            for dy in 0..w.area[3].max(1) {
                g.insert((w.area[0] + dx, w.area[1] + dy));
            }
        }
    }
    g
}

type Bumps = HashMap<(i32, i32), i32>;

/// height of a cell, with the crates the bot has put down
fn height(room: &RoomRes, bumps: &Bumps, p: (i32, i32)) -> Option<i32> {
    room.room.cell_at_world(p.0, p.1).map(|c| c.height as i32 + bumps.get(&p).copied().unwrap_or(0))
}

fn goals(room: &RoomRes, next: i32) -> HashSet<(i32, i32)> {
    let f = &room.room.file;
    let mut g = HashSet::new();
    for w in f.warps.iter().filter(|w| w.target_room == next) {
        for dx in 0..w.area[2].max(1) {
            for dy in 0..w.area[3].max(1) {
                g.insert((w.area[0] + dx, w.area[1] + dy));
            }
        }
    }
    if f.fall_destination == Some(next) {
        let off = f.map.cell_offset;
        let [sx, sy] = f.map.heightmap.size;
        for x in 0..sx {
            for y in 0..sy {
                if room.room.cell(x, y).is_some_and(|c| c.floor_type == 7) {
                    g.insert((x + off, y + off));
                }
            }
        }
    }
    g
}

/// The cells from `from` to one of the goals; None when there is no way.
fn search(room: &RoomRes, bumps: &Bumps, from: (i32, i32), goals: &HashSet<(i32, i32)>, avoid: &HashSet<(i32, i32)>) -> Option<Vec<(i32, i32)>> {
    let cell = |p: (i32, i32)| room.room.cell_at_world(p.0, p.1);
    let mut prev: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(from);
    prev.insert(from, from);
    while let Some(p) = queue.pop_front() {
        if goals.contains(&p) {
            let mut path = vec![p];
            let mut q = p;
            while q != from {
                q = prev[&q];
                path.push(q);
            }
            path.reverse();
            return Some(path);
        }
        let Some(h) = height(room, bumps, p) else { continue };
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let n = (p.0 + dx, p.1 + dy);
            if prev.contains_key(&n) || avoid.contains(&n) {
                continue;
            }
            let Some(nc) = cell(n) else { continue };
            if nc.blocked() || (nc.floor_type == 7 && !goals.contains(&n)) {
                continue;
            }
            let nh = height(room, bumps, n).unwrap_or(0);
            if nh > h + 1 || nh < h - 6 {
                continue;
            }
            prev.insert(n, p);
            queue.push_back(n);
        }
    }
    None
}

fn reachable(room: &RoomRes, bumps: &Bumps, from: (i32, i32), goals: &HashSet<(i32, i32)>, avoid: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    let cell = |p: (i32, i32)| room.room.cell_at_world(p.0, p.1);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(from);
    seen.insert(from);
    while let Some(p) = queue.pop_front() {
        let Some(h) = height(room, bumps, p) else { continue };
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let n = (p.0 + dx, p.1 + dy);
            if seen.contains(&n) || avoid.contains(&n) {
                continue;
            }
            let Some(nc) = cell(n) else { continue };
            if nc.blocked() || (nc.floor_type == 7 && !goals.contains(&n)) {
                continue;
            }
            let nh = height(room, bumps, n).unwrap_or(0);
            if nh > h + 1 || nh < h - 6 {
                continue;
            }
            seen.insert(n);
            queue.push_back(n);
        }
    }
    seen.into_iter().collect()
}

/// A step of two levels is too high for a jump: a crate put on the lower cell makes it. Finds where to stand (two cells before the crate's place, so that he
/// arrives facing it), the direction, and the cell for the crate; the way to the exit must exist once the crate is there.
fn plan_drop(room: &RoomRes, bumps: &Bumps, from: (i32, i32), goals: &HashSet<(i32, i32)>, avoid: &HashSet<(i32, i32)>) -> Option<((i32, i32), (i32, i32), (i32, i32))> {
    let reach: HashSet<(i32, i32)> = reachable(room, bumps, from, goals, avoid).into_iter().collect();
    let mut cells: Vec<_> = reach.iter().copied().collect();
    cells.sort();
    for r in cells {
        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (n, s, s2) = ((r.0 + d.0, r.1 + d.1), (r.0 - d.0, r.1 - d.1), (r.0 - 2 * d.0, r.1 - 2 * d.1));
            if reach.contains(&n) || !reach.contains(&s) || !reach.contains(&s2) {
                continue;
            }
            let Some(nc) = room.room.cell_at_world(n.0, n.1) else { continue };
            if nc.blocked() || nc.floor_type == 7 {
                continue;
            }
            let (hr, hs, hn) = (height(room, bumps, r).unwrap_or(0), height(room, bumps, s).unwrap_or(0), height(room, bumps, n).unwrap_or(0));
            if hn != hr + 2 || hr > hs + 1 {
                continue;
            }
            let mut b = bumps.clone();
            *b.entry(r).or_insert(0) += 1;
            if search(room, &b, s, goals, avoid).is_some() {
                return Some((s2, d, r));
            }
        }
    }
    None
}

/// The two keys of a direction (the original's pad: two keys together are a diagonal of the screen): north-west up + left, north-east up + right, south-east
/// down + right, south-west down + left.
fn keys_of(d: (i32, i32), original: bool, config: &crate::config::Config) -> Vec<KeyCode> {
    let (up, down, left, right) = (bound(config, "up"), bound(config, "down"), bound(config, "left"), bound(config, "right"));
    if original {
        match d {
            (-1, 0) => vec![up, left],
            (1, 0) => vec![down, right],
            (0, 1) => vec![down, left],
            _ => vec![up, right],
        }
    } else {
        // one fixed key per direction
        vec![match d {
            (-1, 0) => up,
            (1, 0) => down,
            (0, 1) => left,
            _ => right,
        }]
    }
}

/// The first key bound to a control in landstalker.ini: the bot presses the player's own keys.
fn bound(config: &crate::config::Config, name: &str) -> KeyCode {
    config.keys.get(name).and_then(|v| v.first()).copied().unwrap_or(KeyCode::Pause)
}

/// What stops Nigel on the way to `target`: the cells around it (height, floor type) and the solid entities near him.
fn describe_block(room: &RoomRes, script: &Scripting, p: &Player, target: Option<(i32, i32)>) -> String {
    let here = (p.x.floor() as i32, p.y.floor() as i32);
    let cell = |c: (i32, i32)| room.room.cell_at_world(c.0, c.1).map_or("none".to_string(), |x| if x.blocked() { "wall".to_string() } else { format!("h{} type {:#x}", x.height, x.floor_type) });
    let mut text = format!("here {} ", cell(here));
    if let Some(t) = target {
        text.push_str(&format!("target {} ", cell(t)));
    }
    let near: Vec<String> = script
        .entities()
        .iter()
        .filter(|e| e.visible && e.solid && (e.x - p.x).abs() < 2.0 && (e.y - p.y).abs() < 2.0)
        .map(|e| format!("#{} kind {} at ({:.1}, {:.1}) z {:.1} top {:.1}", e.index, e.kind, e.x, e.y, e.z, e.z + e.hheight))
        .collect();
    if !near.is_empty() {
        text.push_str(&format!("near: {}", near.join("; ")));
    }
    text
}

#[allow(clippy::too_many_arguments)]
pub fn bot(
    time: Res<Time>,
    mut st: Local<BotState>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    room: Res<RoomRes>,
    dlg: Res<Dialogue>,
    frozen: Res<GameFrozen>,
    cut: Res<crate::cutscene::Cutscene>,
    config: Res<crate::config::Config>,
    msg: Res<Message>,
    lock: Res<crate::ControlLock>,
    script: NonSend<Scripting>,
    players: Query<&Player>,
    mut exit: MessageWriter<AppExit>,
) {
    if !st.started {
        st.started = true;
        st.route = std::env::var("LS_BOT").unwrap_or_default().split(',').filter_map(|v| v.trim().parse().ok()).collect();
    }
    if st.route.is_empty() || st.done {
        return;
    }
    let original = config.original_directions;
    let dt = time.delta_secs();
    st.log_t += dt;
    st.press_t += dt;
    let (action, jump) = (bound(&config, "action"), bound(&config, "jump"));
    let all = [bound(&config, "up"), bound(&config, "down"), bound(&config, "left"), bound(&config, "right"), jump, action, bound(&config, "attack")];
    for k in all {
        keys.release(k);
    }
    // a conversation, a scene: read on (the action key), skip the intro (the sword key)
    if dlg.open || frozen.0 || cut.active() {
        // (the intro cannot be skipped: it is waited out)
        if st.press_t > 0.4 && !cut.intro {
            st.press_t = 0.0;
            keys.press(action);
        }
        return;
    }
    // the controls are off (the raft that carries him from room 163 to 169): nothing to press, he is taken
    if lock.0.is_some() {
        st.path.clear();
        st.still = 0.0;
        if st.log_t > 3.0 {
            st.log_t = 0.0;
            println!("[bot] controls off ({}): waiting", lock.0.as_deref().unwrap_or(""));
        }
        return;
    }
    let Ok(p) = players.single() else { return };
    let id = room.room.file.id;
    if id != st.room {
        println!("[bot] room {} at ({:.1}, {:.1}) z {:.1}", id, p.x, p.y, p.z);
        st.room = id;
        st.path.clear();
        st.avoid.clear();
        st.still = 0.0;
        st.replans = 0;
        st.bumps.clear();
        st.puzzle = 0;
        st.drop = None;
        if Some(&id) == st.route.last() {
            println!("[bot] reached the end of the route: room {}", id);
            st.done = true;
            return;
        }
    }
    let Some(i) = st.route.iter().rposition(|r| *r == id) else {
        if st.log_t > 3.0 {
            st.log_t = 0.0;
            println!("[bot] room {} is not on the route", id);
        }
        return;
    };
    let next = st.route[i + 1];
    let here = (p.x.floor() as i32, p.y.floor() as i32);
    // pressing a direction and not moving: blocked. Say what is in the way at once, forget that way after half a second, give up after a few tries
    if (p.x - st.last_pos.0).abs() + (p.y - st.last_pos.1).abs() < 0.02 {
        if st.intent {
            st.still += dt;
        }
    } else {
        st.still = 0.0;
        st.last_pos = (p.x, p.y);
    }
    if st.still > 0.8 && !(st.puzzle == 2 && st.drop.is_some()) && p.carrying.is_none() {
        st.still = 0.0;
        let target = st.path.iter().position(|c| *c == here).and_then(|i| st.path.get(i + 1)).copied();
        if let Some(n) = target {
            st.avoid.insert(n);
        }
        st.path.clear();
        st.replans += 1;
        println!("[bot] BLOCKED at ({:.2}, {:.2}) z {:.1} in room {} going to {:?}: {} (try {})", p.x, p.y, p.z, id, target, describe_block(&room, &script, &p, target), st.replans);
        if st.replans >= 12 {
            println!("[bot] GIVING UP in room {}: blocked {} times", id, st.replans);
            exit.write(AppExit::from_code(2));
            return;
        }
    }
    st.intent = false;
    let g = goals(&room, next);
    let mut avoid = st.avoid.clone();
    avoid.extend(other_warps(&room, next).into_iter().filter(|c| !g.contains(c) && *c != here));
    // the crate: lifted (action key in front of it), carried to the foot of the step that is too high, put down (action key, no direction), and climbed
    if st.puzzle == 1 && p.carrying.is_some() {
        match plan_drop(&room, &st.bumps, here, &g, &avoid) {
            Some(plan) => {
                println!("[bot] room {}: carries the crate to {:?} (stands on {:?})", id, plan.2, plan.0);
                st.drop = Some(plan);
                st.puzzle = 2;
            }
            None => {
                println!("[bot] room {}: no place found for the crate", id);
                st.puzzle = 4;
            }
        }
        st.path.clear();
    }
    if st.puzzle == 2 && p.carrying.is_none() && p.lift_phase == 0 {
        if let Some((_, _, r)) = st.drop {
            *st.bumps.entry(r).or_insert(0) += 1;
            println!("[bot] room {}: crate put down on {:?}", id, r);
        }
        st.puzzle = 3;
        st.drop = None;
        st.path.clear();
    }
    let mut goalset = g.clone();
    if let Some((s2, d, _)) = st.drop.filter(|_| st.puzzle == 2) {
        let s = (s2.0 + d.0, s2.1 + d.1);
        let face = match d { (-1, 0) => 0, (0, -1) => 1, (1, 0) => 2, _ => 3 };
        // (the crate goes down one cell ahead of where he stands: he must be on the middle line of the row and at the middle of the cell, not at its edge:
        // a crate put down off the line leaves him a corner in the wall when he climbs it)
        let lateral = if d.0 != 0 { p.y - (s.1 as f32 + 0.5) } else { p.x - (s.0 as f32 + 0.5) };
        if (here == s || here == s2) && lateral.abs() > 0.06 && p.grounded && p.lift_phase == 0 {
            st.intent = true;
            let fix = if d.0 != 0 { if lateral > 0.0 { (0, -1) } else { (0, 1) } } else if lateral > 0.0 { (-1, 0) } else { (1, 0) };
            for k in keys_of(fix, original, &config) {
                keys.press(k);
            }
            return;
        }
        let along = (p.x - (s.0 as f32 + 0.5)) * d.0 as f32 + (p.y - (s.1 as f32 + 0.5)) * d.1 as f32;
        if here == s && p.facing & 3 == face && along < -0.12 && p.carrying.is_some() && p.lift_phase == 0 {
            st.intent = true;
            for k in keys_of(d, original, &config) {
                keys.press(k);
            }
            return;
        }
        if here == s && p.facing & 3 == face {
            // in front of the place: down it goes (no direction held)
            if p.grounded && p.carrying.is_some() && p.lift_phase == 0 && st.press_t > 0.3 {
                st.press_t = 0.0;
                keys.press(action);
            }
            return;
        }
        if p.carrying.is_none() || p.lift_phase != 0 {
            return;
        }
        if here == s2 {
            st.intent = true;
            for k in keys_of(d, original, &config) {
                keys.press(k);
            }
            if p.grounded && height(&room, &st.bumps, s) > height(&room, &st.bumps, s2) && st.press_t > 0.3 {
                st.press_t = 0.0;
                keys.press(jump);
            }
            return;
        }
        goalset = HashSet::from([s2]);
    } else if st.puzzle == 1 {
        goalset = HashSet::from([st.crate_cell]);
        let near = script.entities().iter().any(|e| e.kind == 22 && !e.carried && (e.x - p.x).hypot(e.y - p.y) < 1.3);
        if near && p.grounded && st.press_t > 0.3 {
            st.press_t = 0.0;
            keys.press(action);
        }
    }
    if st.path.is_empty() || !st.path.contains(&here) {
        match search(&room, &st.bumps, here, &goalset, &avoid) {
            Some(path) => st.path = path,
            None => {
                if st.puzzle == 0 {
                    // no way: maybe a crate to carry
                    let crate_at = script.entities().iter().filter(|e| e.kind == 22 && e.pickable && e.visible && !e.carried).min_by_key(|e| ((e.x - p.x).abs() + (e.y - p.y).abs()) as i32).map(|e| (e.x.floor() as i32, e.y.floor() as i32));
                    if let Some(c) = crate_at {
                        println!("[bot] room {}: no way up, a crate at {:?}: lifts it", id, c);
                        st.crate_cell = c;
                        st.puzzle = 1;
                        return;
                    }
                }
                // no way on foot: a raft in the room takes him (he steps on it, the script carries him)
                let raft = script.entities().iter().find(|e| e.kind == 53 && e.visible).map(|e| (e.x, e.y));
                if let Some((rx, ry)) = raft {
                    let (dx, dy) = (rx - p.x, ry - p.y);
                    if dx.abs() + dy.abs() > 0.6 {
                        for k in keys_of(if dx.abs() >= dy.abs() { if dx > 0.0 { (1, 0) } else { (-1, 0) } } else if dy > 0.0 { (0, 1) } else { (0, -1) }, original, &config) {
                            keys.press(k);
                        }
                    }
                    if st.log_t > 3.0 {
                        st.log_t = 0.0;
                        println!("[bot] room {}: steps on the raft at ({:.1}, {:.1}), he is at ({:.1}, {:.1}) z {:.1}", id, rx, ry, p.x, p.y, p.z);
                    }
                    return;
                }
                if st.log_t > 3.0 {
                    st.log_t = 0.0;
                    println!("[bot] room {}: no way from ({:.1}, {:.1}) to room {} (goals {}, waiting: a platform, a raft?)", id, p.x, p.y, next, g.len());
                    let _ = &msg;
                }
                return;
            }
        }
    }
    let at = st.path.iter().position(|c| *c == here).unwrap_or(0);
    let Some(target) = st.path.get(at + 1).or(st.path.last()).copied() else { return };
    let (tx, ty) = (target.0 as f32 + 0.5, target.1 as f32 + 0.5);
    let (dx, dy) = (tx - p.x, ty - p.y);
    // keys: up -x, down +x, left +y, right -y. The way is followed along its middle: the exits are narrow, so the error across the direction of the
    // step is corrected first (a corner of the wall stops him when he is a little off)
    let (sx, sy) = (target.0 - here.0, target.1 - here.1);
    let (cx, cy) = (here.0 as f32 + 0.5, here.1 as f32 + 0.5);
    let along_x = sx != 0;
    let lateral = if along_x { p.y - cy } else { p.x - cx };
    let towards = |dx: f32, dy: f32| if dx.abs() >= dy.abs() { if dx > 0.0 { (1, 0) } else { (-1, 0) } } else if dy > 0.0 { (0, 1) } else { (0, -1) };
    let key = if sx != 0 || sy != 0 {
        if lateral.abs() > 0.12 && p.grounded {
            if along_x { towards(0.0, -lateral) } else { towards(-lateral, 0.0) }
        } else {
            towards(dx, dy)
        }
    } else {
        towards(dx, dy)
    };
    if dx.abs() + dy.abs() > 0.05 {
        st.intent = true;
        for k in keys_of(key, original, &config) {
            keys.press(k);
        }
    }
    // a jump only for a real step up, close to it, and not again before the last one is over
    let higher = height(&room, &st.bumps, target).zip(height(&room, &st.bumps, here)).is_some_and(|(n, h)| n > h);
    let stairs = room.room.cell_at_world(target.0, target.1).is_some_and(|c| c.floor_type == 5) || room.room.cell_at_world(here.0, here.1).is_some_and(|c| c.floor_type == 5);
    let close = (dx.abs() + dy.abs()) < 1.6;
    if std::env::var("LS_BOT_LOG").is_ok() && higher {
        println!("[bot] step up from {:?} to {:?}: stairs {} close {} grounded {} wait {:.2}", here, target, stairs, close, p.grounded, st.press_t);
    }
    if higher && !stairs && close && p.grounded && st.press_t > 0.35 {
        st.press_t = 0.0;
        keys.press(jump);
    }
    if st.log_t > 5.0 {
        st.log_t = 0.0;
        println!("[bot] room {}: ({:.1}, {:.1}) z {:.1} -> room {} via cell {:?}", id, p.x, p.y, p.z, next, target);
    }
}

/// Record mode: writes what a person does, to be read afterwards. One file per room (recordings/room_<id>.txt, a new recording replaces the old one), and it
/// starts only with the first key pressed in the room. Two modes: `Room` records the room where it was asked and stops when the room changes, `Continuous`
/// goes on from room to room until it is stopped (each room in its own file). One line per frame (what he pressed, where he is, what moves): `<t seconds> room <id> pos x y z <ground|air> carrying <n> keys <list> [| #index kind x y z ...]`
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum RecMode {
    #[default]
    Off,
    Room,
    Continuous,
}

#[derive(Resource, Default)]
pub struct Recorder {
    pub mode: RecMode,
    /// the room being recorded (Room mode) or waiting for the first key
    room: i32,
    file: Option<std::fs::File>,
    pub path: String,
    keys: String,
    t: f32,
    since: f32,
    checked_env: bool,
}

impl Recorder {
    pub fn active(&self) -> bool {
        self.mode != RecMode::Off
    }

    /// true once the first key was pressed and the file is being written
    pub fn writing(&self) -> bool {
        self.file.is_some()
    }

    pub fn start(&mut self, mode: RecMode, room: i32) {
        self.stop();
        self.mode = mode;
        self.room = room;
        println!("[record] {:?}: waiting for the first key in room {}", mode, room);
    }

    pub fn stop(&mut self) {
        if self.file.take().is_some() {
            println!("[record] stopped: {}", self.path);
        }
        self.mode = RecMode::Off;
    }

    fn close_room(&mut self) {
        if self.file.take().is_some() {
            println!("[record] room finished: {}", self.path);
        }
    }
}

pub fn record(time: Res<Time>, ctl: Res<crate::config::Controls>, room: Res<RoomRes>, script: NonSend<Scripting>, players: Query<&Player>, mut rec: ResMut<Recorder>) {
    use std::io::Write;
    let id = room.room.file.id;
    if !rec.checked_env {
        rec.checked_env = true;
        if std::env::var("LS_RECORD").is_ok() {
            rec.start(RecMode::Continuous, id);
        }
    }
    if rec.mode == RecMode::Off {
        return;
    }
    // the room changed
    if id != rec.room {
        rec.close_room();
        if rec.mode == RecMode::Room {
            rec.mode = RecMode::Off;
            return;
        }
        rec.room = id;
        rec.keys.clear();
    }
    let Ok(p) = players.single() else { return };
    // what the game hears (keyboard and game pad together): the four directions and the buttons, by their game names
    let mut names: Vec<&str> = Vec::new();
    for (b, n) in [(&ctl.dir[0], "up"), (&ctl.dir[1], "down"), (&ctl.dir[2], "left"), (&ctl.dir[3], "right"), (&ctl.jump, "jump"), (&ctl.attack, "attack"), (&ctl.action, "action"), (&ctl.menu, "menu"), (&ctl.quick_previous, "quick_previous"), (&ctl.quick_use, "quick_use"), (&ctl.quick_next, "quick_next"), (&ctl.use_key, "use_key"), (&ctl.equipment, "equipment"), (&ctl.ekeeke, "ekeeke")] {
        if b.pressed {
            names.push(n);
        }
    }
    let list = if names.is_empty() { "-".to_string() } else { names.join("+") };
    if rec.file.is_none() {
        // waiting for the first key of the room
        if list == "-" {
            return;
        }
        // in the project folder (not where the program was started from)
        let dir = std::path::Path::new(&std::env::var("BEVY_ASSET_ROOT").unwrap_or_default()).join("recordings");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!("room_{}.txt", id)).to_string_lossy().to_string();
        match std::fs::File::create(&path) {
            Ok(f) => {
                println!("[record] writing {}", path);
                rec.file = Some(f);
                rec.path = path;
                rec.t = 0.0;
                rec.since = 1.0;
                rec.keys = "?".into();
            }
            Err(e) => {
                println!("[record] cannot write {}: {}", path, e);
                rec.mode = RecMode::Off;
                return;
            }
        }
    } else {
        rec.t += time.delta_secs();
        rec.since += time.delta_secs();
    }
    if list == rec.keys && rec.since < 0.5 {
        return;
    }
    let periodic = rec.since >= 0.5;
    let first = rec.keys == "?";
    rec.since = 0.0;
    let mut line = format!("{:.2} room {} pos {:.2} {:.2} {:.2} {} carrying {} keys {}", rec.t, id, p.x, p.y, p.z, if p.grounded { "ground" } else { "air" }, p.carrying.map_or(-1, |i| i as i32), list);
    if periodic || first {
        for e in script.entities().iter().filter(|e| e.visible && (first || e.moved || e.carried)) {
            line.push_str(&format!(" | #{} kind {} {:.2} {:.2} {:.2}", e.index, e.kind, e.x, e.y, e.z));
        }
    }
    rec.keys = list;
    if let Some(f) = rec.file.as_mut() {
        let _ = writeln!(f, "{}", line);
        let _ = f.flush();
    }
}
