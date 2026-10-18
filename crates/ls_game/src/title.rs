//! The screens around the game: the title (the Climax logo fades in and out, then the game's own title picture is wiped in from the left;
//! New game / Continue / Quit where the original says "PUSH START BUTTON") and the pause menu (Esc: Resume / Load game / Quit to title /
//! Quit) and the game over menu (once the death is over: Continue / Load game / Quit to title). Port of TitleScreen.h and the menus of
//! Menus.h of the C++ engine.

use crate::audio::{SfxQueue, Sound};
use crate::combat::Combat;
use crate::config::Controls;
use crate::cutscene::Cutscene;
use crate::game::{GameState, Message};
use crate::scripting::Scripting;
use crate::{Arrival, DataDir, GameFrozen, RoomRequest};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// seconds the splash (the Climax logo) lasts, the wipe of the title picture, the fades of the logo
const CLIMAX_END: f32 = 3.8;
const REVEAL_TIME: f32 = 2.0;
const FADE: f32 = 0.7;
/// seconds after the fall before the game over menu comes up (the death animation)
const DEATH_PAUSE: f32 = 1.8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Choice {
    NewGame,
    Continue,
    Quit,
}

#[derive(Resource)]
pub struct Title {
    pub active: bool,
    time: f32,
    selected: usize,
    /// the pause menu (Esc) is open, and the item chosen in it
    pub paused: bool,
    pause_index: usize,
    /// the game over menu is open, and the item chosen in it
    pub over: bool,
    over_index: usize,
    froze: bool,
    can_continue: bool,
}

impl Title {
    pub fn new(active: bool) -> Title {
        // LS_TITLE_T=seconds starts the sequence that far in (screenshots)
        let time = std::env::var("LS_TITLE_T").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
        Title { active, time, selected: 0, paused: false, pause_index: 0, over: false, over_index: 0, froze: false, can_continue: false }
    }

    fn ready(&self) -> bool {
        self.time >= CLIMAX_END + REVEAL_TIME
    }

