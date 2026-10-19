//! The sword, the enemies' blows, the floors that hurt, what the dead leave behind. The rules are the C++ engine's
//! (RoomView::Attack, ApplySwingHit, HurtPlayer, UpdateCombat, UpdateFloorEffects, CheckPickups) with the original game's damage
//! formulas (game.rs). The enemies' behaviour is in data/scripts/ai.lua; here is what Nigel does to them and they to him.

use crate::audio::{Sound, SfxQueue};
use crate::config::Controls;
use crate::game::{GameState, Message, BOOTS_FIREPROOF, BOOTS_HEALING, BOOTS_IRON, STATUS_CONFUSION, STATUS_CURSE, STATUS_PARALYSIS, STATUS_POISON};
use crate::scripting::{EntSnapshot, Scripting};
use crate::{ChestOpen, GameFrozen, Player, RoomRes, WorldRoot};
use bevy::prelude::*;
use std::collections::HashMap;

pub const ATTACK_COOLDOWN: f32 = 0.4;
pub const SWING_DURATION: f32 = 0.3;
/// the blow lands when the arc is out: the second frame of four
pub const SWING_HIT_TIME: f32 = 0.10;
pub const ATTACK_REACH: f32 = 0.9;
pub const KNOCKBACK: f32 = 0.6;
pub const HURT_POSE_TIME: f32 = 0.3;
pub const HIT_INVULNERABILITY: f32 = 1.0;
pub const PLAYER_KNOCK_SPEED: f32 = 6.0;
pub const PLAYER_KNOCK_TIME: f32 = 0.14;
pub const DEATH_ANIM_TIME: f32 = 1.0;
pub const SPARK_TIME: f32 = 0.25;
const PLAYER_RADIUS: f32 = 0.4;
const PLAYER_HEIGHT: f32 = 2.0;
const BOSSES: [i32; 11] = [112, 124, 133, 157, 159, 160, 162, 165, 167, 170, 171];

/// What an enemy is worth, from the entity table (entities.yaml).
#[derive(Clone, Copy, Debug, Default)]
pub struct EnemyStats {
    pub health: i32,
    pub defence: i32,
    pub attack: i32,
    pub gold: i32,
}

struct Spark {
    x: f32,
    y: f32,
    z: f32,
    big: bool,
    /// 0 white, 1 orange (Nigel hurt), 2 ice, 3 fire, 4 thunder, 5 gaia
    tint: u8,
}

/// The charge of the sword is full at this value (the original's 0x3200); it rises 32 a frame at 60 Hz (6.7 s) and a swing empties it.
pub const CHARGE_FULL: f32 = 12800.0;
/// the frame of the swing (11 of 60 a second) when the Ice Sword's wave leaves and the Gaia Sword strikes
const MAGIC_TIME: f32 = 11.0 / 60.0;
const WAVE_SPEED: f32 = 8.0;
const WAVE_RANGE: f32 = 8.0;

/// The Ice Sword's wave: it flies along the blow and hurts what it meets, once each.
struct Wave {
    x: f32,
    y: f32,
    z: f32,
    dx: f32,
    dy: f32,
    travelled: f32,
    hit: Vec<usize>,
    spark_t: f32,
}

#[derive(Resource, Default)]
pub struct Combat {
    pub stats: HashMap<i32, EnemyStats>,
    cooldown: f32,
    /// seconds into the swing of the sword
    pub swing: Option<f32>,
    swing_facing: u8,
    hit_pending: bool,
    /// protected after a blow: Nigel blinks
    pub invulnerable: f32,
    pub hurt: f32,
    /// thrown back: velocity and seconds left
    pub knock: (f32, f32, f32),
    pub dead: bool,
    pub death_t: f32,
    /// the screen shakes: seconds left, total, amplitude (pixels of the original picture)
    pub shake: (f32, f32, f32),
    new_sparks: Vec<Spark>,
    room_seen: i32,
    /// poison: seconds walked since the last point of health it took, and the points it still takes
    poison_walk: f32,
    poison_ticks: i32,
    /// paralysis: seconds left (it wears off, or Nigel struggles free with the sword button) and the tries so far
    pub paralysis: f32,
    paralysis_presses: usize,
    /// the Healing Boots: seconds walked since the last point
    heal_acc: f32,
    synced: bool,
    /// the charge of the sword (0 .. CHARGE_FULL); the sword of the swing under way when it was charged (0: an ordinary swing);
    /// the Ice wave or the Gaia blow still to come in this swing
    pub charge: f32,
    swing_magic: i32,
    magic_pending: bool,
    waves: Vec<Wave>,
}

