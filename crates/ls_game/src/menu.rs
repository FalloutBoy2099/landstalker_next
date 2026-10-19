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
    /// the equipment menu (swords, armours, boots, rings) instead of the item menu
    pub equipment: bool,
    /// the category the equipment menu is on: sword, armour, boots, ring
    pub category: usize,
    /// the game was frozen by the menu (and is given back a frame after it closes: the button that closed it must not also swing the sword)
    froze: bool,
    unfreeze: bool,
    /// seconds before the Idol Stone's door opens for good
    idol: f32,
    /// seconds left to confirm the EkeEke asked for (the key pressed once)
    confirm: f32,
}

/// The categories of the equipment menu, left to right.
pub const CATEGORIES: [&str; 4] = ["sword", "armour", "boots", "ring"];

/// What Nigel can use, in the order of data/items.yaml.
pub fn usable_list(game: &GameState) -> Vec<i32> {
    let mut v: Vec<i32> = game.items.iter().filter(|(i, n)| **n > 0 && game.item_kind(**i) == "usable").map(|(i, _)| *i).collect();
    v.sort_by_key(|i| (game.item_place(*i), *i));
    v
}

/// What he only carries.
pub fn pocket_list(game: &GameState) -> Vec<i32> {
    let mut v: Vec<i32> = game.items.iter().filter(|(i, n)| **n > 0 && game.item_kind(**i) == "pocket").map(|(i, _)| *i).collect();
    v.sort();
    v
}

/// What can be worn in a category of the equipment menu: nothing (-1), then what he carries.
pub fn equip_list(game: &GameState, category: usize) -> Vec<i32> {
    let mut v: Vec<i32> = game.items.iter().filter(|(i, n)| **n > 0 && game.item_kind(**i) == CATEGORIES[category % 4]).map(|(i, _)| *i).collect();
    v.sort();
    v.insert(0, -1);
    v
}

/// What is worn in a category.
pub fn worn(game: &GameState, category: usize) -> i32 {
    match category % 4 {
        0 => game.sword,
        1 => game.armour,
        2 => game.boots,
        _ => game.ring,
    }
}

fn wear(game: &mut GameState, category: usize, item: i32) {
    match category % 4 {
        0 => game.sword = item,
        1 => game.armour = item,
        2 => game.boots = item,
        _ => game.ring = item,
    }
}

/// Using a key: the floor type of the cell Nigel stands on tells which door (26 + n: the locked tile door n of the room; 38 / 39: a
/// side door, an entity that disappears). Returns the flag to set, or why not.
fn door_flag(room: &RoomRes, script: &Scripting, p: &Player) -> Result<i32, &'static str> {
    let cell = room.room.cell_at_world(p.x.floor() as i32, p.y.floor() as i32);
    let ty = cell.map_or(0, |c| (c.raw() & 0xFF) as i32);
    let tile_door = (26..26 + 15).contains(&ty) && ty != 38 && ty != 39;
    let side_door = ty == 38 || ty == 39;
    if !tile_door && !side_door {
        return Err("messages.menu.nothing_to_unlock");
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
        None => Err("messages.menu.key_no_fit"),
        Some(f) if script.flag(f) => Err("messages.menu.door_open"),
        Some(f) => Ok(f),
    }
}

