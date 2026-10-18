//! Shops, inns and churches (data/text/shops.json, data/text/item_prices.txt) and the little dialogue box they speak through.
//! Port of RoomView::TryShop / OfferInn / OfferChurch / ShowText / AskChoice of the C++ engine.
//!
//! In a shop room the goods are items lying on the counter: facing one and pressing the action button asks if you want to buy it
//! (its base price times the shop's markup / 16). The innkeeper gives a night's rest (full health), the priest cures the ailments
//! or records the adventure (Nigel wakes up there after dying).

use crate::audio::{SfxQueue, Sound};
use crate::combat::Combat;
use crate::config::Controls;
use crate::dialogue::{Dialogues, Page, Story};
use crate::game::{GameState, Message};
use crate::cutscene::{Cutscene, Op};
use crate::menu::Menu;
use crate::scripting::Scripting;
use crate::{ChestOpen, GameFrozen, Player, RoomRes};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::collections::HashMap;
use std::path::Path;

pub struct Info {
    pub name: String,
    /// shop / inn / church / other
    pub kind: String,
    pub markup: i32,
    pub lifestock_markup: i32,
}

/// What the shops sell and for how much.
#[derive(Resource, Default)]
pub struct Shops {
    rooms: HashMap<i32, Info>,
    prices: HashMap<i32, i32>,
}

const LIFE_STOCK: i32 = 62;
/// One night at an inn: the original derives it from a table this port does not read, so a base price is assumed (like the C++
/// engine) and the inn's markup applied.
const INN_BASE_PRICE: i32 = 20;

impl Shops {
    pub fn load(data: &Path) -> Shops {
        let mut shops = Shops::default();
        if let Ok(text) = std::fs::read_to_string(data.join("text/shops.json")) {
            if let Ok(root) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(rooms) = root.get("rooms").and_then(|r| r.as_object()) {
                    for (k, v) in rooms {
                        let Ok(id) = k.parse::<i32>() else { continue };
                        let int = |key: &str| v.get(key).and_then(|x| x.as_i64()).unwrap_or(16) as i32;
                        shops.rooms.insert(id, Info { name: v.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string(), kind: v.get("kind").and_then(|n| n.as_str()).unwrap_or("").to_string(), markup: int("markup"), lifestock_markup: int("lifestock_markup") });
                    }
                }
            }
        }
        for line in std::fs::read_to_string(data.join("text/item_prices.txt")).unwrap_or_default().lines() {
            let n: Vec<i32> = line.split_whitespace().filter_map(|t| t.parse().ok()).collect();
            if !line.starts_with('#') && n.len() >= 2 {
                shops.prices.insert(n[0], n[1]);
            }
        }
        shops
    }

    pub fn for_room(&self, room: i32) -> Option<&Info> {
        self.rooms.get(&room)
    }

    pub fn is_shop(&self, room: i32) -> bool {
        self.for_room(room).map_or(false, |i| i.kind == "shop")
    }

    /// What this shop asks for an item: its base price times the shop's markup / 16 (another one for the Life Stock). None: not for sale.
    pub fn item_price(&self, info: &Info, item: i32) -> Option<i32> {
        let base = *self.prices.get(&item)?;
        Some(base * if item == LIFE_STOCK { info.lifestock_markup } else { info.markup } / 16)
    }

    pub fn inn_price(info: &Info) -> i32 {
        INN_BASE_PRICE * info.markup / 16
    }
}

/// What a yes / no question is about.
#[derive(Clone, Debug)]
enum Ask {
    /// the end of a conversation with a character: the answer decides what he says next
    Node,
    /// the question of a cutscene text: the operations of the branch the answer takes go on
    Cutscene,
    Buy { item: i32, price: i32, entity: usize },
    Inn { price: i32 },
    Pray { price: i32 },
    Record,
}