impl Combat {
    /// Nigel cannot walk, jump or act: swinging the sword, thrown back, dead.
    pub fn locked(&self) -> bool {
        self.swing.is_some() || self.knock.2 > 0.0 || self.dead || self.paralysis > 0.0
    }

    /// An ailment strikes (nothing happens if he has it already).
    pub fn inflict(&mut self, game: &mut GameState, sfx: &mut SfxQueue, msg: &mut Message, status: u8) {
        if game.has_status(status) || self.dead || game.ring_protects(status) {
            return;
        }
        game.add_status(status);
        sfx.play(0x7A); // SND_Poison
        match status {
            STATUS_POISON => {
                self.poison_walk = 0.0;
                self.poison_ticks = 8;
                msg.show(crate::i18n::t("messages.status.poisoned"));
            }
            STATUS_CONFUSION => msg.show(crate::i18n::t("messages.status.confused")),
            STATUS_PARALYSIS => {
                self.paralysis = 20.0;
                self.paralysis_presses = 0;
                msg.show(crate::i18n::t("messages.status.paralysed"));
            }
            _ => msg.show(crate::i18n::t("messages.status.cursed")),
        }
    }

    /// The timers of the ailments Nigel starts with (a test aid, a loaded game).
    pub fn sync_status(&mut self, game: &GameState) {
        self.poison_ticks = if game.has_status(STATUS_POISON) { 8 } else { 0 };
        self.paralysis = if game.has_status(STATUS_PARALYSIS) { 20.0 } else { 0.0 };
    }

    /// Back on his feet (a loaded game, the end of the death): nothing under way, nothing hurting.
    pub fn revive(&mut self) {
        self.dead = false;
        self.death_t = 0.0;
        self.hurt = 0.0;
        self.knock.2 = 0.0;
        self.swing = None;
        self.paralysis = 0.0;
        self.poison_ticks = 0;
        self.invulnerable = HIT_INVULNERABILITY;
        self.charge = 0.0;
        self.waves.clear();
    }

    /// The end of the death (the CONTINUE of the game over menu): back on his feet with full health, in the church where the adventure
    /// was recorded, else where he fell.
    pub fn respawn(&mut self, game: &mut GameState, request: &mut crate::RoomRequest, arrival: &mut crate::Arrival, msg: &mut Message) {
        game.health = game.max_health;
        game.status = 0;
        if let Some((room, x, y)) = game.church {
            request.0 = Some(room);
            arrival.0 = Some(Vec2::new(x, y));
        }
        self.revive();
        msg.time = 0.0;
    }

    /// A burst of sparks over an entity (something appears).
    pub fn burst_at(&mut self, script: &Scripting, index: usize) {
        if let Some((x, y, z)) = script.entity_pos(index as i32) {
            self.spark_tint(x, y, z + 1.0, true, 0);
        }
    }

    pub fn cure(&mut self, game: &mut GameState, status: u8) {
        game.clear_status(status);
        if status == STATUS_PARALYSIS {
            self.paralysis = 0.0;
        }
        if status == STATUS_POISON {
            self.poison_ticks = 0;
        }
    }

    /// Nigel is dead.
    fn kill(&mut self, sfx: &mut SfxQueue) {
        self.dead = true;
        self.death_t = 0.0;
        self.swing = None;
        self.paralysis = 0.0;
        sfx.play(0x65);
    }

