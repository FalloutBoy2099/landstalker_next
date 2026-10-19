//! What the player has and what the game tells him: gold, items, opened chests, the message line.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::collections::{HashMap, HashSet};

/// Health is kept in 1/256 of a point, like the original (8.8 fixed point). Nigel starts with 4 points (the HUD's "4/ 4"), a
/// Life Stock adds one, an enemy hits for its attack value.
pub const START_HEALTH: i32 = 4;
pub const MAX_HEALTH_CAP: i32 = 99;
pub const ITEM_EKEEKE: i32 = 0;
pub const ITEM_LIFE_STOCK: i32 = 62;
/// Boots: Fireproof (5, lava), Iron (6, spikes), Healing (7, slowly heals), Snow Spikes (8, ice).
pub const BOOTS_FIREPROOF: i32 = 5;
pub const BOOTS_IRON: i32 = 6;
pub const BOOTS_HEALING: i32 = 7;
pub const BOOTS_SNOW: i32 = 8;

/// Status ailments (the bit numbers of the original's g_PlayerStatus).
pub const STATUS_POISON: u8 = 0;
pub const STATUS_CONFUSION: u8 = 1;
pub const STATUS_PARALYSIS: u8 = 2;
pub const STATUS_CURSE: u8 = 3;

/// Swords: Magic Sword (1), Sword of Ice (2), Thunder Sword (3), Sword of Gaia (4). Armours: Steel (9), Chrome (10), Shell (11), Hyper (12) Breast.
pub fn is_sword(item: i32) -> bool {
    (1..=4).contains(&item)
}
pub fn is_armour(item: i32) -> bool {
    (9..=12).contains(&item)
}
pub fn is_boots(item: i32) -> bool {
    (5..=8).contains(&item)
}
/// The rings: Mars (13: no poison, no paralysis), Moon (14: no confusion, no curse), Saturn (15) and Venus (16): the sword charges faster.
pub const RING_MARS: i32 = 13;
pub const RING_MOON: i32 = 14;
pub const RING_SATURN: i32 = 15;
pub const RING_VENUS: i32 = 16;
pub fn is_ring(item: i32) -> bool {
    (RING_MARS..=RING_VENUS).contains(&item)
}
/// What the equipment is, in the order of the equipment menu.
pub fn is_equipment(item: i32) -> bool {
    is_sword(item) || is_armour(item) || is_boots(item) || is_ring(item)
}

/// The ailment an item cures: Detox Grass / Detox Book (poison), Mind Repair (confusion), Anti-Paralyze, Anti-Curse.
pub fn cure_of(item: i32) -> Option<u8> {
    match item {
        18 | 33 => Some(STATUS_POISON),
        21 => Some(STATUS_CONFUSION),
        28 => Some(STATUS_PARALYSIS),
        34 => Some(STATUS_CURSE),
        _ => None,
    }
}

#[derive(Resource)]
pub struct GameState {
    pub gold: i32,
    pub items: HashMap<i32, i32>,
    /// chests opened: (room, entity index)
    pub opened: HashSet<(i32, usize)>,
    pub item_names: Vec<String>,
    /// what each item is (data/items.yaml): usable, sword, armour, boots, ring or pocket; and the place of the usable ones in the item menu
    pub item_kinds: Vec<String>,
    pub item_order: Vec<i32>,
    /// health and maximum health, in 1/256 of a point
    pub health: i32,
    pub max_health: i32,
    /// what Nigel wears: item numbers, -1 for nothing
    pub sword: i32,
    pub armour: i32,
    pub boots: i32,
    pub ring: i32,
    /// the quick item (the one the bar at the top shows, used by a key of its own): an EkeEke, or one of the cures
    pub quick: i32,
    /// ailments, one bit each (STATUS_*)
    pub status: u8,
    /// the story's progress, which only goes up (the dialogues of the characters follow it)
    pub progress: i32,
    /// where the priest recorded the adventure: room and place (Nigel wakes up there after dying)
    pub church: Option<(i32, f32, f32)>,
    /// enemies killed (and things taken): (room, entity index) never come back
    pub collected: HashSet<(i32, usize)>,
    /// what is left of the enemies that were hurt, in 1/256 of a point
    pub enemy_health: HashMap<(i32, usize), i32>,
    rng: u64,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            gold: 0,
            items: HashMap::new(),
            opened: HashSet::new(),
            item_names: Vec::new(),
            item_kinds: Vec::new(),
            item_order: Vec::new(),
            health: START_HEALTH * 256,
            max_health: START_HEALTH * 256,
            sword: -1,
            armour: -1,
            boots: -1,
            ring: -1,
            quick: ITEM_EKEEKE,
            status: 0,
            church: None,
            progress: 0,
            collected: HashSet::new(),
            enemy_health: HashMap::new(),
            rng: 0x2545_F491_4F6C_DD1D,
        }
    }
}

