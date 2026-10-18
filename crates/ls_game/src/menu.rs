//! The item menu (Tab): what Nigel carries, with what each item does when used: an EkeEke heals, a Key opens the door he stands in
//! front of, the cure items cure (poison, confusion, paralysis, curse), swords, armours and boots are put on or taken off, the Idol
//! Stone opens the door of the Swamp Shrine. Port of RoomView::OpenMenu / UseMenuItem / UseKey / UseIdol of the C++ engine.

use crate::audio::SfxQueue;
use crate::combat::Combat;
use crate::config::Controls;
use crate::game::{self, GameState, Message};
use crate::scripting::Scripting;
use crate::{ChestOpen, GameFrozen, Player, RoomRes};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub const ITEM_IDOL_STONE: i32 = 49;
pub const ITEM_KEY: i32 = 50;

#[derive(Resource, Default)]
pub struct Menu {
    pub open: bool,
    pub index: usize,
    /// the game was frozen by the menu (and is given back a frame after it closes: the button that closed it must not also swing the sword)
    froze: bool,
    unfreeze: bool,
    /// seconds before the Idol Stone's door opens for good
    idol: f32,
}

/// The items in the bag, by number.
pub fn bag(game: &GameState) -> Vec<i32> {
    let mut v: Vec<i32> = game.items.iter().filter(|(_, n)| **n > 0).map(|(i, _)| *i).collect();
    v.sort();
    v
}

/// Using a key: the floor type of the cell Nigel stands on tells which door (26 + n: the locked tile door n of the room; 38 / 39: a
/// side door, an entity that disappears). Returns the flag to set, or why not.
fn door_flag(room: &RoomRes, script: &Scripting, p: &Player) -> Result<i32, &'static str> {
    let cell = room.room.cell_at_world(p.x.floor() as i32, p.y.floor() as i32);
    let ty = cell.map_or(0, |c| (c.raw() & 0xFF) as i32);
    let tile_door = (26..26 + 15).contains(&ty) && ty != 38 && ty != 39;
    let side_door = ty == 38 || ty == 39;
    if !tile_door && !side_door {
        return Err("Nothing to unlock here.");
    }
    let mut flag = None;
    if tile_door {
        let trigger = (ty - 26) as i64;
        flag = room.room.file.swap_flags.iter().find(|s| s.get("locked").and_then(|v| v.as_bool()).unwrap_or(false) && s.get("trigger").and_then(|v| v.as_i64()) == Some(trigger)).and_then(|s| s.get("flag").and_then(|v| v.as_i64())).map(|f| f as i32);
    } else {
        // the door entity closest to Nigel whose flag is not set yet
        let mut best = f32::MAX;
        for r in &room.room.file.entity_flags {
            let int = |k: &str| r.get(k).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            if r.get("kind").and_then(|v| v.as_str()) != Some("hide") || !r.get("when_set").and_then(|v| v.as_bool()).unwrap_or(false) || script.flag(int("flag")) {
                continue;
            }
            if let Some((x, y, _)) = script.entity_pos(int("entity")) {
                let d = (x - p.x).abs() + (y - p.y).abs();
                if d < best {
                    best = d;
                    flag = Some(int("flag"));
                }
            }
        }
        if best > 4.0 {
            flag = None;
        }
    }
    match flag {
        None => Err("The Key does not fit here."),
        Some(f) if script.flag(f) => Err("This door is already open."),
        Some(f) => Ok(f),
    }
}

/// What an item does when it is used from the menu. Returns whether the menu stays open.
#[allow(clippy::too_many_arguments)]
fn use_item(item: i32, game: &mut GameState, combat: &mut Combat, sfx: &mut SfxQueue, msg: &mut Message, script: &Scripting, room: &RoomRes, p: &Player, menu: &mut Menu) -> bool {
    sfx.play(0x43); // cursor select
    if item == game::ITEM_EKEEKE {
        if game.use_ekeeke() {
            sfx.play(0x69);
            msg.show("Used an EkeEke");
        } else {
            sfx.play(0x44);
            msg.show("Health is already full");
        }
    } else if item == ITEM_KEY {
        match door_flag(room, script, p) {
            Err(why) => {
                sfx.play(0x44);
                msg.show(why);
            }
            Ok(flag) if game.item_count(ITEM_KEY) > 0 => {
                game.use_item(ITEM_KEY);
                sfx.play(0x58);
                script.set_flag(flag);
                msg.show("The door is unlocked.");
                return false;
            }
            Ok(_) => {
                sfx.play(0x44);
                msg.show("The door is locked. You need a Key.");
            }
        }
        return false;
    } else if item == ITEM_IDOL_STONE {
        // held up in front of the door of the Swamp Shrine (room 433, cells x <= 15, y 25..27): the ground rumbles, a moment later the
        // door is open for good (flag 6)
        let (cx, cy) = (p.x.floor() as i32, p.y.floor() as i32);
        if script.flag(6) || room.room.file.id != 433 || cx > 15 || !(25..=27).contains(&cy) {
            sfx.play(0x44);
            msg.show("Nothing happens.");
        } else {
            msg.show("Nigel held up the Idol Stone over the statue.");
            sfx.play(0x59);
            menu.idol = 0.5;
        }
        return false;
    } else if let Some(status) = game::cure_of(item) {
        if game.has_status(status) {
            combat.cure(game, status);
            game.use_item(item);
            sfx.play(0x69);
            msg.show("You feel better.");
        } else {
            sfx.play(0x44);
            msg.show("It has no effect.");
        }
    } else if game::is_sword(item) || game::is_armour(item) || game::is_boots(item) {
        let on = game.toggle_equip(item);
        msg.show(format!("{} {}", if on { "Equipped" } else { "Removed" }, game.item_name(item)));
    } else {
        sfx.play(0x44);
        msg.show("Nothing happens.");
    }
    true
}

