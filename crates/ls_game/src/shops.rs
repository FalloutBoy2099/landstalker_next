//! Shops, inns and churches (data/shops.yaml, data/items.yaml) and the little dialogue box they speak through.
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
        let root = crate::yamldata::read(&data.join("shops.yaml"));
        if let Some(rooms) = root.get("rooms").and_then(|r| r.as_object()) {
            for (k, v) in rooms {
                let Ok(id) = k.parse::<i32>() else { continue };
                let int = |key: &str| v.get(key).and_then(|x| x.as_i64()).unwrap_or(16) as i32;
                shops.rooms.insert(id, Info { name: v.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string(), kind: v.get("kind").and_then(|n| n.as_str()).unwrap_or("").to_string(), markup: int("markup"), lifestock_markup: int("lifestock_markup") });
            }
        }
        // the base price of every item (data/items.yaml)
        for (index, item) in crate::yamldata::int_keys(&crate::yamldata::read(&data.join("items.yaml"))) {
            if let Some(price) = item.get("price").and_then(|p| p.as_i64()) {
                shops.prices.insert(index, price as i32);
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
    /// the church cures what the mask says (bits of the statuses: poison, confusion, paralysis, curse)
    Pray { price: i32, mask: u8 },
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
    /// the book of a church that lies on the lectern while the priest speaks: it goes back to its shelf afterwards
    book: Option<usize>,
    /// the priest has done his service: the book is used up and goes away (the original moves it off screen), otherwise it goes back to its shelf
    book_used: bool,
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
    let count = game.item_count(item);
    msg.show(crate::i18n::tf(if count > 1 { "messages.pickup.item_count" } else { "messages.pickup.item" }, &[("item", game.item_name(item)), ("count", count.to_string())]));
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
            found = Some((crate::i18n::tf("messages.shop.ask_buy", &[("item", game.item_name(item)), ("price", price.to_string())]), Ask::Buy { item, price, entity: s.index }));
            break;
        }
    } else if info.kind == "inn" {
        // the innkeeper: someone who talks, in front of him
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
            found = Some((crate::i18n::tf("messages.shop.ask_inn", &[("price", price.to_string())]), Ask::Inn { price }));
            break;
        }
    }
    if let Some((text, ask)) = found {
        dlg.ask(text, ask);
        dlg.froze = true;
        frozen.0 = true;
    } else if info.kind != "inn" {
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
    // a sign on the wall: Nigel stands on the cell in front of it (its floor type 0x11-0x18 or 0x1E-0x25), facing it; the sign number is the room's n-th character
    if let Some(cell) = room.room.cell_at_world(p.x.floor() as i32, p.y.floor() as i32) {
        let t = cell.floor_type as usize;
        // (the facing it must have: 1 north-east, 0 north-west; the number of the sign)
        let sign = match t {
            0x11..=0x14 => Some((1u8, t - 0x11)),
            0x15..=0x18 => Some((0, t - 0x15)),
            0x1E..=0x21 => Some((1, 4 + t - 0x1E)),
            0x22..=0x25 => Some((0, 4 + t - 0x22)),
            _ => None,
        };
        if let Some((_, n)) = sign.filter(|(face, _)| p.facing & 3 == *face) {
            if let Some(character) = file.characters.get(n).copied() {
                let conv = data.begin(character, &mut StoryOf { script, game });
                for &item in &conv.give_now {
                    give(game, msg, item);
                }
                if !conv.pages.is_empty() {
                    dlg.start(conv);
                    dlg.froze = true;
                    frozen.0 = true;
                    return;
                }
            }
        }
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
        dlg.timer += time.delta_secs().min(0.05); // like the recorded scenes' clock: a slow frame does not make the boxes run ahead of the picture
    }
    // the original types a letter a frame, then waits the `wait` of the operation (measured on the ROM)
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
                dlg.say(crate::i18n::t("messages.shop.browsing"));
            } else if game.gold < price {
                sfx.play(0x44);
                dlg.say(crate::i18n::t("messages.shop.cant_afford"));
            } else {
                game.gold -= price;
                game.add_item(item);
                sfx.play(0x6A);
                game.collected.insert((room_id, entity));
                script.clear_pickup(entity);
                dlg.say(crate::i18n::t("messages.shop.thanks"));
            }
        }
        Ask::Inn { price } => {
            if !yes {
                dlg.say(crate::i18n::t("messages.shop.take_it_easy"));
            } else if game.gold < price {
                sfx.play(0x44);
                dlg.say(crate::i18n::t("messages.shop.cant_afford"));
            } else {
                game.gold -= price;
                game.health = game.max_health;
                sound.request_jingle(0x06, 4.5); // MusicInnRest
                dlg.say(crate::i18n::t("messages.shop.inn_rested"));
            }
        }
        Ask::Pray { price, mask } => {
            if !yes {
                dlg.say(crate::i18n::t("messages.shop.god_with_you"));
            } else if game.gold < price {
                sfx.play(0x44);
                dlg.say(crate::i18n::t("messages.shop.inn_cant_afford"));
            } else {
                game.gold -= price;
                for s in 0..4 {
                    if mask >> s & 1 != 0 {
                        combat.cure(&mut game, s);
                    }
                }
                sfx.play(0x69);
                dlg.book_used = true;
                dlg.say(crate::i18n::t("messages.shop.cured"));
            }
        }
        Ask::Record => {
            if !yes {
                dlg.say(crate::i18n::t("messages.shop.god_with_you"));
            } else if let Ok(p) = players.single() {
                // the adventure is recorded here (and saved to landstalker.sav): Nigel wakes up in this church after dying
                game.church = Some((room_id, p.x, p.y));
                game.health = game.max_health;
                let saved = crate::save::write(&crate::save::path_of(&dir.0), &crate::save::snapshot(&game, &script, room_id, p.x, p.y));
                sound.request_jingle(0x07, 3.5); // MusicSaveGame
                dlg.book_used = true;
                dlg.say(crate::i18n::t(if saved { "messages.shop.recorded" } else { "messages.shop.record_failed" }));
            }
        }
    }
}