impl GameState {
    /// The name keys of data/items.yaml (`items.life_stock`...), indexed by the number of the item: the names are in data/i18n.
    /// `items` = the tree of data/items.yaml (index -> {name: key, ...}): the item names by index.
    pub fn with_names(items: &serde_json::Value) -> GameState {
        let mut list = crate::yamldata::int_keys(items);
        list.sort_by_key(|(i, _)| *i);
        let names = list.iter().map(|(_, v)| v.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string()).collect();
        let kinds = list.iter().map(|(_, v)| v.get("kind").and_then(|n| n.as_str()).unwrap_or("pocket").to_string()).collect();
        let order = list.iter().map(|(i, v)| v.get("order").and_then(|n| n.as_i64()).unwrap_or(1000 + *i as i64) as i32).collect();
        GameState { item_names: names, item_kinds: kinds, item_order: order, ..GameState::default() }
    }

    /// What an item is: "usable", "sword", "armour", "boots", "ring" or "pocket" (data/items.yaml; without it, what the game itself does with the item).
    pub fn item_kind(&self, id: i32) -> &str {
        if let Some(k) = usize::try_from(id).ok().and_then(|i| self.item_kinds.get(i)) {
            return k;
        }
        if is_sword(id) {
            "sword"
        } else if is_armour(id) {
            "armour"
        } else if is_boots(id) {
            "boots"
        } else if is_ring(id) {
            "ring"
        } else if id == ITEM_EKEEKE || cure_of(id).is_some() || id == 49 || id == 50 {
            "usable"
        } else {
            "pocket"
        }
    }

    /// The place of a usable item in the item menu (logical order: healing, cures, tools, statues...).
    pub fn item_place(&self, id: i32) -> i32 {
        usize::try_from(id).ok().and_then(|i| self.item_order.get(i)).copied().unwrap_or(1000 + id)
    }

    /// The name of an item in the language of the game ({6A} stands for a line break of the original's menu, other codes for nothing).
    pub fn item_name(&self, id: i32) -> String {
        let Some(key) = usize::try_from(id).ok().and_then(|i| self.item_names.get(i)) else { return crate::i18n::tf("messages.unknown_item", &[("id", id.to_string())]) };
        let l = crate::i18n::t(key);
        let mut out = String::new();
        let mut rest = l.as_str();
        while let Some(a) = rest.find('{') {
            out.push_str(&rest[..a]);
            match rest[a..].find('}') {
                Some(b) => {
                    if &rest[a..a + b + 1] == "{6A}" {
                        out.push(' ');
                    }
                    rest = &rest[a + b + 1..];
                }
                None => rest = "",
            }
        }
        out.push_str(rest);
        out
    }

    pub fn add_item(&mut self, id: i32) {
        // a Life Stock is used on the spot: one more point of health, and it is filled
        if id == ITEM_LIFE_STOCK {
            self.max_health = (self.max_health + 256).min(MAX_HEALTH_CAP * 256);
            self.health = (self.health + 256).min(self.max_health);
            return;
        }
        *self.items.entry(id).or_insert(0) += 1;
    }

    pub fn has_status(&self, s: u8) -> bool {
        (self.status >> s) & 1 != 0
    }

    /// The ring that is worn keeps some ailments away (poison 0 and paralysis 2: Mars; confusion 1 and curse 3: Moon).
    pub fn ring_protects(&self, s: u8) -> bool {
        self.ring == if s % 2 == 0 { RING_MARS } else { RING_MOON }
    }