/// Opens and closes the menu, moves in it and uses what is chosen. The game stands still while it is open.
#[allow(clippy::too_many_arguments)]
pub fn menu_input(
    time: Res<Time>,
    ctl: Res<Controls>,
    mut menu: ResMut<Menu>,
    mut frozen: ResMut<GameFrozen>,
    mut game: ResMut<GameState>,
    mut combat: ResMut<Combat>,
    mut sfx: ResMut<SfxQueue>,
    mut msg: ResMut<Message>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    chest: Res<ChestOpen>,
    cut: Res<crate::cutscene::Cutscene>,
    players: Query<&Player>,
) {
    if menu.unfreeze {
        menu.unfreeze = false;
        if menu.froze {
            menu.froze = false;
            frozen.0 = false;
        }
    }
    // the Idol Stone's door opens
    if menu.idol > 0.0 {
        menu.idol -= time.delta_secs();
        if menu.idol <= 0.0 {
            script.set_flag(6);
        }
    }
    let Ok(p) = players.single() else { return };
    if !menu.open {
        // the E key: an EkeEke at once
        if ctl.ekeeke.just && !frozen.0 && !combat.dead && !cut.active() {
            if game.use_ekeeke() {
                sfx.play(0x69);
                msg.show("Used an EkeEke");
            } else {
                sfx.play(0x44);
                msg.show(if game.item_count(game::ITEM_EKEEKE) <= 0 { "No EkeEke left" } else { "Health is already full" });
            }
        }
        if ctl.menu.just && !frozen.0 && !combat.dead && chest.0.is_none() && p.lift_phase == 0 && !cut.active() {
            menu.open = true;
            menu.index = 0;
            menu.froze = true;
            frozen.0 = true;
            sfx.play(0x68);
        }
        return;
    }
    let items = bag(&game);
    if ctl.menu.just || ctl.attack.just || ctl.jump.just {
        menu.open = false;
        menu.unfreeze = true;
        sfx.play(0x43);
        return;
    }
    if !items.is_empty() {
        if ctl.dir[0].just {
            menu.index = (menu.index + items.len() - 1) % items.len();
            sfx.play(0x42);
        }
        if ctl.dir[1].just {
            menu.index = (menu.index + 1) % items.len();
            sfx.play(0x42);
        }
        menu.index = menu.index.min(items.len() - 1);
        if ctl.action.just {
            let item = items[menu.index];
            let keep_open = use_item(item, &mut game, &mut combat, &mut sfx, &mut msg, &script, &room, p, &mut menu);
            if !keep_open {
                menu.open = false;
                menu.unfreeze = true;
            }
            let n = bag(&game).len();
            menu.index = menu.index.min(n.saturating_sub(1));
        }
    }
}

/// The menu on the screen.
pub fn menu_overlay(mut contexts: EguiContexts, menu: Res<Menu>, game: Res<GameState>) -> Result {
    if !menu.open {
        return Ok(());
    }
    let ctx = contexts.ctx_mut()?.clone();
    let screen = ctx.content_rect();
    let game_w = screen.width() - crate::playtest::PANEL_W;
    egui::Area::new(egui::Id::new("item_menu")).fixed_pos(egui::pos2(game_w / 2.0 - 250.0, 90.0)).interactable(false).show(&ctx, |ui| {
        egui::Frame::new().fill(egui::Color32::from_rgba_unmultiplied(20, 20, 60, 240)).stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(220, 220, 240))).inner_margin(egui::Margin::same(12)).show(ui, |ui| {
            ui.set_min_width(480.0);
            ui.label(egui::RichText::new(format!("ITEMS      HP {}/{}      Gold {}", game.hp(), game.max_hp(), game.gold)).size(22.0).color(egui::Color32::WHITE));
            ui.add_space(8.0);
            let items = bag(&game);
            if items.is_empty() {
                ui.label(egui::RichText::new("  (nothing)").size(20.0).color(egui::Color32::GRAY));
            }
            const ROWS: usize = 10;
            let first = menu.index.saturating_sub(ROWS / 2).min(items.len().saturating_sub(ROWS));
            for (row, item) in items.iter().enumerate().skip(first).take(ROWS) {
                let mut text = format!("{} {}", if row == menu.index { ">" } else { " " }, game.item_name(*item));
                if game.item_count(*item) > 1 {
                    text.push_str(&format!(" x{}", game.item_count(*item)));
                }
                if *item == game.sword || *item == game.armour || *item == game.boots {
                    text.push_str("   (on)");
                }
                let color = if row == menu.index { egui::Color32::from_rgb(250, 230, 120) } else { egui::Color32::WHITE };
                ui.label(egui::RichText::new(text).size(20.0).monospace().color(color));
            }
            if items.len() > ROWS {
                ui.label(egui::RichText::new(format!("  ... {} items", items.len())).size(16.0).color(egui::Color32::GRAY));
            }
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Action: use      Menu / Attack: close").size(16.0).color(egui::Color32::LIGHT_GRAY));
        });
    });
    Ok(())
}