    /// Some enemies add an ailment to the damage they do (worms and mushrooms in the original): in 40 % of the blows.
    fn try_status_attack(&mut self, game: &mut GameState, sfx: &mut SfxQueue, msg: &mut Message, enemy: i32) {
        if self.dead || game.rand(100) >= 40 {
            return;
        }
        let status = match enemy {
            8 | 55 => STATUS_POISON, // second worm / second mushroom
            9 => {
                // third worm: poison, paralysis or confusion
                if game.rand(3) == 0 {
                    STATUS_POISON
                } else if game.rand(2) == 1 {
                    STATUS_PARALYSIS
                } else {
                    STATUS_CONFUSION
                }
            }
            56 => game.rand(4) as u8, // third mushroom: any of the four
            _ => return,
        };
        self.inflict(game, sfx, msg, status);
    }

    fn spark(&mut self, x: f32, y: f32, z: f32, big: bool, hurt: bool) {
        self.new_sparks.push(Spark { x, y, z, big, tint: hurt as u8 });
    }

    fn spark_tint(&mut self, x: f32, y: f32, z: f32, big: bool, tint: u8) {
        self.new_sparks.push(Spark { x, y, z, big, tint });
    }
}

/// facing: 0 NW, 1 NE, 2 SE, 3 SW
const FACING_DIR: [(f32, f32); 4] = [(-1.0, 0.0), (0.0, -1.0), (1.0, 0.0), (0.0, 1.0)];

/// The sword button: starts the swing (the blow itself lands a moment later). After the death, a button brings Nigel back.
pub fn attack_input(ctl: Res<Controls>, frozen: Res<GameFrozen>, chest: Res<ChestOpen>, mut combat: ResMut<Combat>, mut game: ResMut<GameState>, mut sfx: ResMut<SfxQueue>, mut msg: ResMut<Message>, cut: Res<crate::cutscene::Cutscene>, players: Query<&Player>) {
    if frozen.0 {
        return;
    }
    if combat.dead {
        // the game over menu (title.rs) decides what comes next
        return;
    }
    let Ok(p) = players.single() else { return };
    if cut.active() {
        return;
    }
    // the button that opens a chest (or lifts, talks) must not swing the sword as well, even when both are on the same key
    if !ctl.attack.just || ctl.action.just || chest.0.is_some() || combat.cooldown > 0.0 || combat.swing.is_some() || combat.knock.2 > 0.0 || p.carrying.is_some() || p.lift_phase != 0 || p.climbing {
        return;
    }
    if game.has_status(STATUS_PARALYSIS) {
        // struggling: each press has a growing chance of breaking free (1 in 3, then 1 in 2, then sure)
        const ODDS: [u32; 3] = [3, 2, 1];
        let n = combat.paralysis_presses.min(2);
        combat.paralysis_presses += 1;
        if game.rand(ODDS[n]) == 0 {
            combat.cure(&mut game, STATUS_PARALYSIS);
            msg.show(crate::i18n::t("messages.status.break_free"));
        } else {
            sfx.play(0x44);
        }
        combat.cooldown = 0.25;
        return;
    }
    if game.has_status(STATUS_CURSE) {
        sfx.play(0x44);
        msg.show(crate::i18n::t("messages.status.cursed"));
        combat.cooldown = 0.5;
        return;
    }
    // a swing with the charge full is a magic one (the equipped sword's); any swing empties the charge
    combat.swing_magic = if game.sword > 0 && combat.charge >= CHARGE_FULL { game.sword } else { 0 };
    combat.magic_pending = matches!(combat.swing_magic, 2 | 4);
    if game.sword > 0 {
        combat.charge = 0.0;
    }
    combat.cooldown = ATTACK_COOLDOWN;
    combat.swing = Some(0.0);
    combat.swing_facing = p.facing;
    combat.hit_pending = true;
    sfx.play(0x51);
}