/// The dialogue box at the bottom of the game area.
/// Seconds a letter takes to be typed in the boxes of the story.
const TYPE_TIME: f32 = 1.0 / 60.0;

pub fn dialogue_overlay(mut contexts: EguiContexts, dlg: Res<Dialogue>, config: Res<crate::config::Config>) -> Result {
    if !dlg.open {
        return Ok(());
    }
    let ctx = contexts.ctx_mut()?.clone();
    let shown: String = if dlg.wait > 0.0 { dlg.text.chars().take((dlg.timer / TYPE_TIME) as usize + 1).collect() } else { dlg.text.clone() };
    text_box(&ctx, &config.ui, "dialogue", &dlg.text, &shown, dlg.ask.as_ref().map(|_| dlg.yes));
    Ok(())
}

/// A notification (an item found, gold, a refusal): the window of the dialogues, as small as its text, where the dialogues appear.
pub fn notice_box(ctx: &egui::Context, ui_cfg: &crate::config::UiSettings, id: &str, text: &str) {
    let screen = ctx.content_rect();
    let game_w = screen.width() - crate::playtest::PANEL_W;
    let at = egui::pos2(game_w / 2.0, screen.top() + 224.0 * crate::SCALE - ui_cfg.dialog_margin);
    egui::Area::new(egui::Id::new(id)).pivot(egui::Align2::CENTER_BOTTOM).order(egui::Order::Tooltip).fixed_pos(at).interactable(false).show(ctx, |ui| {
        let [r, g, b, a] = ui_cfg.dialog_background;
        egui::Frame::new()
            .fill(egui::Color32::from_rgba_unmultiplied(r, g, b, a))
            .corner_radius(egui::CornerRadius::same(10))
            .stroke(egui::Stroke::new(3.0, egui::Color32::from_gray(204)))
            .inner_margin(egui::Margin::symmetric((ui_cfg.dialog_padding * 0.8) as i8, (ui_cfg.dialog_padding * 0.5) as i8))
            .show(ui, |ui| {
                ui.label(egui::RichText::new(text).size(ui_cfg.font_size * 1.3).monospace().color(egui::Color32::WHITE));
            });
    });
}