/// The dialogue box: a text, perhaps a question with its answer to pick.
#[derive(Resource, Default)]
pub struct Dialogue {
    pub open: bool,
    text: String,
    ask: Option<Ask>,
    yes: bool,
    froze: bool,
    unfreeze: bool,
    /// a conversation with a character: the boxes still to show (the first is on the screen), what follows a yes and a no
    pages: Vec<Page>,
    asks: bool,
    yes_node: Option<serde_json::Value>,
    no_node: Option<serde_json::Value>,
    /// the box belongs to a cutscene: when it is over the story goes on; the operations after a yes and after a no
    cutscene: bool,
    branch: Option<(Vec<Op>, Vec<Op>)>,
    /// the actions a conversation asks for once it is over
    actions: Vec<i32>,
    /// seconds a box stays on the screen before it goes away by itself (0: until it is dismissed), and the time it has been there
    wait: f32,
    timer: f32,
}

/// The story as the dialogues see it: the story flags (of the scripts) and the progress (of the game).
struct StoryOf<'a> {
    script: &'a Scripting,
    game: &'a mut GameState,
}

impl Story for StoryOf<'_> {
    fn flag(&self, n: i32) -> bool {
        self.script.flag(n)
    }
    fn set_flag(&mut self, n: i32) {
        self.script.set_flag(n);
    }
    fn progress(&self) -> i32 {
        self.game.progress
    }
    fn raise_progress(&mut self, v: i32) {
        self.game.progress = self.game.progress.max(v);
    }
}

/// What he gives: an item, with the message.
fn give(game: &mut GameState, msg: &mut Message, item: i32) {
    if item < 0 {
        return;
    }
    game.add_item(item);
    let mut text = format!("Got {}", game.item_name(item));
    if game.item_count(item) > 1 {
        text.push_str(&format!(" (x{})", game.item_count(item)));
    }
    msg.show(text);
}

impl Dialogue {
    fn say(&mut self, text: impl Into<String>) {
        self.open = true;
        self.text = text.into();
        self.ask = None;
        self.pages.clear();
    }

    fn ask(&mut self, text: impl Into<String>, ask: Ask) {
        self.open = true;
        self.text = text.into();
        self.ask = Some(ask);
        self.yes = true;
        self.pages.clear();
    }

    /// The boxes of a cutscene text, perhaps ending with a yes / no question (the two branches of the story).
    pub fn start_cutscene(&mut self, pages: Vec<String>, wait: f32, branch: Option<(Vec<Op>, Vec<Op>)>) {
        self.pages = pages.into_iter().map(|text| Page { text, give_item: -1 }).collect();
        self.asks = branch.is_some();
        self.branch = branch;
        self.cutscene = true;
        self.wait = wait;
        self.timer = 0.0;
        self.froze = true;
        self.show_page();
    }

    /// Everything said is over (the intro was skipped).
    pub fn close_all(&mut self) {
        self.open = false;
        self.pages.clear();
        self.ask = None;
        self.cutscene = false;
        self.unfreeze = false;
        self.froze = false;
    }

    /// Shows the first box of the conversation (the question comes with the last one).
    fn show_page(&mut self) {
        let Some(p) = self.pages.first() else { return };
        self.open = true;
        self.text = p.text.clone();
        self.timer = 0.0;
        if self.asks && self.pages.len() == 1 {
            self.ask = Some(if self.cutscene { Ask::Cutscene } else { Ask::Node });
            self.yes = true;
        } else {
            self.ask = None;
        }
    }

    fn start(&mut self, c: crate::dialogue::Conversation) {
        self.actions = c.actions;
        self.pages = c.pages;
        self.asks = c.asks;
        self.yes_node = c.yes;
        self.no_node = c.no;
        self.show_page();
    }
}