    /// How fast the sword charges, in points a frame at 60 Hz (the original's 0x20, 0x40 with the Saturn Stone, 0x60 with the Venus Stone).
    pub fn charge_rate(&self) -> f32 {
        match self.ring {
            RING_VENUS => 96.0,
            RING_SATURN => 64.0,
            _ => 32.0,
        }
    }

    pub fn add_status(&mut self, s: u8) {
        self.status |= 1 << s;
    }

    pub fn clear_status(&mut self, s: u8) {
        self.status &= !(1 << s);
    }

    /// Uses up one of an item; false when there is none.
    pub fn use_item(&mut self, id: i32) -> bool {
        match self.items.get_mut(&id) {
            Some(n) if *n > 0 => {
                *n -= 1;
                true
            }
            _ => false,
        }
    }

    /// What the quick item can be: the EkeEke, and the cures Nigel carries.
    pub fn quick_items(&self) -> Vec<i32> {
        let mut v = vec![ITEM_EKEEKE];
        let mut cures: Vec<i32> = self.items.iter().filter(|(i, n)| **n > 0 && cure_of(**i).is_some()).map(|(i, _)| *i).collect();
        cures.sort();
        v.extend(cures);
        v
    }

    /// The quick item as it stands: the one chosen, or the EkeEke once there is none of it left.
    pub fn quick_item(&self) -> i32 {
        if self.quick == ITEM_EKEEKE || self.item_count(self.quick) > 0 {
            self.quick
        } else {
            ITEM_EKEEKE
        }
    }

    /// The next quick item (`step` 1), or the previous one (`step` -1).
    pub fn step_quick(&mut self, step: i32) -> i32 {
        let items = self.quick_items();
        let at = items.iter().position(|i| *i == self.quick_item()).unwrap_or(0) as i32;
        self.quick = items[(at + step).rem_euclid(items.len() as i32) as usize];
        self.quick
    }

    /// One EkeEke if there is one and health is not full: it restores half the maximum health, at least 10 points.
    pub fn use_ekeeke(&mut self) -> bool {
        if self.item_count(ITEM_EKEEKE) <= 0 || self.health >= self.max_health {
            return false;
        }
        self.use_item(ITEM_EKEEKE);
        self.health = (self.health + (self.max_health / 2).max(10 * 256)).min(self.max_health);
        true
    }

    /// Puts an item on, or takes it off if it is the one worn. Returns true if it is worn now.
    pub fn toggle_equip(&mut self, item: i32) -> bool {
        let slot = if is_sword(item) {
            &mut self.sword
        } else if is_boots(item) {
            &mut self.boots
        } else if is_ring(item) {
            &mut self.ring
        } else {
            &mut self.armour
        };
        if *slot == item {
            *slot = -1;
            false
        } else {
            *slot = item;
            true
        }
    }

    /// A new adventure: nothing carried over (the names of the items stay).
    pub fn reset(&mut self) {
        let names = std::mem::take(&mut self.item_names);
        *self = GameState::default();
        self.item_names = names;
    }

    pub fn item_count(&self, id: i32) -> i32 {
        self.items.get(&id).copied().unwrap_or(0)
    }

