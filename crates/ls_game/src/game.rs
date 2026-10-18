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
    /// health and maximum health, in 1/256 of a point
    pub health: i32,
    pub max_health: i32,
    /// what Nigel wears: item numbers, -1 for nothing
    pub sword: i32,
    pub armour: i32,
    pub boots: i32,
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
            health: START_HEALTH * 256,
            max_health: START_HEALTH * 256,
            sword: -1,
            armour: -1,
            boots: -1,
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
    /// The keys of data/text/items.txt (`items.life_stock`...), one per line, the number of the item is the line: the names are in data/i18n.
    pub fn with_names(text: &str) -> GameState {
        let names = text.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
        GameState { item_names: names, ..GameState::default() }
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

pub fn message_overlay(mut contexts: EguiContexts, mut msg: ResMut<Message>, time: Res<Time>) -> Result {
    if msg.time <= 0.0 || msg.text.is_empty() {
        return Ok(());
    }
    msg.time -= time.delta_secs();
    let ctx = contexts.ctx_mut()?.clone();
    let screen = ctx.content_rect();
    let game_w = screen.width() - crate::playtest::PANEL_W;
    egui::Area::new(egui::Id::new("game_message")).fixed_pos(egui::pos2(game_w / 2.0 - 220.0, screen.height() - 90.0)).interactable(false).show(&ctx, |ui| {
        egui::Frame::new().fill(egui::Color32::from_rgba_unmultiplied(20, 20, 60, 230)).stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(220, 220, 240))).inner_margin(egui::Margin::same(10)).show(ui, |ui| {
            ui.set_min_width(420.0);
            ui.label(egui::RichText::new(&msg.text).size(22.0).color(egui::Color32::WHITE));
        });
    });
    Ok(())
}

/// Life, gold and EkeEke at the top left of the game area (the console's bar; hearts and numbers).
pub fn hud_overlay(mut contexts: EguiContexts, game: Res<GameState>, combat: Res<crate::combat::Combat>, time: Res<Time>, cut: Res<crate::cutscene::Cutscene>, title: Res<crate::title::Title>) -> Result {
    if cut.intro || title.active {
        return Ok(()); // the intro scenes have no bar
    }
    let ctx = contexts.ctx_mut()?.clone();
    egui::Area::new(egui::Id::new("game_hud")).fixed_pos(egui::pos2(14.0, 10.0)).interactable(false).show(&ctx, |ui| {
        egui::Frame::new().fill(egui::Color32::from_rgba_unmultiplied(20, 20, 60, 220)).stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(220, 220, 240))).inner_margin(egui::Margin::symmetric(10, 6)).show(ui, |ui| {
            ui.horizontal(|ui| {
                let (hp, max) = (game.hp(), game.max_hp());
                // a heart per point of health, up to 20 of them; the numbers say the rest
                let mut hearts = String::new();
                for i in 0..max.min(20) {
                    hearts.push(if i < hp { '\u{2665}' } else { '\u{2661}' });
                }
                ui.label(egui::RichText::new(hearts).size(20.0).color(egui::Color32::from_rgb(240, 70, 70)));
                ui.label(egui::RichText::new(format!("{:>2}/{:>2}", hp, max)).size(20.0).monospace().color(egui::Color32::WHITE));
                ui.add_space(10.0);
                ui.label(egui::RichText::new(format!("EkeEke {:>2}", game.item_count(ITEM_EKEEKE))).size(18.0).color(egui::Color32::from_rgb(120, 220, 120)));
                ui.add_space(10.0);
                for (s, name, color) in [(STATUS_POISON, "POISON", egui::Color32::from_rgb(120, 230, 120)), (STATUS_CONFUSION, "CONFUSED", egui::Color32::from_rgb(240, 230, 100)), (STATUS_PARALYSIS, "PARALYSED", egui::Color32::from_rgb(130, 150, 255)), (STATUS_CURSE, "CURSED", egui::Color32::from_rgb(210, 140, 255))] {
                    if game.has_status(s) {
                        ui.label(egui::RichText::new(name).size(18.0).strong().color(color));
                    }
                }
                // the charge of the sword: when it is full the next swing is a magic one (the sword's colour, flashing)
                if game.sword > 0 {
                    ui.add_space(10.0);
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(110.0, 14.0), egui::Sense::hover());
                    let fill = (combat.charge / crate::combat::CHARGE_FULL).clamp(0.0, 1.0);
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
                        egui::Color32::from_rgb(90, 130, 220)
                    };
                    ui.painter().rect_filled(rect, 2.0, egui::Color32::from_rgb(10, 10, 30));
                    ui.painter().rect_filled(egui::Rect::from_min_size(rect.min, egui::vec2(rect.width() * fill, rect.height())), 2.0, color);
                    ui.painter().rect_stroke(rect, 2.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 220, 240)), egui::StrokeKind::Inside);
                }
                ui.add_space(10.0);
                ui.label(egui::RichText::new(format!("{} G", game.gold)).size(18.0).color(egui::Color32::from_rgb(240, 210, 80)));
            });
        });
    });
    Ok(())
}