/// The action button in front of a thing for sale, or of the innkeeper or the priest.
#[allow(clippy::too_many_arguments)]
pub fn talk(
    ctl: Res<Controls>,
    mut frozen: ResMut<GameFrozen>,
    mut dlg: ResMut<Dialogue>,
    shops: Res<Shops>,
    mut game: ResMut<GameState>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    combat: Res<Combat>,
    chest: Res<ChestOpen>,
    menu: Res<Menu>,
    data: Res<Dialogues>,
    mut msg: ResMut<Message>,
    mut cut: ResMut<Cutscene>,
    players: Query<&Player>,
) {
    if !ctl.action.just || frozen.0 || dlg.open || menu.open || combat.dead || chest.0.is_some() || cut.active() {
        return;
    }
    let Ok(p) = players.single() else { return };
    let room_id = room.room.file.id;
    if p.carrying.is_some() || p.lift_phase != 0 || !p.grounded {
        return;
    }
    let Some(info) = shops.for_room(room_id) else {
        talk_to_character(&mut dlg, &mut frozen, &mut game, &script, &room, &*data, &mut msg, &mut cut, p);
        return;
    };
    const DIR: [(f32, f32); 4] = [(-1.0, 0.0), (0.0, -1.0), (1.0, 0.0), (0.0, 1.0)]; // facing: NW NE SE SW
    let (dx, dy) = DIR[(p.facing & 3) as usize];
    let snaps = script.entities();
    let mut found: Option<(String, Ask)> = None;
    if info.kind == "shop" {
        // the goods on the counter: items lying about (entity types 0xC0 and up)
        let (qx, qy) = (p.x + dx * (crate::physics::PLAYER_RADIUS + 0.4), p.y + dy * (crate::physics::PLAYER_RADIUS + 0.4));
        for s in &snaps {
            if s.kind < 0xC0 || !s.visible || game.collected.contains(&(room_id, s.index)) {
                continue;
            }
            let half = s.hbase.max(0.5) + 0.35;
            if (qx - s.x).abs() > half || (qy - s.y).abs() > half || (p.z - s.z).abs() > 2.0 {
                continue;
            }
            let item = s.kind - 0xC0;
            let Some(price) = shops.item_price(info, item) else { continue };
            found = Some((format!("{}: {} golds.\nDo you want to buy it?", game.item_name(item), price), Ask::Buy { item, price, entity: s.index }));
            break;
        }
    } else if info.kind == "inn" || info.kind == "church" {
        // the innkeeper, the priest: someone who talks, in front of him
        let (qx, qy) = (p.x + dx * (crate::physics::PLAYER_RADIUS + 0.4), p.y + dy * (crate::physics::PLAYER_RADIUS + 0.4));
        for s in &snaps {
            if !s.talks || !s.visible || s.kind == 0 || s.hbase <= 0.0 || game.collected.contains(&(room_id, s.index)) {
                continue;
            }
            let half = s.hbase + 0.3;
            if (qx - s.x).abs() > half || (qy - s.y).abs() > half || p.z >= s.z + s.hheight || p.z + crate::physics::PLAYER_HEIGHT <= s.z {
                continue;
            }
            let price = Shops::inn_price(info);
            found = Some(if info.kind == "inn" {
                (format!("Would you like to stay the night?\n{} golds.", price), Ask::Inn { price })
            } else if game.status & 0x0f != 0 {
                let count = (game.status & 0x0f).count_ones() as i32;
                (format!("My child, you are afflicted.\nA prayer costs {} golds.\nShall I pray for you?", price * count), Ask::Pray { price: price * count })
            } else {
                ("Welcome, my child.\nShall I record your adventure?".to_string(), Ask::Record)
            });
            break;
        }
    }
    if let Some((text, ask)) = found {
        dlg.ask(text, ask);
        dlg.froze = true;
        frozen.0 = true;
    } else if info.kind != "inn" && info.kind != "church" {
        talk_to_character(&mut dlg, &mut frozen, &mut game, &script, &room, &*data, &mut msg, &mut cut, p);
    }
}