/// Debug (landstalker.ini [debug] `one_hit_kill`, and the debug panel): every blow of the sword kills what it hits (except what is indestructible).
pub static ONE_HIT_KILL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The sword (or its magic) hits an enemy: the damage, the death, the push back (`push`: the direction of the blow).
#[allow(clippy::too_many_arguments)]
fn strike(combat: &mut Combat, game: &mut GameState, script: &Scripting, sfx: &mut SfxQueue, room_id: i32, s: &EntSnapshot, stats: EnemyStats, magic: i32, push: Option<(f32, f32)>) {
    let tint = match magic {
        1 => 3,
        2 => 2,
        3 => 4,
        4 => 5,
        _ => 0,
    };
    if stats.health >= 255 {
        // indestructible: the sword just bounces off
        combat.spark_tint(s.x, s.y, s.z + 1.0, false, tint);
        sfx.play(0x52);
        return;
    }
    let dmg = game.roll_player_damage(stats.defence, magic);
    let hp = game.enemy_hp(room_id, s.index, stats.health);
    *hp -= (dmg * 256.0) as i32;
    if ONE_HIT_KILL.load(std::sync::atomic::Ordering::Relaxed) {
        *hp = 0;
    }
    if std::env::var("LS_LOG_COMBAT").is_ok() {
        println!("[combat] sword hits entity {} (kind {}) for {:.2}: {:.2} left{}", s.index, s.kind, dmg, *hp as f32 / 256.0, if magic > 0 { format!(" (magic {})", magic) } else { String::new() });
    }
    if *hp <= 0 {
        // dead for good right away; the body stays a moment for the death animation (longer for a boss)
        let boss = BOSSES.contains(&s.kind);
        game.collected.insert((room_id, s.index));
        script.kill_enemy(s.index, if boss { 1.8 } else { 0.45 }, stats.gold);
        combat.spark_tint(s.x, s.y, s.z + 1.0, true, tint);
        sfx.play(if boss { 0x6D } else { 0x66 });
        if boss {
            combat.shake = (1.8, 1.8, 3.0);
        }
    } else {
        if let Some((fx, fy)) = push {
            // pushed back along the blow: big enemies less, bosses hardly at all
            let base = (s.hbase * 16.0).round().max(8.0);
            let mut knock = KNOCKBACK * 8.0 / base;
            if base >= 16.0 {
                knock *= 0.5;
            }
            script.hit_enemy(s.index, fx, fy, knock);
        }
        combat.spark_tint(s.x, s.y, s.z + 1.0, false, tint);
        sfx.play(0x67);
    }
}

/// An enemy that can be fought: awake, hostile, in the room, not already dead.
fn alive_enemy(s: &EntSnapshot, room: i32, game: &GameState) -> bool {
    s.enemy && s.hostile && !s.dormant && !s.dying && s.visible && !game.collected.contains(&(room, s.index))
}

/// Damage to Nigel from something that is not a body: knocked away from (fx, fy).
fn hurt_player(combat: &mut Combat, game: &mut GameState, sfx: &mut SfxQueue, p: &Player, attack: i32, fx: f32, fy: f32, knockback: bool) {
    if combat.dead || combat.invulnerable > 0.0 {
        return;
    }
    combat.invulnerable = HIT_INVULNERABILITY;
    combat.hurt = HURT_POSE_TIME;
    let (ax, ay) = (p.x - fx, p.y - fy);
    combat.knock = if ax.abs() >= ay.abs() { (ax.signum() * PLAYER_KNOCK_SPEED, 0.0, 0.0) } else { (0.0, ay.signum() * PLAYER_KNOCK_SPEED, 0.0) };
    combat.knock.2 = if knockback { PLAYER_KNOCK_TIME } else { 0.0 };
    combat.spark(p.x, p.y, p.z + 1.0, false, true);
    let dmg = game.roll_enemy_damage(attack);
    if std::env::var("LS_LOG_COMBAT").is_ok() {
        println!("[combat] Nigel hurt for {:.2} (attack {}), {:.2} left", dmg, attack, game.health as f32 / 256.0 - dmg);
    }
    if game.damage(dmg) {
        combat.kill(sfx);
    } else {
        sfx.play(0x61);
    }
}