/// The window every text of the game is shown in (the dialogues of the characters, their questions, the boxes of the story and of the scenes): the
/// background, a thin light frame with round corners, the text in monospace, at the place and the size of landstalker.ini, [ui]. `full` is all the text
/// (the box keeps its height while `shown` is typed in), `ask`: Some(yes selected) adds the answers.
pub fn text_box(ctx: &egui::Context, ui_cfg: &crate::config::UiSettings, id: &str, full: &str, shown: &str, ask: Option<bool>) {
    let screen = ctx.content_rect();
    let game_w = screen.width() - crate::playtest::PANEL_W;
    // the box sits at the bottom, the middle or the top of the picture of the game (320 x 224 of the original, drawn SCALE times larger from the top of the window)
    let picture_h = 224.0 * crate::SCALE;
    let (pivot, at) = match ui_cfg.dialog_position.as_str() {
        "top" => (egui::Align2::CENTER_TOP, egui::pos2(game_w / 2.0, screen.top() + ui_cfg.dialog_margin)),
        "middle" => (egui::Align2::CENTER_CENTER, egui::pos2(game_w / 2.0, screen.top() + picture_h / 2.0)),
        _ => (egui::Align2::CENTER_BOTTOM, egui::pos2(game_w / 2.0, screen.top() + picture_h - ui_cfg.dialog_margin)),
    };
    // as wide as the story boxes of the original (294 of the 320 columns), unless the ini says otherwise
    let width = if ui_cfg.dialog_width > 0.0 { ui_cfg.dialog_width } else { (294.0 * crate::SCALE).min(game_w - 16.0) };
    egui::Area::new(egui::Id::new(id)).pivot(pivot).order(egui::Order::Tooltip).fixed_pos(at).interactable(false).show(ctx, |ui| {
        let [r, g, b, a] = ui_cfg.dialog_background;
        let pad = ui_cfg.dialog_padding;
        egui::Frame::new().fill(egui::Color32::from_rgba_unmultiplied(r, g, b, a)).corner_radius(egui::CornerRadius::same(10)).stroke(egui::Stroke::new(3.0, egui::Color32::from_gray(204))).inner_margin(egui::Margin::same(pad as i8)).show(ui, |ui| {
            ui.set_min_width(width - 2.0 * pad);
            ui.set_max_width(width - 2.0 * pad);
            // (the original's boxes are three lines high: 53 of the 224 lines of the picture)
            let line = ui_cfg.dialog_font_size * 1.3;
            ui.set_min_height((53.0 * crate::SCALE - 2.0 * pad).max(full.lines().count() as f32 * line));
            ui.label(egui::RichText::new(shown).size(ui_cfg.dialog_font_size).monospace().color(egui::Color32::WHITE));
            if let Some(yes) = ask {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    for (label, on) in [(crate::i18n::t("ui.dialogue.yes"), true), (crate::i18n::t("ui.dialogue.no"), false)] {
                        let selected = yes == on;
                        let color = if selected { egui::Color32::from_rgb(250, 230, 120) } else { egui::Color32::LIGHT_GRAY };
                        ui.label(egui::RichText::new(format!("{} {}", if selected { ">" } else { " " }, label)).size(ui_cfg.dialog_font_size * 0.92).monospace().color(color));
                        ui.add_space(24.0);
                    }
                });
            }
        });
    });
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

/// The entity types of the books of the churches (record, detox, anti-curse, awakening, spell, restoration).
pub const CHURCH_BOOKS: [i32; 6] = [227, 225, 226, 209, 228, 238];

/// The books of the churches: Nigel lifts one off its shelf like a crate, carries it to the lectern (the cell of the floor type "counter", in front of the priest)
/// and puts it down there: that asks the priest for what the book is for (EB_ShopItem). The book goes back to its shelf when the priest has spoken, unless the service was done: then it is used up.
pub fn church_books(
    mut dlg: ResMut<Dialogue>,
    mut frozen: ResMut<GameFrozen>,
    shops: Res<Shops>,
    game: Res<GameState>,
    script: NonSend<Scripting>,
    room: Res<RoomRes>,
    thrown: Res<crate::Thrown>,
) {
    // (the kinds of the books: record, detox, anti-curse, awakening, spell, restoration; what each cures: the statuses poison 0, confusion 1, paralysis 2, curse 3)
    const BOOKS: [(i32, u8); 6] = [(227, 0), (225, 0b0001), (226, 0b1000), (209, 0b0100), (228, 0b0010), (238, 0b1111)];
    let file = &room.room.file;
    if let Some(i) = dlg.book {
        if !dlg.open {
            if std::mem::take(&mut dlg.book_used) {
                // the service is done: the book is used up
                script.set_b(i, "visible", false);
                script.set_b(i, "solid", false);
            } else if let Some(e) = file.entities.get(i) {
                // refused (or nothing to cure): the book goes back to its shelf
                script.set_f(i, "x", e.position[0] as f32);
                script.set_f(i, "y", e.position[1] as f32);
                script.set_f(i, "z", e.position[2] as f32);
            }
            dlg.book = None;
        }
        return;
    }
    let Some(info) = shops.for_room(file.id).filter(|i| i.kind == "church") else { return };
    if dlg.open || frozen.0 {
        return;
    }
    for s in script.entities() {
        let Some(&(_, mask)) = BOOKS.iter().find(|(k, _)| *k == s.kind) else { continue };
        if !s.visible || s.carried || thrown.0.iter().any(|t| t.index == s.index) {
            continue;
        }
        let Some(cell) = room.room.cell_at_world(s.x.floor() as i32, s.y.floor() as i32) else { continue };
        if cell.floor_type != 0x0E || (s.z - cell.height as f32).abs() > 0.2 {
            continue;
        }
        // on the lectern
        let price = Shops::inn_price(info);
        dlg.book = Some(s.index);
        if s.kind == 227 {
            dlg.ask(crate::i18n::t("messages.shop.ask_record"), Ask::Record);
        } else if game.status & mask != 0 {
            let count = (game.status & mask).count_ones() as i32;
            dlg.ask(crate::i18n::tf("messages.shop.ask_pray", &[("price", (price * count).to_string())]), Ask::Pray { price: price * count, mask });
        } else {
            dlg.say(crate::i18n::t("messages.shop.browsing"));
        }
        dlg.froze = true;
        frozen.0 = true;
        return;
    }
}
