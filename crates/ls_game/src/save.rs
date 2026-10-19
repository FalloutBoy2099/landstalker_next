//! Saving and loading the adventure (landstalker.sav, next to landstalker.ini): what Nigel has and carries, what he did (the story
//! flags, the chests opened, the enemies killed), and where he is. F5 saves, F9 loads (landstalker.ini, [input] save / load); the priest
//! saves too when he records the adventure.

use crate::combat::Combat;
use crate::config::Controls;
use crate::game::{GameState, Message};
use crate::audio::SfxQueue;
use crate::scripting::Scripting;
use crate::{Arrival, DataDir, GameFrozen, Player, RoomRequest, RoomRes};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct SaveData {
    pub version: u32,
    pub room: i32,
    pub x: f32,
    pub y: f32,
    pub gold: i32,
    pub items: Vec<(i32, i32)>,
    pub health: i32,
    pub max_health: i32,
    pub sword: i32,
    pub armour: i32,
    pub boots: i32,
    /// the ring worn (saves from before the rings have none)
    #[serde(default = "no_ring")]
    pub ring: i32,
    pub status: u8,
    pub progress: i32,
    pub flags: Vec<i32>,
    pub opened: Vec<(i32, usize)>,
    pub collected: Vec<(i32, usize)>,
    pub enemy_health: Vec<(i32, usize, i32)>,
    pub church: Option<(i32, f32, f32)>,
}

/// Where the save is: next to the data folder (the project's root).
pub fn path_of(data: &Path) -> PathBuf {
    data.parent().unwrap_or(Path::new(".")).join("landstalker.sav")
}

/// The adventure as it is: in `room`, at (x, y).
fn no_ring() -> i32 {
    -1
}

pub fn snapshot(game: &GameState, script: &Scripting, room: i32, x: f32, y: f32) -> SaveData {
    let mut items: Vec<(i32, i32)> = game.items.iter().filter(|(_, n)| **n > 0).map(|(i, n)| (*i, *n)).collect();
    items.sort();
    let mut flags = script.flags();
    flags.sort();
    let mut opened: Vec<_> = game.opened.iter().copied().collect();
    opened.sort();
    let mut collected: Vec<_> = game.collected.iter().copied().collect();
    collected.sort();
    let mut enemy_health: Vec<_> = game.enemy_health.iter().map(|((r, i), h)| (*r, *i, *h)).collect();
    enemy_health.sort();
    SaveData {
        version: 1,
        room,
        x,
        y,
        gold: game.gold,
        items,
        health: game.health,
        max_health: game.max_health,
        sword: game.sword,
        armour: game.armour,
        boots: game.boots,
        ring: game.ring,
        status: game.status,
        progress: game.progress,
        flags,
        opened,
        collected,
        enemy_health,
        church: game.church,
    }
}

/// Puts a saved adventure back into the game state and the story flags.
pub fn restore(save: &SaveData, game: &mut GameState, script: &Scripting) {
    game.gold = save.gold;
    game.items = save.items.iter().copied().collect();
    game.health = save.health;
    game.max_health = save.max_health;
    game.sword = save.sword;
    game.armour = save.armour;
    game.boots = save.boots;
    game.ring = if crate::game::is_ring(save.ring) { save.ring } else { -1 };
    game.status = save.status;
    game.progress = save.progress;
    game.opened = save.opened.iter().copied().collect();
    game.collected = save.collected.iter().copied().collect();
    game.enemy_health = save.enemy_health.iter().map(|(r, i, h)| ((*r, *i), *h)).collect();
    game.church = save.church;
    script.replace_flags(&save.flags);
}

pub fn write(path: &Path, save: &SaveData) -> bool {
    serde_json::to_string_pretty(save).ok().and_then(|t| std::fs::write(path, t).ok()).is_some()
}

pub fn read(path: &Path) -> Option<SaveData> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

/// The buttons Save / Load of the debug panel ask for the same as the keys.
#[derive(Resource, Default)]
pub struct SaveRequest {
    pub save: bool,
    pub load: bool,
}

/// F5 saves, F9 loads.
#[allow(clippy::too_many_arguments)]
pub fn save_load(
    ctl: Res<Controls>,
    data: Res<DataDir>,
    frozen: Res<GameFrozen>,
    mut game: ResMut<GameState>,
    mut combat: ResMut<Combat>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    mut request: ResMut<RoomRequest>,
    mut arrival: ResMut<Arrival>,
    mut msg: ResMut<Message>,
    mut sfx: ResMut<SfxQueue>,
    cut: Res<crate::cutscene::Cutscene>,
    mut request_button: ResMut<SaveRequest>,
    players: Query<&Player>,
) {
    let (want_save, want_load) = (ctl.save.just || std::mem::take(&mut request_button.save), ctl.load.just || std::mem::take(&mut request_button.load));
    if frozen.0 || cut.active() || (!want_save && !want_load) {
        return;
    }
    let path = path_of(&data.0);
    if want_save {
        let Ok(p) = players.single() else { return };
        if combat.dead {
            return;
        }
        let ok = write(&path, &snapshot(&game, &script, room.room.file.id, p.x, p.y));
        sfx.play(if ok { 0x43 } else { 0x44 });
        msg.show(crate::i18n::t(if ok { "messages.save.saved" } else { "messages.save.save_failed" }));
    } else {
        let Some(save) = read(&path) else {
            sfx.play(0x44);
            msg.show(crate::i18n::t("messages.save.no_save"));
            return;
        };
        restore(&save, &mut game, &script);
        // back on his feet where he saved
        combat.revive();
        request.0 = Some(save.room);
        arrival.0 = Some(Vec2::new(save.x, save.y));
        msg.show(crate::i18n::t("messages.save.loaded"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_saved_adventure_reads_back() {
        let mut g = GameState::default();
        g.gold = 123;
        g.add_item(0);
        g.add_item(0);
        g.add_item(50);
        g.sword = 2;
        g.add_status(1);
        g.opened.insert((18, 3));
        g.collected.insert((0, 2));
        g.church = Some((597, 27.5, 25.5));
        g.damage(1.5);
        let script = Scripting::new(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data")).unwrap();
        script.set_flag(52);
        let save = snapshot(&g, &script, 18, 30.5, 31.5);
        let dir = std::env::temp_dir().join("ls_save_test.sav");
        assert!(write(&dir, &save));
        let back = read(&dir).unwrap();
        assert_eq!(back, save);
        let mut h = GameState::default();
        restore(&back, &mut h, &script);
        assert_eq!((h.gold, h.item_count(0), h.item_count(50), h.sword, h.status, h.health), (123, 2, 1, 2, 2, g.health));
        assert!(h.opened.contains(&(18, 3)) && h.collected.contains(&(0, 2)) && h.church == g.church);
        assert!(script.flag(52));
        let _ = std::fs::remove_file(dir);
    }
}