/// The character in front of Nigel (someone who talks, and has something to say): the conversation starts.
#[allow(clippy::too_many_arguments)]
fn talk_to_character(dlg: &mut Dialogue, frozen: &mut GameFrozen, game: &mut GameState, script: &Scripting, room: &RoomRes, data: &Dialogues, msg: &mut Message, cut: &mut Cutscene, p: &Player) {
    const DIR: [(f32, f32); 4] = [(-1.0, 0.0), (0.0, -1.0), (1.0, 0.0), (0.0, 1.0)]; // facing: NW NE SE SW
    let (dx, dy) = DIR[(p.facing & 3) as usize];
    let (qx, qy) = (p.x + dx * (crate::physics::PLAYER_RADIUS + 0.4), p.y + dy * (crate::physics::PLAYER_RADIUS + 0.4));
    let file = &room.room.file;
    let log = std::env::var("LS_LOG_DIALOGUE").is_ok();
    if log {
        println!("[dialogue] action at ({:.2}, {:.2}) facing {}, looking at ({:.2}, {:.2})", p.x, p.y, p.facing, qx, qy);
    }
    for s in script.entities() {
        if log && s.talks {
            println!("[dialogue]   entity {} kind {} at ({:.2}, {:.2}, {:.2}) visible {} hbase {}", s.index, s.kind, s.x, s.y, s.z, s.visible, s.hbase);
        }
        if !s.talks || !s.visible || s.kind == 0 || s.hbase <= 0.0 || game.collected.contains(&(file.id, s.index)) {
            continue;
        }
        let half = s.hbase + 0.3;
        if (qx - s.x).abs() > half || (qy - s.y).abs() > half || p.z >= s.z + s.hheight || p.z + crate::physics::PLAYER_HEIGHT <= s.z {
            continue;
        }
        // the character: the entity's number into the room's list
        let Some(character) = file.entities.get(s.index).and_then(|e| e.dialogue).and_then(|d| usize::try_from(d).ok()).and_then(|d| file.characters.get(d)).copied() else { continue };
        let conv = data.begin(character, &mut StoryOf { script, game });
        for &item in &conv.give_now {
            give(game, msg, item);
        }
        if conv.pages.is_empty() {
            // nothing to say: the story acts at once (or the next one is tried)
            for &a in &conv.actions {
                cut.request(a, Some(s.index));
            }
            if conv.actions.is_empty() {
                continue;
            }
            return;
        }
        dlg.start(conv);
        dlg.froze = true;
        frozen.0 = true;
        return;
    }
}