/// Timers, the blow of the sword, the enemies touching Nigel, the floor under his feet.
pub fn combat_update(
    time: Res<Time>,
    frozen: Res<GameFrozen>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    mut game: ResMut<GameState>,
    mut combat: ResMut<Combat>,
    mut sfx: ResMut<SfxQueue>,
    mut msg: ResMut<Message>,
    mut players: Query<&mut Player>,
) {
    let room_id = room.room.file.id;
    if !combat.synced {
        combat.synced = true;
        combat.sync_status(&game);
        if std::env::var("LS_CHARGE").is_ok() {
            combat.charge = CHARGE_FULL; // test aid: the first swing is a magic one
        }
    }
    // a new room: what was killed or taken here before is gone
    if combat.room_seen != room_id {
        combat.room_seen = room_id;
        let gone: Vec<usize> = game.collected.iter().filter(|(r, _)| *r == room_id).map(|(_, i)| *i).collect();
        script.remove_entities(&gone);
    }
    if frozen.0 {
        return;
    }
    let Ok(mut p) = players.single_mut() else { return };
    let dt = time.delta_secs().min(0.05);
    let combat = &mut *combat;
    combat.cooldown = (combat.cooldown - dt).max(0.0);
    combat.invulnerable = (combat.invulnerable - dt).max(0.0);
    combat.hurt = (combat.hurt - dt).max(0.0);
    combat.knock.2 = (combat.knock.2 - dt).max(0.0);
    combat.shake.0 = (combat.shake.0 - dt).max(0.0);
    // test aid: LS_DIE=seconds kills Nigel that far into the game
    static DIED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !combat.dead && std::env::var("LS_DIE").ok().and_then(|v| v.parse::<f32>().ok()).is_some_and(|t| time.elapsed_secs() > t) && !DIED.swap(true, std::sync::atomic::Ordering::Relaxed) {
        game.damage(1000.0);
        combat.kill(&mut sfx);
    }
    if combat.dead {
        combat.death_t += dt;
        return;
    }
    let snaps = script.entities();

    // poison costs a point of health every couple of seconds of walking, 8 times in all; paralysis wears off by itself
    if game.has_status(STATUS_POISON) && p.moving {
        combat.poison_walk += dt;
        if combat.poison_walk >= 2.1 {
            combat.poison_walk = 0.0;
            if game.damage(1.0) {
                combat.kill(&mut sfx);
                return;
            }
            combat.poison_ticks -= 1;
            if combat.poison_ticks <= 0 {
                combat.cure(&mut game, STATUS_POISON);
                msg.show(crate::i18n::t("messages.status.poison_gone"));
            }
        }
    }
    if game.has_status(STATUS_PARALYSIS) {
        combat.paralysis -= dt;
        if combat.paralysis <= 0.0 {
            combat.cure(&mut game, STATUS_PARALYSIS);
            msg.show(crate::i18n::t("messages.status.can_move"));
        }
    }
    // the Healing Boots mend a little while walking
    if game.boots == BOOTS_HEALING && p.moving && game.health < game.max_health {
        combat.heal_acc += dt;
        if combat.heal_acc >= 256.0 / 60.0 {
            combat.heal_acc = 0.0;
            game.heal(1.0);
        }
    }

    // the charge of the sword rises (32 a frame at 60 Hz, more with the Saturn or the Venus Stone) while a sword is worn
    if game.sword > 0 {
        combat.charge = (combat.charge + dt * 60.0 * game.charge_rate()).min(CHARGE_FULL);
    }

    // the sword
    if let Some(t) = combat.swing.as_mut() {
        *t += dt;
        let t = *t;
        if combat.hit_pending && t >= SWING_HIT_TIME {
            combat.hit_pending = false;
            let (fx, fy) = FACING_DIR[combat.swing_facing as usize & 3];
            let (cx, cy) = (p.x + fx * ATTACK_REACH, p.y + fy * ATTACK_REACH);
            const HALF_REACH: f32 = 0.8; // the arc spreads to both sides
            for s in &snaps {
                if !alive_enemy(s, room_id, &game) {
                    continue;
                }
                let Some(stats) = combat.stats.get(&s.kind).copied() else { continue };
                if (cx - s.x).abs() >= s.hbase + HALF_REACH || (cy - s.y).abs() >= s.hbase + HALF_REACH || p.z >= s.z + s.hheight || p.z + PLAYER_HEIGHT <= s.z {
                    continue;
                }
                let magic = combat.swing_magic;
                strike(combat, &mut game, &script, &mut sfx, room_id, s, stats, magic, Some((fx, fy)));
            }
        }
        // the magic of the Ice Sword (a wave) and of the Gaia Sword (every enemy of the room is hurt)
        if combat.magic_pending && t >= MAGIC_TIME {
            combat.magic_pending = false;
            let (fx, fy) = FACING_DIR[combat.swing_facing as usize & 3];
            if combat.swing_magic == 2 {
                combat.waves.push(Wave { x: p.x + fx * 0.6, y: p.y + fy * 0.6, z: p.z + 0.5, dx: fx, dy: fy, travelled: 0.0, hit: Vec::new(), spark_t: 0.0 });
            } else {
                sfx.play(0x59);
                combat.shake = (0.7, 0.7, 2.0);
                for s in &snaps {
                    if alive_enemy(s, room_id, &game) {
                        if let Some(stats) = combat.stats.get(&s.kind).copied() {
                            strike(combat, &mut game, &script, &mut sfx, room_id, s, stats, 4, None);
                        }
                    }
                }
            }
        }
        if t >= SWING_DURATION {
            combat.swing = None;
        }
    }

    // the Ice Sword's waves
    let mut waves = std::mem::take(&mut combat.waves);
    waves.retain_mut(|w| {
        let step = WAVE_SPEED * dt;
        w.x += w.dx * step;
        w.y += w.dy * step;
        w.travelled += step;
        w.spark_t -= dt;
        if w.spark_t <= 0.0 {
            w.spark_t = 0.05;
            combat.spark_tint(w.x, w.y, w.z, false, 2);
        }
        let wall = room.room.cell_at_world(w.x.floor() as i32, w.y.floor() as i32).map_or(true, |c| c.blocked());
        if wall || w.travelled > WAVE_RANGE {
            return false;
        }
        for s in &snaps {
            if w.hit.contains(&s.index) || !alive_enemy(s, room_id, &game) {
                continue;
            }
            if (w.x - s.x).abs() < s.hbase + 0.4 && (w.y - s.y).abs() < s.hbase + 0.4 && w.z < s.z + s.hheight + 1.0 && w.z + 1.0 > s.z {
                w.hit.push(s.index);
                if let Some(stats) = combat.stats.get(&s.kind).copied() {
                    strike(combat, &mut game, &script, &mut sfx, room_id, s, stats, 2, Some((w.dx, w.dy)));
                }
            }
        }
        true
    });
    combat.waves = waves;

    // enemies touching Nigel hurt him; a boulder rolling along its script crushes whoever it meets
    if combat.invulnerable <= 0.0 {
        for s in &snaps {
            let rolling = s.kind == 20 && s.visible && s.moved;
            if !alive_enemy(s, room_id, &game) && !rolling {
                continue;
            }
            let Some(stats) = combat.stats.get(&s.kind).copied() else { continue };
            let reach = s.hbase + PLAYER_RADIUS + 0.1;
            if (p.x - s.x).abs() < reach && (p.y - s.y).abs() < reach && p.z < s.z + s.hheight && p.z + PLAYER_HEIGHT > s.z {
                hurt_player(combat, &mut game, &mut sfx, &p, stats.attack, s.x, s.y, true);
                combat.try_status_attack(&mut game, &mut sfx, &mut msg, s.kind);
                break;
            }
        }
    }

    // the fireballs, axes and boomerangs of the bosses (they pass through him while he is protected)
    for s in &snaps {
        if s.proj_attack > 0 && s.visible && combat.invulnerable <= 0.0 && (p.x - s.x).abs() < PLAYER_RADIUS + 0.25 && (p.y - s.y).abs() < PLAYER_RADIUS + 0.25 && s.z + 0.3 > p.z && s.z - 0.3 < p.z + PLAYER_HEIGHT {
            hurt_player(combat, &mut game, &mut sfx, &p, s.proj_attack, s.x - s.proj_vx, s.y - s.proj_vy, true);
            script.clear_projectile(s.index);
        }
    }

    // the floor: spikes and swamp hurt (1), lava burns (5); Iron Boots protect from spikes, Fireproof boots from lava
    if p.grounded && !combat.dead {
        if let Some(cell) = room.room.cell_at_world(p.x.floor() as i32, p.y.floor() as i32) {
            if (p.z - cell.height as f32).abs() <= 0.5 {
                let boots = game.boots;
                if (cell.floor_type == 0x10 && boots != BOOTS_IRON) || cell.floor_type == 0x19 {
                    hurt_player(combat, &mut game, &mut sfx, &p, 1, p.x, p.y, false);
                } else if cell.floor_type == 0x29 && boots != BOOTS_FIREPROOF {
                    hurt_player(combat, &mut game, &mut sfx, &p, 5, p.x, p.y, false);
                }
            }
        }
    }
    let _ = &mut p;
}