    /// A number in 0..n (xorshift: the game does not need better, and tests can seed it).
    pub fn rand(&mut self, n: u32) -> u32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        ((self.rng >> 33) as u32) % n.max(1)
    }

    // ---- the combat formulas of the original (code/gamelogic/combat.asm, 8.8 fixed point) ----

    /// Armour scales the damage taken: none 1.0, Steel 0.9, Chrome 0.8, Shell 0.7, Hyper 0.5.
    pub fn armour_multiplier(&self) -> f32 {
        const MULT: [f32; 5] = [1.0, 0.9, 0.8, 0.7, 0.5];
        if (9..=12).contains(&self.armour) {
            MULT[(self.armour - 8) as usize]
        } else {
            1.0
        }
    }

    /// Damage an enemy attack of strength `attack` does to Nigel: the strength times the armour factor, plus its integer part
    /// times a random 0..95/256.
    pub fn roll_enemy_damage(&mut self, attack: i32) -> f32 {
        let a = (attack & 0x7F) as f32 * self.armour_multiplier();
        let bonus = a.floor() * self.rand(96) as f32 / 256.0;
        ((a + bonus) * 256.0).round() / 256.0
    }

    /// Damage Nigel's sword does to an enemy of defence byte `defence`: a quarter of his maximum health minus half the defence,
    /// at least 1, plus the same random bonus; `magic` is the sword of a charged swing (0 for an ordinary one).
    pub fn roll_player_damage(&mut self, defence: i32, magic: i32) -> f32 {
        let d = (self.max_health as f32 / 256.0 / 4.0 - defence as f32 / 2.0).max(1.0);
        let dmg = d + d.floor() * self.rand(96) as f32 / 256.0;
        // a charged swing: the sword's magic multiplies it (MagicSwordBoost: 1.75, 1.5, 2.0, 1.25 for swords 1 to 4)
        const BOOST: [f32; 4] = [1.75, 1.5, 2.0, 1.25];
        if (1..=4).contains(&magic) {
            dmg * BOOST[magic as usize - 1]
        } else {
            dmg
        }
    }

    /// Hurts Nigel; true when that killed him.
    pub fn damage(&mut self, amount: f32) -> bool {
        self.health = (self.health - (amount * 256.0).round().max(0.0) as i32).max(0);
        self.health == 0
    }

    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + (amount * 256.0).round() as i32).min(self.max_health);
    }

    /// Health as the HUD shows it: a started point counts.
    pub fn hp(&self) -> i32 {
        if self.health <= 0 {
            0
        } else {
            (self.health + 255) / 256
        }
    }

    pub fn max_hp(&self) -> i32 {
        self.max_health / 256
    }

    pub fn set_max_hp(&mut self, hearts: i32) {
        self.max_health = hearts.clamp(START_HEALTH, MAX_HEALTH_CAP) * 256;
        self.health = self.max_health;
    }

    /// What is left of an enemy (room, entity), in 1/256 of a point; `initial` points on first use.
    pub fn enemy_hp(&mut self, room: i32, entity: usize, initial: i32) -> &mut i32 {
        self.enemy_health.entry((room, entity)).or_insert(initial * 256)
    }

    pub fn add_gold(&mut self, amount: i32) {
        self.gold += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_follows_the_original_formulas() {
        let mut g = GameState::default();
        // 4 points of health: a quarter is 1, so a defence of 6 leaves the minimum of 1 (+ bonus below 1 point)
        for _ in 0..50 {
            let d = g.roll_player_damage(6, 0);
            assert!((1.0..2.0).contains(&d), "{}", d);
            // the Thunder Sword's charged swing does twice that
            let t = g.roll_player_damage(6, 3);
            assert!((2.0..4.0).contains(&t), "{}", t);
        }
        // an attack of 3 does 3 plus up to 3 * 95/256
        for _ in 0..50 {
            let d = g.roll_enemy_damage(3);
            assert!((3.0..4.12).contains(&d), "{}", d);
        }
        g.armour = 12;
        assert_eq!(g.armour_multiplier(), 0.5);
        assert!(!g.damage(1.0));
        assert_eq!(g.hp(), 3);
        assert!(g.damage(10.0));
        assert_eq!(g.hp(), 0);
    }

    #[test]
    fn the_quick_item_goes_round_the_cures_he_carries() {
        let mut g = GameState::default();
        assert_eq!(g.quick_item(), ITEM_EKEEKE);
        assert_eq!(g.step_quick(1), ITEM_EKEEKE, "only the EkeEke");
        g.add_item(21);
        g.add_item(18);
        assert_eq!(g.step_quick(1), 18);
        assert_eq!(g.step_quick(1), 21);
        assert_eq!(g.step_quick(1), ITEM_EKEEKE);
        assert_eq!(g.step_quick(-1), 21);
        g.use_item(21);
        assert_eq!(g.quick_item(), ITEM_EKEEKE, "used up: back to the EkeEke");
    }

    #[test]
    fn the_rings_keep_ailments_away_and_charge_the_sword() {
        let mut g = GameState::default();
        assert!(!g.ring_protects(STATUS_POISON));
        assert_eq!(g.charge_rate(), 32.0);
        g.add_item(RING_MARS);
        g.add_item(RING_VENUS);
        assert!(g.toggle_equip(RING_MARS));
        assert!(g.ring_protects(STATUS_POISON) && g.ring_protects(STATUS_PARALYSIS));
        assert!(!g.ring_protects(STATUS_CONFUSION) && !g.ring_protects(STATUS_CURSE));
        assert!(g.toggle_equip(RING_VENUS), "one ring at a time");
        assert_eq!(g.ring, RING_VENUS);
        assert_eq!(g.charge_rate(), 96.0);
        assert!(!g.toggle_equip(RING_VENUS));
    }

    #[test]
    fn items_heal_cure_and_dress() {
        let mut g = GameState::default();
        g.set_max_hp(10);
        g.damage(8.0);
        assert!(!g.use_ekeeke(), "no EkeEke");
        g.add_item(ITEM_EKEEKE);
        g.add_item(ITEM_EKEEKE);
        assert!(g.use_ekeeke());
        assert_eq!(g.item_count(ITEM_EKEEKE), 1);
        assert_eq!(g.hp(), 10, "an EkeEke gives back half the maximum, at least 10 points");
        assert!(!g.use_ekeeke(), "not when health is full");
        // ailments
        g.add_status(STATUS_POISON);
        g.add_status(STATUS_CURSE);
        assert!(g.has_status(STATUS_POISON) && g.has_status(STATUS_CURSE) && !g.has_status(STATUS_PARALYSIS));
        assert_eq!(cure_of(18), Some(STATUS_POISON));
        g.clear_status(STATUS_POISON);
        assert!(!g.has_status(STATUS_POISON) && g.has_status(STATUS_CURSE));
        // equipment: one of each kind, the same item takes it off
        assert!(g.toggle_equip(9) && g.armour == 9);
        assert!(g.toggle_equip(10) && g.armour == 10);
        assert!(g.toggle_equip(6) && g.boots == 6 && g.armour == 10);
        assert!(!g.toggle_equip(6) && g.boots == -1);
        assert!(g.toggle_equip(2) && g.sword == 2);
    }

    #[test]
    fn a_life_stock_raises_and_fills_the_health() {
        let mut g = GameState::default();
        g.damage(2.0);
        g.add_item(ITEM_LIFE_STOCK);
        assert_eq!(g.max_hp(), 5);
        assert_eq!(g.hp(), 3);
        assert_eq!(g.item_count(ITEM_LIFE_STOCK), 0);
    }
}