/// What an item does when it is used from the menu. Returns whether the menu stays open.
#[allow(clippy::too_many_arguments)]
fn use_item(item: i32, game: &mut GameState, combat: &mut Combat, sfx: &mut SfxQueue, msg: &mut Message, script: &Scripting, room: &RoomRes, p: &Player, menu: &mut Menu) -> bool {
    sfx.play(0x43); // cursor select
    if item == game::ITEM_EKEEKE {
        if game.use_ekeeke() {
            sfx.play(0x5A); // SND_HealthRecover2: what eating an item sounds like
            msg.show(crate::i18n::t("messages.menu.used_ekeeke"));
        } else {
            sfx.play(0x44);
            msg.show(crate::i18n::t("messages.menu.health_full"));
        }
    } else if item == ITEM_KEY {
        match door_flag(room, script, p) {
            Err(why) => {
                sfx.play(0x44);
                msg.show(crate::i18n::t(why));
            }
            Ok(flag) if game.item_count(ITEM_KEY) > 0 => {
                game.use_item(ITEM_KEY);
                sfx.play(0x58);
                script.set_flag(flag);
                msg.show(crate::i18n::t("messages.menu.door_unlocked"));
                return false;
            }
            Ok(_) => {
                sfx.play(0x44);
                msg.show(crate::i18n::t("messages.menu.door_locked"));
            }
        }
        return false;
    } else if item == ITEM_IDOL_STONE {
        // held up in front of the door of the Swamp Shrine (room 433, cells x <= 15, y 25..27): the ground rumbles, a moment later the
        // door is open for good (flag 6)
        let (cx, cy) = (p.x.floor() as i32, p.y.floor() as i32);
        if script.flag(6) || room.room.file.id != 433 || cx > 15 || !(25..=27).contains(&cy) {
            sfx.play(0x44);
            msg.show(crate::i18n::t("messages.menu.nothing_happens"));
        } else {
            msg.show(crate::i18n::t("messages.menu.idol"));
            sfx.play(0x59);
            menu.idol = 0.5;
        }
        return false;
    } else if let Some(status) = game::cure_of(item) {
        if game.has_status(status) {
            combat.cure(game, status);
            game.use_item(item);
            sfx.play(0x5A);
            msg.show(crate::i18n::t("messages.menu.feel_better"));
        } else {
            sfx.play(0x44);
            msg.show(crate::i18n::t("messages.menu.no_effect"));
        }
    } else if game::is_equipment(item) {
        let on = game.toggle_equip(item);
        msg.show(crate::i18n::tf(if on { "messages.menu.equipped" } else { "messages.menu.removed" }, &[("item", game.item_name(item))]));
    } else {
        sfx.play(0x44);
        msg.show(crate::i18n::t("messages.menu.nothing_happens"));
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
    menu.confirm = (menu.confirm - time.delta_secs()).max(0.0);
    // the Idol Stone's door opens
    if menu.idol > 0.0 {
        menu.idol -= time.delta_secs();
        if menu.idol <= 0.0 {
            script.set_flag(6);
        }
    }
    let Ok(p) = players.single() else { return };
    if !menu.open {
        let free = !frozen.0 && !combat.dead && !cut.active();
        // the quick item: the next one, then used at once (the keys are those of landstalker.ini)
        if (ctl.quick_next.just || ctl.quick_previous.just) && free {
            let item = game.step_quick(if ctl.quick_next.just { 1 } else { -1 });
            sfx.play(0x42);
            msg.show(crate::i18n::tf("messages.menu.quick_item", &[("item", game.item_name(item))]));
        }
        // an EkeEke at once: the key asks, the same key again does it
        if ctl.ekeeke.just && free {
            if game.item_count(game::ITEM_EKEEKE) <= 0 {
                sfx.play(0x44);
                msg.show(crate::i18n::t("messages.menu.no_ekeeke"));
            } else if game.health >= game.max_health {
                sfx.play(0x44);
                msg.show(crate::i18n::t("messages.menu.health_full"));
            } else if menu.confirm > 0.0 {
                menu.confirm = 0.0;
                if game.use_ekeeke() {
                    sfx.play(0x5A);
                    msg.show(crate::i18n::t("messages.menu.used_ekeeke"));
                }
            } else {
                menu.confirm = 3.0;
                sfx.play(0x42);
                msg.show(crate::i18n::t("messages.menu.confirm_ekeeke"));
            }
        }
        if ctl.use_key.just && free {
            if game.item_count(ITEM_KEY) <= 0 {
                sfx.play(0x44);
                msg.show(crate::i18n::tf("messages.menu.none_left", &[("item", game.item_name(ITEM_KEY))]));
            } else {
                use_item(ITEM_KEY, &mut game, &mut combat, &mut sfx, &mut msg, &script, &room, p, &mut menu);
            }
        }
        if ctl.quick_use.just && free {
            let item = game.quick_item();
            if game.item_count(item) <= 0 {
                sfx.play(0x44);
                msg.show(crate::i18n::tf("messages.menu.none_left", &[("item", game.item_name(item))]));
            } else {
                use_item(item, &mut game, &mut combat, &mut sfx, &mut msg, &script, &room, p, &mut menu);
            }
        }
        if (ctl.menu.just || ctl.equipment.just) && !frozen.0 && !combat.dead && chest.0.is_none() && p.lift_phase == 0 && !cut.active() {
            menu.open = true;
            menu.equipment = !ctl.menu.just;
            menu.index = 0;
            menu.froze = true;
            frozen.0 = true;
            sfx.play(0x68);
        }
        return;
    }
    // the menu key goes from the items to the equipment and back; the quit key closes
    if ctl.quit.just {
        menu.open = false;
        menu.unfreeze = true;
        sfx.play(0x43);
        return;
    }
    // each page has its key: it opens the page, closes it when it is the one that is open, and goes to it when the other one is open
    if ctl.menu.just || ctl.equipment.just {
        let wants_equipment = !ctl.menu.just;
        if menu.equipment == wants_equipment {
            menu.open = false;
            menu.unfreeze = true;
            sfx.play(0x43);
        } else {
            menu.equipment = wants_equipment;
            menu.index = 0;
            sfx.play(0x68);
        }
        return;
    }
    if menu.equipment {
        // left / right: the category; up / down: the item of the category, worn at once
        let n = CATEGORIES.len();
        if ctl.dir[2].just {
            menu.category = (menu.category + n - 1) % n;
            sfx.play(0x42);
        }
        if ctl.dir[3].just {
            menu.category = (menu.category + 1) % n;
            sfx.play(0x42);
        }
        let list = equip_list(&game, menu.category);
        let at = list.iter().position(|i| *i == worn(&game, menu.category)).unwrap_or(0);
        let to = if ctl.dir[0].just { Some((at + list.len() - 1) % list.len()) } else if ctl.dir[1].just { Some((at + 1) % list.len()) } else { None };
        if let Some(to) = to.filter(|t| list.len() > 1 && *t != at) {
            wear(&mut game, menu.category, list[to]);
            sfx.play(0x42);
        }
        return;
    }
    let items = usable_list(&game);
    if !items.is_empty() {
        // a grid of two columns: up / down by a row, left / right by one
        const COLS: usize = ITEM_COLUMNS;
        if ctl.dir[0].just && menu.index >= COLS {
            menu.index -= COLS;
            sfx.play(0x42);
        }
        if ctl.dir[1].just && menu.index + COLS < items.len() {
            menu.index += COLS;
            sfx.play(0x42);
        }
        if ctl.dir[2].just && menu.index > 0 {
            menu.index -= 1;
            sfx.play(0x42);
        }
        if ctl.dir[3].just && menu.index + 1 < items.len() {
            menu.index += 1;
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
            let n = usable_list(&game).len();
            menu.index = menu.index.min(n.saturating_sub(1));
        }
    }
}

const ITEM_COLUMNS: usize = 2;

pub type IconCache = std::collections::HashMap<i32, Option<(egui::TextureHandle, [f32; 2])>>;

/// The picture of an item: its sprite in the world (the entities from 0xC0 on are the items), as a texture of the menu.
pub fn icon_of(ctx: &egui::Context, cache: &mut IconCache, files: &crate::DataFiles, sprites: &crate::Sprites, item: i32) -> Option<(egui::TextureId, [f32; 2])> {
    icon_of_kind(ctx, cache, files, sprites, 0xC0 + item)
}

/// The picture of an entity type, as a texture.
pub fn icon_of_kind(ctx: &egui::Context, cache: &mut IconCache, files: &crate::DataFiles, sprites: &crate::Sprites, kind: i32) -> Option<(egui::TextureId, [f32; 2])> {
    cache
        .entry(kind)
        .or_insert_with(|| {
            let pic = crate::entity_picture(kind, 0, None, &sprites.entities, "", None, None)?;
            let img = files.image(&pic.path)?;
            let size = [img.width() as usize, img.height() as usize];
            let color = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
            Some((ctx.load_texture(format!("icon_{}", kind), color, egui::TextureOptions::NEAREST), [size[0] as f32, size[1] as f32]))
        })
        .as_ref()
        .map(|(t, s)| (t.id(), *s))
}

/// An icon in a box: whole-number scale (the pixels stay square), centred.
pub fn draw_icon(painter: &egui::Painter, icon: Option<(egui::TextureId, [f32; 2])>, area: egui::Rect) {
    let Some((tex, [w, h])) = icon else { return };
    let fit = (area.width() / w).min(area.height() / h);
    let scale = if fit >= 1.0 { fit.floor() } else { fit };
    let rect = egui::Rect::from_center_size(area.center(), egui::vec2(w * scale, h * scale));
    painter.image(tex, rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
}

/// A small triangle pointing up or down (the font has no arrows).
fn arrow(painter: &egui::Painter, at: egui::Pos2, up: bool, color: egui::Color32) {
    let d = if up { -1.0 } else { 1.0 };
    painter.add(egui::Shape::convex_polygon(vec![egui::pos2(at.x - 8.0, at.y - 4.0 * d), egui::pos2(at.x + 8.0, at.y - 4.0 * d), egui::pos2(at.x, at.y + 4.0 * d)], color, egui::Stroke::NONE));
}

const GOLD: egui::Color32 = egui::Color32::from_rgb(250, 230, 120);

/// The menu on the screen: transparent, over the game. The item menu is a grid of what can be used, then what is only carried; the equipment menu
/// is a row of the four categories with the item worn (and its neighbours) in the middle.
#[allow(clippy::too_many_arguments)]
pub fn menu_overlay(mut contexts: EguiContexts, config: Res<crate::config::Config>, menu: Res<Menu>, game: Res<GameState>, files: Res<crate::DataFiles>, sprites: Res<crate::Sprites>, mut icons: Local<IconCache>) -> Result {
    if !menu.open {
        return Ok(());
    }
    let ctx = contexts.ctx_mut()?.clone();
    let screen = ctx.content_rect();
    let game_w = screen.width() - crate::playtest::PANEL_W;
    const PANEL_W: f32 = 640.0;
    // in the middle of the game area, across and down
    egui::Area::new(egui::Id::new("item_menu")).pivot(egui::Align2::CENTER_CENTER).fixed_pos(egui::pos2(game_w / 2.0, screen.center().y)).interactable(false).show(&ctx, |ui| {
        egui::Frame::new()
            .fill({
                let [r, g, b, a] = config.ui.menu_background;
                egui::Color32::from_rgba_unmultiplied(r, g, b, a)
            })
            .corner_radius(egui::CornerRadius::same(10))
            .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(220, 220, 240, 230)))
            .inner_margin(egui::Margin::same(16))
            .show(ui, |ui| {
                let inner = PANEL_W - 32.0;
                ui.set_width(inner);
                ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);
                ui.label(egui::RichText::new(crate::i18n::t(if menu.equipment { "ui.menu.header_equipment" } else { "ui.menu.header" })).size(22.0).color(egui::Color32::WHITE));
                ui.add_space(6.0);
                if menu.equipment {
                    equipment_ui(ui, &ctx, &menu, &game, &files, &sprites, &mut icons, inner);
                } else {
                    items_ui(ui, &ctx, &menu, &game, &files, &sprites, &mut icons, inner);
                }
                ui.add_space(6.0);
                ui.label(egui::RichText::new(crate::i18n::t(if menu.equipment { "ui.menu.help_equipment" } else { "ui.menu.help" })).size(16.0).color(egui::Color32::LIGHT_GRAY));
            });
    });
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn items_ui(ui: &mut egui::Ui, ctx: &egui::Context, menu: &Menu, game: &GameState, files: &crate::DataFiles, sprites: &crate::Sprites, icons: &mut IconCache, inner: f32) {
    let items = usable_list(game);
    if items.is_empty() {
        ui.label(egui::RichText::new(crate::i18n::t("ui.menu.nothing")).size(20.0).color(egui::Color32::GRAY));
    }
    const ROWS: usize = 5;
    let total = (items.len() + ITEM_COLUMNS - 1) / ITEM_COLUMNS;
    let first = (menu.index / ITEM_COLUMNS).saturating_sub(ROWS / 2).min(total.saturating_sub(ROWS));
    let cell = egui::vec2((inner - 8.0 * (ITEM_COLUMNS as f32 - 1.0)) / ITEM_COLUMNS as f32, 56.0);
    for row in first..(first + ROWS).min(total) {
        ui.horizontal(|ui| {
            for col in 0..ITEM_COLUMNS {
                let at = row * ITEM_COLUMNS + col;
                let Some(&item) = items.get(at) else { break };
                let (rect, _) = ui.allocate_exact_size(cell, egui::Sense::hover());
                let painter = ui.painter();
                if at == menu.index {
                    painter.rect_filled(rect, 6.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 30));
                    painter.rect_stroke(rect, 6.0, egui::Stroke::new(2.0, GOLD), egui::StrokeKind::Inside);
                }
                draw_icon(painter, icon_of(ctx, icons, files, sprites, item), egui::Rect::from_min_size(rect.min + egui::vec2(8.0, 6.0), egui::vec2(44.0, 44.0)));
                let color = if at == menu.index { GOLD } else { egui::Color32::WHITE };
                painter.text(egui::pos2(rect.left() + 62.0, rect.center().y), egui::Align2::LEFT_CENTER, game.item_name(item), egui::FontId::proportional(20.0), color);
                if game.item_count(item) > 1 {
                    painter.text(egui::pos2(rect.right() - 10.0, rect.center().y), egui::Align2::RIGHT_CENTER, format!("\u{d7}{}", game.item_count(item)), egui::FontId::proportional(20.0), color);
                }
            }
        });
    }
    if total > ROWS {
        ui.label(egui::RichText::new(crate::i18n::tf("ui.menu.more", &[("n", items.len().to_string())])).size(14.0).color(egui::Color32::GRAY));
    }
    // what he only carries: not to be used
    let pocket = pocket_list(game);
    if !pocket.is_empty() {
        ui.add_space(8.0);
        ui.label(egui::RichText::new(crate::i18n::t("ui.menu.pocket")).size(16.0).color(egui::Color32::from_rgb(160, 180, 240)));
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 6.0);
            for item in pocket {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(30.0, 30.0), egui::Sense::hover());
                draw_icon(ui.painter(), icon_of(ctx, icons, files, sprites, item), rect);
                let mut text = game.item_name(item);
                if game.item_count(item) > 1 {
                    text.push_str(&format!(" \u{d7}{}", game.item_count(item)));
                }
                ui.label(egui::RichText::new(text).size(15.0).color(egui::Color32::from_gray(190)));
            }
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn equipment_ui(ui: &mut egui::Ui, ctx: &egui::Context, menu: &Menu, game: &GameState, files: &crate::DataFiles, sprites: &crate::Sprites, icons: &mut IconCache, inner: f32) {
    // the four categories side by side
    let tab = egui::vec2((inner - 8.0 * 3.0) / 4.0, 64.0);
    ui.horizontal(|ui| {
        for (c, name) in CATEGORIES.iter().enumerate() {
            let (rect, _) = ui.allocate_exact_size(tab, egui::Sense::hover());
            let on = c == menu.category;
            let painter = ui.painter();
            painter.rect_filled(rect, 6.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, if on { 34 } else { 10 }));
            if on {
                painter.rect_stroke(rect, 6.0, egui::Stroke::new(2.0, GOLD), egui::StrokeKind::Inside);
            }
            painter.text(egui::pos2(rect.center().x, rect.top() + 6.0), egui::Align2::CENTER_TOP, crate::i18n::t(&format!("ui.menu.cat_{}", name)), egui::FontId::proportional(18.0), if on { GOLD } else { egui::Color32::LIGHT_GRAY });
            let worn_item = worn(game, c);
            let label = if worn_item < 0 { crate::i18n::t("items.none") } else { game.item_name(worn_item) };
            painter.text(egui::pos2(rect.center().x, rect.bottom() - 8.0), egui::Align2::CENTER_BOTTOM, label, egui::FontId::proportional(14.0), egui::Color32::from_gray(200));
        }
    });
    // the middle: the item of the category (worn), the one before above it and the one after below
    let list = equip_list(game, menu.category);
    let at = list.iter().position(|i| *i == worn(game, menu.category)).unwrap_or(0);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(inner, 260.0), egui::Sense::hover());
    let painter = ui.painter();
    let name = |i: i32| if i < 0 { crate::i18n::t("items.none") } else { game.item_name(i) };
    let centre = rect.center();
    if list.len() > 1 {
        let before = list[(at + list.len() - 1) % list.len()];
        let after = list[(at + 1) % list.len()];
        let grey = egui::Color32::from_gray(150);
        arrow(painter, egui::pos2(centre.x, rect.top() + 8.0), true, grey);
        painter.text(egui::pos2(centre.x, rect.top() + 18.0), egui::Align2::CENTER_TOP, name(before), egui::FontId::proportional(17.0), grey);
        painter.text(egui::pos2(centre.x, rect.bottom() - 18.0), egui::Align2::CENTER_BOTTOM, name(after), egui::FontId::proportional(17.0), grey);
        arrow(painter, egui::pos2(centre.x, rect.bottom() - 8.0), false, grey);
    }
    let current = list[at];
    if current >= 0 {
        draw_icon(painter, icon_of(ctx, icons, files, sprites, current), egui::Rect::from_center_size(egui::pos2(centre.x, centre.y - 12.0), egui::vec2(96.0, 96.0)));
    }
    painter.text(egui::pos2(centre.x, centre.y + 50.0), egui::Align2::CENTER_CENTER, name(current), egui::FontId::proportional(26.0), GOLD);
    if list.len() > 1 {
        painter.text(egui::pos2(centre.x, centre.y + 76.0), egui::Align2::CENTER_CENTER, format!("{}/{}", at, list.len() - 1), egui::FontId::proportional(14.0), egui::Color32::from_gray(150));
    }
}