/// Moves in an open dialogue box and acts on the answer.
#[allow(clippy::too_many_arguments)]
pub fn dialogue_input(
    ctl: Res<Controls>,
    mut dlg: ResMut<Dialogue>,
    mut frozen: ResMut<GameFrozen>,
    mut game: ResMut<GameState>,
    mut combat: ResMut<Combat>,
    mut sfx: ResMut<SfxQueue>,
    mut sound: ResMut<Sound>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    data: Res<Dialogues>,
    dir: Res<crate::DataDir>,
    mut msg: ResMut<Message>,
    mut cut: ResMut<Cutscene>,
    time: Res<Time>,
    players: Query<&Player>,
) {
    if dlg.unfreeze {
        dlg.unfreeze = false;
        if dlg.froze {
            dlg.froze = false;
            frozen.0 = false;
        }
    }
    if !dlg.open {
        return;
    }
    // a cutscene box that goes away by itself
    if dlg.wait > 0.0 && dlg.ask.is_none() && !dlg.pages.is_empty() {
        dlg.timer += time.delta_secs();
    }
    // the original types two frames a letter, then waits the `wait` of the operation (measured on the ROM)
    let timed_out = dlg.wait > 0.0 && dlg.timer >= dlg.wait + dlg.text.chars().count() as f32 * TYPE_TIME && dlg.ask.is_none();
    let Some(ask) = dlg.ask.clone() else {
        if ctl.action.just || ctl.attack.just || ctl.jump.just || timed_out {
            sfx.play(0x43);
            if dlg.pages.is_empty() {
                dlg.open = false;
                dlg.unfreeze = true;
            } else {
                // the box is dismissed (what it gives is received), the next one comes
                let item = dlg.pages[0].give_item;
                give(&mut game, &mut msg, item);
                dlg.pages.remove(0);
                if dlg.pages.is_empty() {
                    dlg.open = false;
                    dlg.unfreeze = true;
                    if std::mem::take(&mut dlg.cutscene) {
                        cut.resume = true;
                    }
                    for a in std::mem::take(&mut dlg.actions) {
                        cut.request(a, None);
                    }
                } else {
                    dlg.show_page();
                }
            }
        }
        return;
    };
    if ctl.dir.iter().any(|d| d.just) {
        dlg.yes = !dlg.yes;
        sfx.play(0x42);
    }
    let answer = if ctl.attack.just {
        Some(false)
    } else if ctl.action.just {
        Some(dlg.yes)
    } else {
        None
    };
    let Some(yes) = answer else { return };
    dlg.ask = None;
    let room_id = room.room.file.id;
    match ask {
        Ask::Cutscene => {
            // the story goes on with the branch the answer chose
            let (then_ops, else_ops) = dlg.branch.take().unwrap_or_default();
            cut.answer_with(yes, if yes { then_ops } else { else_ops });
            dlg.pages.clear();
            dlg.open = false;
            dlg.unfreeze = true;
            dlg.cutscene = false;
            cut.resume = true;
        }
        Ask::Node => {
            let item = dlg.pages.first().map_or(-1, |p| p.give_item);
            give(&mut game, &mut msg, item);
            let node = if yes { dlg.yes_node.take() } else { dlg.no_node.take() };
            let conv = data.follow(node.as_ref(), &mut StoryOf { script: &script, game: &mut game });
            if conv.pages.is_empty() {
                for &a in &conv.actions {
                    cut.request(a, None);
                }
                dlg.open = false;
                dlg.unfreeze = true;
            } else {
                dlg.start(conv);
            }
        }
        Ask::Buy { item, price, entity } => {
            if !yes {
                dlg.say("Are you just browsing?");
            } else if game.gold < price {
                sfx.play(0x44);
                dlg.say("You can't afford that now.\nMake more golds somewhere!");
            } else {
                game.gold -= price;
                game.add_item(item);
                sfx.play(0x6A);
                game.collected.insert((room_id, entity));
                script.clear_pickup(entity);
                dlg.say("Thank you!\nDon't waste it.");
            }
        }
        Ask::Inn { price } => {
            if !yes {
                dlg.say("Take it easy!");
            } else if game.gold < price {
                sfx.play(0x44);
                dlg.say("You can't afford that now.\nMake more golds somewhere!");
            } else {
                game.gold -= price;
                game.health = game.max_health;
                sound.request_jingle(0x06, 4.5); // MusicInnRest
                dlg.say("Good morning!\nYou feel fully rested.");
            }
        }
        Ask::Pray { price } => {
            if !yes {
                dlg.say("May God be with you.");
            } else if game.gold < price {
                sfx.play(0x44);
                dlg.say("You can't afford that now.\nCome back when you can.");
            } else {
                game.gold -= price;
                for s in 0..4 {
                    combat.cure(&mut game, s);
                }
                sfx.play(0x69);
                dlg.say("You are cured.\nGod bless you!");
            }
        }
        Ask::Record => {
            if !yes {
                dlg.say("May God be with you.");
            } else if let Ok(p) = players.single() {
                // the adventure is recorded here (and saved to landstalker.sav): Nigel wakes up in this church after dying
                game.church = Some((room_id, p.x, p.y));
                game.health = game.max_health;
                let saved = crate::save::write(&crate::save::path_of(&dir.0), &crate::save::snapshot(&game, &script, room_id, p.x, p.y));
                sound.request_jingle(0x07, 3.5); // MusicSaveGame
                dlg.say(if saved { "Your adventure is recorded.\nGod bless you!" } else { "Something went wrong...\nYour adventure could not be recorded." });
            }
        }
    }
}