/// The message line at the bottom of the game area.
#[derive(Resource, Default)]
pub struct Message {
    pub text: String,
    pub time: f32,
}

impl Message {
    pub fn show(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.time = 4.0;
    }
}

/// The chest being opened: the lid rises, then the content is given and held up.
pub const CHEST_OPEN_TIME: f32 = 0.4;
pub const CHEST_REVEAL_TIME: f32 = 0.8;

pub struct ChestSeq {
    pub entity: usize,
    /// chest content value (an item, or gold: see `give_chest_content`), -1 empty
    pub content: i32,
    pub t: f32,
    pub given: bool,
    pub item_sprite: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct ChestOpen(pub Option<ChestSeq>);

/// A thing rising over the chest.
#[derive(Component)]
pub struct ChestItem;

pub fn message_overlay(mut contexts: EguiContexts, mut msg: ResMut<Message>, time: Res<Time>, config: Res<crate::config::Config>) -> Result {
    if msg.time <= 0.0 || msg.text.is_empty() {
        return Ok(());
    }
    msg.time -= time.delta_secs();
    let ctx = contexts.ctx_mut()?.clone();
    crate::shops::notice_box(&ctx, &config.ui, "game_message", &msg.text);
    Ok(())
}

/// A heart, drawn: two lobes and a point.
fn draw_heart(painter: &egui::Painter, centre: egui::Pos2, size: f32, color: egui::Color32) {
    let r = size * 0.27;
    painter.circle_filled(egui::pos2(centre.x - r, centre.y - r * 0.5), r, color);
    painter.circle_filled(egui::pos2(centre.x + r, centre.y - r * 0.5), r, color);
    painter.add(egui::Shape::convex_polygon(vec![egui::pos2(centre.x - 2.0 * r + 0.6, centre.y - r * 0.2), egui::pos2(centre.x + 2.0 * r - 0.6, centre.y - r * 0.2), egui::pos2(centre.x, centre.y + size * 0.42)], color, egui::Stroke::NONE));
}

/// The bar at the top of the game area, in two halves with the middle left free: on the left a quarter of the width for the sword and its charge, on the right
/// a quarter for the rest: the item chosen, the heart with the health, then the EkeEke, the keys and the gold, each with its picture and its number.
#[allow(clippy::too_many_arguments)]
pub fn hud_overlay(
    mut contexts: EguiContexts,
    config: Res<crate::config::Config>,
    game: Res<GameState>,
    combat: Res<crate::combat::Combat>,
    time: Res<Time>,
    cut: Res<crate::cutscene::Cutscene>,
    title: Res<crate::title::Title>,
    files: Res<crate::DataFiles>,
    sprites: Res<crate::Sprites>,
    mut icons: Local<crate::menu::IconCache>,
) -> Result {
    if cut.intro || title.active {
        return Ok(()); // the intro scenes have no bar
    }
    let ctx = contexts.ctx_mut()?.clone();
    // the bar scales as a whole (landstalker.ini, [ui]): sizes in pixels times hud_scale, the text at hud_font_size
    let (sc, fs) = (config.ui.hud_scale, config.ui.hud_font_size);
    let icon = 38.0 * sc;
    let game_w = ctx.content_rect().width() - if config.playtest_panel { crate::playtest::PANEL_W } else { 0.0 };
    // each half is 30 % of the width of the game area: the frame (its margins and its line included) and what is inside
    let margin = 12.0 * sc;
    let outer = game_w * 0.30 - 8.0 * sc;
    let inner = (outer - 2.0 * margin - 4.0).max(120.0);
    // the window of the dialogues: their background, a light frame with round corners
    let [r, g, b, a] = config.ui.dialog_background;
    let frame = || egui::Frame::new().fill(egui::Color32::from_rgba_unmultiplied(r, g, b, a)).corner_radius(egui::CornerRadius::same(10)).stroke(egui::Stroke::new(3.0, egui::Color32::from_gray(204))).inner_margin(egui::Margin::symmetric(margin as i8, (9.0 * sc) as i8));

    // the left half: the sword, long, with its charge; when it is full the next swing is a magic one (the sword's colour, flashing)
    egui::Area::new(egui::Id::new("game_hud_sword")).fixed_pos(egui::pos2(8.0 * sc, 6.0 * sc)).interactable(false).show(&ctx, |ui| {
        frame().show(ui, |ui| {
            ui.set_width(inner);
            let (rect, _) = ui.allocate_exact_size(egui::vec2(inner, icon), egui::Sense::hover());
            let fill = if game.sword > 0 { (combat.charge / crate::combat::CHARGE_FULL).clamp(0.0, 1.0) } else { 0.0 };
            let color = if fill >= 1.0 {
                let flash = (time.elapsed_secs() * 8.0).sin() > 0.0;
                match game.sword {
                    1 => egui::Color32::from_rgb(255, 140, 40),
                    2 => egui::Color32::from_rgb(120, 220, 255),
                    3 => egui::Color32::from_rgb(255, 240, 80),
                    _ => egui::Color32::from_rgb(120, 255, 120),
                }
                .gamma_multiply(if flash { 1.0 } else { 0.6 })
            } else {
                egui::Color32::from_rgb(230, 70, 80)
            };
            let metal = egui::Color32::from_rgb(200, 200, 215);
            let p = ui.painter();
            let mid = rect.center().y;
            // the guard and the grip
            p.rect_filled(egui::Rect::from_center_size(egui::pos2(rect.left() + 13.0 * sc, mid), egui::vec2(6.0 * sc, icon * 0.85)), 1.0, metal);
            p.rect_filled(egui::Rect::from_min_max(egui::pos2(rect.left(), mid - 3.0 * sc), egui::pos2(rect.left() + 11.0 * sc, mid + 3.0 * sc)), 1.0, egui::Color32::from_rgb(150, 110, 70));
            // the blade, which fills with the charge
            let blade = egui::Rect::from_min_max(egui::pos2(rect.left() + 18.0 * sc, mid - 7.0 * sc), egui::pos2(rect.right(), mid + 7.0 * sc));
            p.rect_filled(blade, 2.0, egui::Color32::from_rgb(10, 10, 30));
            p.rect_filled(egui::Rect::from_min_size(blade.min, egui::vec2(blade.width() * fill, blade.height())), 2.0, color);
            p.rect_stroke(blade, 2.0, egui::Stroke::new(1.5, metal), egui::StrokeKind::Inside);
        });
    });

    // the item chosen (the EkeEke unless another was picked): in a frame of its own, to the left of the one on the right
    egui::Area::new(egui::Id::new("game_hud_item")).pivot(egui::Align2::RIGHT_TOP).fixed_pos(egui::pos2(game_w - 8.0 * sc - outer - 8.0 * sc, 6.0 * sc)).interactable(false).show(&ctx, |ui| {
        frame().show(ui, |ui| {
            let (rect, _) = ui.allocate_exact_size(egui::vec2(icon * 1.3, icon), egui::Sense::hover());
            crate::menu::draw_icon(ui.painter(), crate::menu::icon_of_kind(&ctx, &mut icons, &files, &sprites, 0xC0 + game.quick_item()), rect);
        });
    });

    // the right half: four parts, each with its picture and its number
    egui::Area::new(egui::Id::new("game_hud_items")).pivot(egui::Align2::RIGHT_TOP).fixed_pos(egui::pos2(game_w - 8.0 * sc, 6.0 * sc)).interactable(false).show(&ctx, |ui| {
        frame().show(ui, |ui| {
            ui.set_width(inner);
            ui.spacing_mut().item_spacing.x = 0.0;
            let text = fs * 1.0;
            let font = egui::FontId::monospace(text);
            // four equal parts, what is in each one in the middle of it
            let part_w = inner / 4.0;
            ui.horizontal(|ui| {
                for n in 1..5 {
                    ui.allocate_ui(egui::vec2(part_w, icon), |ui| {
                        ui.spacing_mut().item_spacing.x = 5.0 * sc;
                        let value = match n {
                            0 => String::new(),
                            1 => format!("{}/{}", game.hp(), game.max_hp()),
                            2 => game.item_count(ITEM_EKEEKE).to_string(),
                            3 => game.item_count(crate::menu::ITEM_KEY).to_string(),
                            _ => game.gold.to_string(),
                        };
                        let picture_w = if n == 1 { icon * 0.85 } else { icon };
                        let text_w = if value.is_empty() { 0.0 } else { ui.painter().layout_no_wrap(value.clone(), font.clone(), egui::Color32::WHITE).size().x + 5.0 * sc };
                        ui.horizontal(|ui| {
                            ui.add_space(((part_w - picture_w - text_w) / 2.0).max(0.0));
                            let kind = match n {
                                0 => 0xC0 + game.quick_item(),
                                2 => 0xC0 + ITEM_EKEEKE,
                                3 => 0xC0 + crate::menu::ITEM_KEY,
                                _ => GOLD_PICTURE,
                            };
                            let (rect, _) = ui.allocate_exact_size(egui::vec2(picture_w, icon), egui::Sense::hover());
                            if n == 1 {
                                // the heart and the health
                                draw_heart(ui.painter(), rect.center(), icon * 0.8, egui::Color32::from_rgb(240, 70, 70));
                            } else {
                                crate::menu::draw_icon(ui.painter(), crate::menu::icon_of_kind(&ctx, &mut icons, &files, &sprites, kind), rect);
                            }
                            if !value.is_empty() {
                                ui.label(egui::RichText::new(value).font(font.clone()).color(egui::Color32::WHITE));
                            }
                        });
                    });
                }
            });
        });
    });
    Ok(())
}

/// The entity type whose picture is a coin (the gold the enemies drop).
const GOLD_PICTURE: i32 = 10;