/// Gold bags and item boxes the dead left, and the items lying on the floor (entities 0xC0 and up): touching one takes it.
pub fn pickups(frozen: Res<GameFrozen>, shops: Res<crate::shops::Shops>, script: NonSend<Scripting>, room: Res<RoomRes>, combat: Res<Combat>, mut game: ResMut<GameState>, mut msg: ResMut<Message>, mut sfx: ResMut<SfxQueue>, mut sound: ResMut<Sound>, players: Query<&Player>) {
    let Ok(p) = players.single() else { return };
    if frozen.0 || combat.dead {
        return;
    }
    let room_id = room.room.file.id;
    // the goods of a shop are bought, not picked up
    let shop_room = shops.is_shop(room_id);
    let church_room = shops.for_room(room_id).map_or(false, |i| i.kind == "church");
    for s in script.entities() {
        // (the books of a church are carried to the lectern, not picked up)
        let church_book = !(s.pickup_gold > 0 || s.pickup_item >= 0) && church_room && [227, 225, 226, 209, 228, 238].contains(&s.kind);
        if !s.visible || (shop_room && s.kind >= 0xC0) || church_book {
            continue;
        }
        let drop = s.pickup_gold > 0 || s.pickup_item >= 0;
        if !drop && s.kind < 0xC0 {
            continue;
        }
        let (half, top) = if drop { (0.6, s.z + 1.5) } else { (s.hbase.max(0.3), s.z + s.hheight.max(1.0)) };
        if (p.x - s.x).abs() >= half + PLAYER_RADIUS || (p.y - s.y).abs() >= half + PLAYER_RADIUS || p.z >= top || p.z + PLAYER_HEIGHT <= s.z {
            continue;
        }
        if drop {
            script.clear_pickup(s.index);
            if s.pickup_gold > 0 {
                game.add_gold(s.pickup_gold);
                sfx.play(0x6A);
                msg.show(crate::i18n::tf("messages.pickup.gold", &[("n", s.pickup_gold.to_string())]));
            } else {
                game.add_item(s.pickup_item);
                sound.request_jingle(0x05, 2.6);
                msg.show(crate::i18n::tf("messages.pickup.item", &[("item", game.item_name(s.pickup_item))]));
            }
        } else if !game.collected.contains(&(room_id, s.index)) {
            // an item on the floor (type - 0xC0 is the item)
            let item = s.kind - 0xC0;
            game.collected.insert((room_id, s.index));
            script.clear_pickup(s.index);
            game.add_item(item);
            sound.request_jingle(0x05, 2.6);
            let count = game.item_count(item);
            let values = [("item", game.item_name(item)), ("count", count.to_string())];
            msg.show(if count > 1 {
                crate::i18n::tf("messages.pickup.item_count", &values)
            } else if item == crate::game::ITEM_LIFE_STOCK {
                crate::i18n::tf("messages.pickup.max_health_up", &values)
            } else {
                crate::i18n::tf("messages.pickup.item", &values)
            });
        }
    }
}