/// The dialogue box at the bottom of the game area.
/// Seconds a letter takes to be typed in the boxes of the story.
const TYPE_TIME: f32 = 2.0 / 60.0;

pub fn dialogue_overlay(mut contexts: EguiContexts, dlg: Res<Dialogue>) -> Result {
    if !dlg.open {
        return Ok(());
    }
    let ctx = contexts.ctx_mut()?.clone();
    let screen = ctx.content_rect();
    let game_w = screen.width() - crate::playtest::PANEL_W;
    if dlg.wait > 0.0 && dlg.ask.is_none() {
        // the boxes of the story: the original's black box with a white frame, over the lines 165 to 218 of its screen
        let k = game_w / 320.0;
        let rect = egui::Rect::from_min_max(egui::pos2(screen.left() + 13.0 * k, screen.top() + 165.0 * k), egui::pos2(screen.left() + 307.0 * k, screen.top() + 218.0 * k));
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("story_box")));
        painter.rect_filled(rect, 3.0 * k, egui::Color32::BLACK);
        painter.rect_stroke(rect.shrink(k), 3.0 * k, egui::Stroke::new(k, egui::Color32::from_gray(204)), egui::StrokeKind::Inside);
        let shown: String = dlg.text.chars().take((dlg.timer / TYPE_TIME) as usize + 1).collect();
        for (i, line) in shown.lines().enumerate() {
            painter.text(egui::pos2(rect.left() + 4.0 * k, rect.top() + (5.0 + 16.0 * i as f32) * k), egui::Align2::LEFT_TOP, line, egui::FontId::monospace(12.0 * k), egui::Color32::WHITE);
        }
        return Ok(());
    }
    egui::Area::new(egui::Id::new("dialogue")).fixed_pos(egui::pos2(game_w / 2.0 - 320.0, screen.height() - 200.0)).interactable(false).show(&ctx, |ui| {
        egui::Frame::new().fill(egui::Color32::from_rgba_unmultiplied(20, 20, 60, 245)).stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(220, 220, 240))).inner_margin(egui::Margin::same(14)).show(ui, |ui| {
            ui.set_min_width(620.0);
            let shown: String = if dlg.wait > 0.0 { dlg.text.chars().take((dlg.timer / TYPE_TIME) as usize + 1).collect() } else { dlg.text.clone() };
            ui.label(egui::RichText::new(shown).size(24.0).color(egui::Color32::WHITE));
            if dlg.ask.is_some() {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    for (label, on) in [("Yes", true), ("No", false)] {
                        let selected = dlg.yes == on;
                        let color = if selected { egui::Color32::from_rgb(250, 230, 120) } else { egui::Color32::LIGHT_GRAY };
                        ui.label(egui::RichText::new(format!("{} {}", if selected { ">" } else { " " }, label)).size(22.0).monospace().color(color));
                        ui.add_space(24.0);
                    }
                });
            }
        });
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prices_follow_the_markup_of_the_shop() {
        let mut shops = Shops::default();
        shops.prices.insert(0, 15);
        shops.prices.insert(LIFE_STOCK, 400);
        let info = Info { name: "x".into(), kind: "shop".into(), markup: 24, lifestock_markup: 8 };
        assert_eq!(shops.item_price(&info, 0), Some(22)); // 15 * 24 / 16
        assert_eq!(shops.item_price(&info, LIFE_STOCK), Some(200)); // 400 * 8 / 16
        assert_eq!(shops.item_price(&info, 5), None);
        let inn = Info { name: "i".into(), kind: "inn".into(), markup: 8, lifestock_markup: 16 };
        assert_eq!(Shops::inn_price(&inn), 10);
    }

    #[test]
    fn the_real_data_has_shops_and_prices() {
        let data = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");
        let shops = Shops::load(&data);
        assert!(shops.is_shop(596) && shops.for_room(598).map(|i| i.kind.as_str()) == Some("inn"));
        let info = shops.for_room(596).unwrap();
        assert!(shops.item_price(info, 0).is_some());
    }
}