    /// The items of the title menu with whether each can be chosen.
    fn items(&self) -> [(&'static str, bool); 3] {
        [("NEW GAME", true), ("CONTINUE", self.can_continue), ("QUIT", true)]
    }

    fn over_items(&self) -> [(&'static str, bool); 3] {
        [("CONTINUE", true), ("LOAD GAME", self.can_continue), ("QUIT TO TITLE", true)]
    }

    fn pause_items(&self) -> [(&'static str, bool); 4] {
        [("RESUME", true), ("LOAD GAME", self.can_continue), ("QUIT TO TITLE", true), ("QUIT", true)]
    }
}

/// Moves to the next item that can be chosen.
fn step(index: usize, step: i32, items: &[(&str, bool)]) -> usize {
    let n = items.len() as i32;
    let mut i = index as i32;
    for _ in 0..n {
        i = (i + step + n) % n;
        if items[i as usize].1 {
            break;
        }
    }
    i as usize
}

/// The keys of the title and of the pause menu, and what they lead to.
#[allow(clippy::too_many_arguments)]
pub fn title_input(
    time: Res<Time>,
    ctl: Res<Controls>,
    mut title: ResMut<Title>,
    mut frozen: ResMut<GameFrozen>,
    mut game: ResMut<GameState>,
    mut combat: ResMut<Combat>,
    mut cut: ResMut<Cutscene>,
    script: NonSend<Scripting>,
    mut sound: ResMut<Sound>,
    mut sfx: ResMut<SfxQueue>,
    (mut request, mut arrival, data): (ResMut<RoomRequest>, ResMut<Arrival>, Res<DataDir>),
    mut msg: ResMut<Message>,
    mut exit: MessageWriter<AppExit>,
) {
    let save = crate::save::path_of(&data.0);
    if title.active {
        frozen.0 = true;
        title.time += time.delta_secs();
        title.can_continue = save.exists();
        sound.room_song = Some(0x10); // the title music
        if ctl.quit.just {
            exit.write(AppExit::Success);
            return;
        }
        let any = ctl.jump.just || ctl.attack.just || ctl.action.just || ctl.menu.just || ctl.dir.iter().any(|d| d.just);
        if !title.ready() {
            // any key skips the splash and the wipe
            if any {
                title.time = CLIMAX_END + REVEAL_TIME + 0.2;
            }
            return;
        }
        let items = title.items();
        if ctl.dir[0].just {
            title.selected = step(title.selected, -1, &items);
            sfx.play(0x42);
        }
        if ctl.dir[1].just {
            title.selected = step(title.selected, 1, &items);
            sfx.play(0x42);
        }
        if (ctl.action.just || ctl.attack.just || ctl.jump.just) && items[title.selected].1 {
            sfx.play(0x43);
            let choice = [Choice::NewGame, Choice::Continue, Choice::Quit][title.selected];
            match choice {
                Choice::Quit => {
                    exit.write(AppExit::Success);
                }
                Choice::NewGame => {
                    // the adventure from its start: nothing carried over, the intro scenes (room 139) run by the story
                    game.reset();
                    script.replace_flags(&[]);
                    combat.revive();
                    combat.charge = 0.0;
                    // the intro as the original plays it, recorded from the ROM (data/cutscenes/intro.track), else the story's own scripts
                    match std::fs::read_to_string(data.0.join("cutscenes/intro.track")) {
                        Ok(text) => cut.begin_intro_track(crate::track::Track::parse(&text), 139),
                        Err(_) => cut.begin_intro(),
                    }
                    request.0 = Some(139);
                    arrival.0 = Some(Vec2::new(62.5, 24.5));
                    title.active = false;
                    frozen.0 = false;
                    msg.time = 0.0;
                }
                Choice::Continue => {
                    if let Some(s) = crate::save::read(&save) {
                        crate::save::restore(&s, &mut game, &script);
                        *cut = Cutscene::default();
                        combat.revive();
                        request.0 = Some(s.room);
                        arrival.0 = Some(Vec2::new(s.x, s.y));
                        title.active = false;
                        frozen.0 = false;
                        msg.show("Game loaded.");
                    }
                }
            }
        }
        return;
    }

    // the game over menu: once Nigel is on the ground
    if combat.dead && !title.paused && combat.death_t > DEATH_PAUSE && !title.over {
        title.over = true;
        title.over_index = 0;
        msg.time = 0.0;
    }
    if title.over {
        title.can_continue = save.exists();
        let items = title.over_items();
        if ctl.dir[0].just {
            title.over_index = step(title.over_index, -1, &items);
            sfx.play(0x42);
        }
        if ctl.dir[1].just {
            title.over_index = step(title.over_index, 1, &items);
            sfx.play(0x42);
        }
        if ctl.action.just && items[title.over_index].1 {
            sfx.play(0x43);
            title.over = false;
            match title.over_index {
                0 => combat.respawn(&mut game, &mut request, &mut arrival, &mut msg),
                1 => {
                    if let Some(s) = crate::save::read(&save) {
                        crate::save::restore(&s, &mut game, &script);
                        *cut = Cutscene::default();
                        combat.revive();
                        request.0 = Some(s.room);
                        arrival.0 = Some(Vec2::new(s.x, s.y));
                        msg.show("Game loaded.");
                    }
                }
                _ => {
                    title.active = true;
                    title.time = CLIMAX_END + REVEAL_TIME + 0.2;
                    title.selected = 0;
                }
            }
        }
        return;
    }

    // the pause menu
    if title.paused {
        title.can_continue = save.exists();
        let items = title.pause_items();
        let mut close = ctl.quit.just || ctl.attack.just || ctl.menu.just;
        if ctl.dir[0].just {
            title.pause_index = step(title.pause_index, -1, &items);
            sfx.play(0x42);
        }
        if ctl.dir[1].just {
            title.pause_index = step(title.pause_index, 1, &items);
            sfx.play(0x42);
        }
        if ctl.action.just && items[title.pause_index].1 {
            sfx.play(0x43);
            match title.pause_index {
                0 => close = true,
                1 => {
                    if let Some(s) = crate::save::read(&save) {
                        crate::save::restore(&s, &mut game, &script);
                        *cut = Cutscene::default();
                        combat.revive();
                        request.0 = Some(s.room);
                        arrival.0 = Some(Vec2::new(s.x, s.y));
                        msg.show("Game loaded.");
                    }
                    close = true;
                }
                2 => {
                    title.paused = false;
                    title.active = true;
                    title.time = CLIMAX_END + REVEAL_TIME + 0.2; // straight to the menu
                    title.selected = 0;
                    return;
                }
                _ => {
                    exit.write(AppExit::Success);
                    return;
                }
            }
        }
        if close {
            title.paused = false;
            // given back a frame later: the button that closed it must not also act in the game
            title.froze = true;
        }
        return;
    }
    if title.froze {
        title.froze = false;
        frozen.0 = false;
        return;
    }
    // Esc during the game: the pause menu (not during a scene of the story that has the controls, or a text)
    if ctl.quit.just && !frozen.0 && !cut.active() && !combat.dead {
        title.paused = true;
        title.pause_index = 0;
        frozen.0 = true;
        sfx.play(0x68);
    }
}

/// The title and the pause menu on the screen.
pub fn title_overlay(mut contexts: EguiContexts, title: Res<Title>, mut textures: Local<Option<Option<(egui::TextureHandle, egui::TextureHandle)>>>) -> Result {
    if !title.active && !title.paused && !title.over {
        return Ok(());
    }
    let ctx = contexts.ctx_mut()?.clone();
    let screen = ctx.content_rect();
    let area = egui::Rect::from_min_size(screen.min, egui::vec2(screen.width() - crate::playtest::PANEL_W, screen.height()));
    if title.over {
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("over_dim")));
        painter.rect_filled(area, 0.0, egui::Color32::from_black_alpha(150));
        egui::Area::new(egui::Id::new("game_over")).fixed_pos(egui::pos2(area.center().x - 200.0, area.center().y - 110.0)).interactable(false).show(&ctx, |ui| {
            egui::Frame::new().fill(egui::Color32::from_rgba_unmultiplied(20, 20, 60, 245)).stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(220, 220, 240))).inner_margin(egui::Margin::same(16)).show(ui, |ui| {
                ui.set_min_width(380.0);
                ui.label(egui::RichText::new("GAME OVER").size(30.0).color(egui::Color32::from_rgb(240, 90, 90)));
                ui.add_space(10.0);
                for (i, (label, enabled)) in title.over_items().iter().enumerate() {
                    let color = if !enabled { egui::Color32::DARK_GRAY } else if i == title.over_index { egui::Color32::from_rgb(250, 230, 120) } else { egui::Color32::WHITE };
                    ui.label(egui::RichText::new(format!("{} {}{}", if i == title.over_index { ">" } else { " " }, label, if *enabled { "" } else { " (NO SAVE)" })).size(24.0).monospace().color(color));
                }
            });
        });
        return Ok(());
    }
    if title.paused {
        egui::Area::new(egui::Id::new("pause")).fixed_pos(egui::pos2(area.center().x - 200.0, area.center().y - 130.0)).interactable(false).show(&ctx, |ui| {
            egui::Frame::new().fill(egui::Color32::from_rgba_unmultiplied(20, 20, 60, 245)).stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(220, 220, 240))).inner_margin(egui::Margin::same(16)).show(ui, |ui| {
                ui.set_min_width(380.0);
                ui.label(egui::RichText::new("PAUSE").size(26.0).color(egui::Color32::WHITE));
                ui.add_space(10.0);
                for (i, (label, enabled)) in title.pause_items().iter().enumerate() {
                    let color = if !enabled { egui::Color32::DARK_GRAY } else if i == title.pause_index { egui::Color32::from_rgb(250, 230, 120) } else { egui::Color32::WHITE };
                    ui.label(egui::RichText::new(format!("{} {}{}", if i == title.pause_index { ">" } else { " " }, label, if *enabled { "" } else { " (NO SAVE)" })).size(24.0).monospace().color(color));
                }
            });
        });
        return Ok(());
    }
    // the pictures, as textures
    let pics = textures.get_or_insert_with(|| {
        let root = std::path::PathBuf::from(std::env::var("BEVY_ASSET_ROOT").unwrap_or_default()).join("data/graphics");
        let load = |name: &str| {
            let img = image::open(root.join(name)).ok()?.to_rgba8();
            let size = [img.width() as usize, img.height() as usize];
            Some(ctx.load_texture(name, egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw()), egui::TextureOptions::NEAREST))
        };
        Some((load("title.png")?, load("climax.png")?))
    });
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("title")));
    painter.rect_filled(area, 0.0, egui::Color32::BLACK);
    let Some((picture, climax)) = pics.as_ref() else {
        painter.text(area.center(), egui::Align2::CENTER_CENTER, "LANDSTALKER", egui::FontId::monospace(48.0), egui::Color32::WHITE);
        return Ok(());
    };
    let centred = |w: f32, h: f32| egui::Rect::from_center_size(area.center(), egui::vec2(w, h));
    let full_uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
    if title.time < CLIMAX_END {
        // the Climax logo: fades in, holds, fades out
        let a = if title.time < FADE {
            title.time / FADE
        } else if title.time > CLIMAX_END - FADE {
            (CLIMAX_END - title.time) / FADE
        } else {
            1.0
        };
        let [w, h] = climax.size();
        painter.image(climax.id(), centred(w as f32 * 4.0, h as f32 * 4.0), full_uv, egui::Color32::from_white_alpha((a.clamp(0.0, 1.0) * 255.0) as u8));
        return Ok(());
    }
    // the title picture, wiped in from the left
    let [pw, ph] = picture.size();
    let scale = (area.width() / pw as f32).min(area.height() / ph as f32).floor().max(1.0);
    let rect = centred(pw as f32 * scale, ph as f32 * scale);
    let reveal = ((title.time - CLIMAX_END) / REVEAL_TIME).clamp(0.0, 1.0);
    let shown = egui::Rect::from_min_max(rect.min, egui::pos2(rect.min.x + rect.width() * reveal, rect.max.y));
    painter.image(picture.id(), shown, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(reveal, 1.0)), egui::Color32::WHITE);
    if title.ready() {
        // between the logo and the copyright line of the picture (the C++ engine puts its menu box there)
        let y = rect.min.y + 560.0 * scale / 4.0;
        for (i, (label, enabled)) in title.items().iter().enumerate() {
            let selected = i == title.selected;
            let color = if !enabled { egui::Color32::DARK_GRAY } else if selected { egui::Color32::from_rgb(250, 230, 120) } else { egui::Color32::WHITE };
            let text = format!("{} {}{}", if selected { ">" } else { " " }, label, if *enabled { "" } else { " (NO SAVE)" });
            painter.text(egui::pos2(area.center().x, y + i as f32 * 36.0 * scale / 4.0), egui::Align2::CENTER_CENTER, text, egui::FontId::monospace(28.0), color);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cursor_skips_what_cannot_be_chosen() {
        let items = [("NEW GAME", true), ("CONTINUE", false), ("QUIT", true)];
        assert_eq!(step(0, 1, &items), 2);
        assert_eq!(step(2, 1, &items), 0);
        assert_eq!(step(0, -1, &items), 2);
        assert_eq!(step(2, -1, &items), 0);
    }

    #[test]
    fn the_title_waits_for_the_splash_and_the_wipe() {
        let mut t = Title::new(true);
        t.time = 0.0;
        assert!(!t.ready());
        t.time = CLIMAX_END + REVEAL_TIME;
        assert!(t.ready());
        assert!(!t.items()[1].1, "no save, no continue");
        t.can_continue = true;
        assert!(t.items()[1].1);
    }
}