/// One ray of a spark: they fly out of the place of a blow and fade (RoomView draws them as lines).
#[derive(Component)]
pub struct SparkRay {
    x: f32,
    y: f32,
    z: f32,
    k: u32,
    rays: u32,
    big: bool,
    t: f32,
}

/// The sparks the combat asked for become rays under the room's root.
pub fn spawn_sparks(mut commands: Commands, mut combat: ResMut<Combat>, roots: Query<Entity, With<WorldRoot>>) {
    if combat.new_sparks.is_empty() {
        return;
    }
    let Ok(root) = roots.single() else {
        combat.new_sparks.clear();
        return;
    };
    for s in combat.new_sparks.drain(..) {
        let rays = if s.big { 12 } else { 8 };
        for k in 0..rays {
            // the rays alternate between two colours
            let (c1, c2) = match s.tint {
                1 => ((1.0, 0.63, 0.16), (1.0, 0.31, 0.16)),
                2 => ((0.6, 0.9, 1.0), (0.9, 1.0, 1.0)),
                3 => ((1.0, 0.55, 0.1), (1.0, 0.9, 0.2)),
                4 => ((1.0, 1.0, 0.4), (0.7, 0.8, 1.0)),
                5 => ((0.4, 1.0, 0.4), (0.9, 1.0, 0.6)),
                _ => ((1.0, 1.0, 1.0), (1.0, 1.0, 0.47)),
            };
            let c = if k & 1 == 1 { c2 } else { c1 };
            let color = Color::srgb(c.0, c.1, c.2);
            let e = commands.spawn((SparkRay { x: s.x, y: s.y, z: s.z, k, rays, big: s.big, t: 0.0 }, Sprite::from_color(color, Vec2::new(4.0, 1.0)), Transform::from_xyz(0.0, 0.0, 9.0))).id();
            commands.entity(root).add_child(e);
        }
    }
}

pub fn animate_sparks(mut commands: Commands, time: Res<Time>, room: Res<RoomRes>, mut rays: Query<(Entity, &mut SparkRay, &mut Sprite, &mut Transform)>) {
    for (e, mut r, mut sprite, mut tf) in &mut rays {
        r.t += time.delta_secs();
        if r.t >= SPARK_TIME {
            commands.entity(e).despawn();
            continue;
        }
        let u = r.t / SPARK_TIME;
        let c = room.to_screen(r.x, r.y, r.z);
        let a = std::f32::consts::TAU * r.k as f32 / r.rays as f32 + r.t * 3.0;
        let inner = 4.0 + 10.0 * u;
        let outer = 10.0 + (if r.big { 26.0 } else { 16.0 }) * u;
        // squashed vertically: the burst is drawn on the isometric screen
        let (cx, cy) = (a.cos(), a.sin() * 0.6);
        let mid = (inner + outer) / 2.0;
        sprite.custom_size = Some(Vec2::new(outer - inner, 1.0));
        sprite.color = sprite.color.with_alpha(1.0 - u);
        tf.translation = Vec3::new(c.x + cx * mid, -(c.y + cy * mid), 9.0);
        tf.rotation = Quat::from_rotation_z((-cy).atan2(cx));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sword_can_only_start_when_free() {
        let mut c = Combat::default();
        assert!(!c.locked());
        c.swing = Some(0.0);
        assert!(c.locked());
        c.swing = None;
        c.knock.2 = 0.1;
        assert!(c.locked());
        c.knock.2 = 0.0;
        c.dead = true;
        assert!(c.locked());
    }
}
