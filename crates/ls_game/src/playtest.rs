//! The playtest panel (egui, on the right of the window): room number and room changer, health, notes per room
//! (saved in playtest_notes.yaml next to the exe) and a capture button (F12): the game freezes, drag a rectangle
//! over the picture and it is saved in screenshots/ next to the exe, its name being added to the room's notes.

use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use std::collections::BTreeMap;
use std::path::PathBuf;

use super::game::GameState;
use super::{ControlLock, GameFrozen, RoomIndex, RoomRequest, RoomRes};

pub const PANEL_W: f32 = 420.0;

enum Capture {
    Idle,
    /// the screenshot is being written (frames waited)
    Pending(u32),
    /// choosing the area
    Selecting { texture: egui::TextureHandle, size: [usize; 2], image: image::RgbaImage, start: Option<egui::Pos2>, now: Option<egui::Pos2> },
}

#[derive(Resource)]
pub struct Playtest {
    notes: BTreeMap<i32, String>,
    shown_room: i32,
    text: String,
    room_field: String,
    dirty: bool,
    save_timer: f32,
    message: Option<(String, f32)>,
    capture: Capture,
    dir: PathBuf,
    /// the flags table: which block of 128 flags is shown (4 = those above 511 that are set), and only the flags that are set
    flags_block: usize,
    flags_set_only: bool,
}

impl Playtest {
    pub fn new() -> Playtest {
        let dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf())).unwrap_or_else(|| PathBuf::from("."));
        let notes = std::fs::read_to_string(dir.join("playtest_notes.yaml"))
            .ok()
            .and_then(|t| serde_yaml::from_str::<BTreeMap<String, String>>(&t).ok())
            .map(|m| m.into_iter().filter_map(|(k, v)| k.parse::<i32>().ok().map(|k| (k, v))).collect())
            .unwrap_or_default();
        Playtest { notes, shown_room: -1, text: String::new(), room_field: String::new(), dirty: false, save_timer: 0.0, message: None, capture: Capture::Idle, dir, flags_block: 0, flags_set_only: false }
    }

    fn save(&mut self) {
        let map: BTreeMap<String, &String> = self.notes.iter().filter(|(_, v)| !v.is_empty()).map(|(k, v)| (k.to_string(), v)).collect();
        if let Ok(text) = serde_yaml::to_string(&map) {
            let _ = std::fs::write(self.dir.join("playtest_notes.yaml"), text);
        }
        self.dirty = false;
        self.save_timer = 0.0;
    }

    fn commit(&mut self) {
        if self.shown_room >= 0 {
            if self.text.is_empty() {
                self.notes.remove(&self.shown_room);
            } else {
                self.notes.insert(self.shown_room, self.text.clone());
            }
        }
    }

    fn say(&mut self, text: impl Into<String>) {
        self.message = Some((text.into(), 6.0));
    }
}

/// F12 starts a capture from anywhere.
pub fn capture_hotkey(keys: Res<ButtonInput<KeyCode>>, mut pt: ResMut<Playtest>, mut commands: Commands, mut frozen: ResMut<GameFrozen>) {
    if keys.just_pressed(KeyCode::F12) {
        start_capture(&mut pt, &mut commands, &mut frozen);
    }
}

fn start_capture(pt: &mut Playtest, commands: &mut Commands, frozen: &mut GameFrozen) {
    if !matches!(pt.capture, Capture::Idle) {
        return;
    }
    let dir = pt.dir.join("screenshots");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("_capture.png");
    let _ = std::fs::remove_file(&path);
    commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    pt.capture = Capture::Pending(0);
    frozen.0 = true;
}

fn timestamp() -> String {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) as i64;
    let (days, rem) = (secs.div_euclid(86400), secs.rem_euclid(86400));
    // civil from days (Howard Hinnant)
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{:04}{:02}{:02}_{:02}{:02}{:02}", y, m, d, rem / 3600, (rem % 3600) / 60, rem % 60)
}

/// What the panel needs besides the playtest: the effects, the TAS and the recorder (a bundle: a system takes only so many parameters).
#[derive(bevy::ecs::system::SystemParam)]
pub struct DebugTools<'w, 's> {
    pub config: ResMut<'w, crate::config::Config>,
    pub effects: ResMut<'w, crate::effects::EffectsData>,
    pub tas: ResMut<'w, crate::tas::TasPlayer>,
    pub recorder: ResMut<'w, crate::bot::Recorder>,
    pub status: Local<'s, String>,
}

pub fn panel(
    mut contexts: EguiContexts,
    mut pt: ResMut<Playtest>,
    room: Res<RoomRes>,
    index: Res<RoomIndex>,
    mut game: ResMut<GameState>,
    mut request: ResMut<RoomRequest>,
    mut frozen: ResMut<GameFrozen>,
    lock: Res<ControlLock>,
    players: Query<&crate::Player>,
    mut save_request: ResMut<crate::save::SaveRequest>,
    mut tools: DebugTools,
    mut cut: ResMut<crate::cutscene::Cutscene>,
    script: NonSend<crate::scripting::Scripting>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) -> Result {
    let ctx = contexts.ctx_mut()?.clone();
    let dt = time.delta_secs();
    let id = room.room.file.id;

    // follow the room
    if pt.shown_room != id {
        pt.commit();
        if pt.dirty {
            pt.save();
        }
        pt.shown_room = id;
        pt.text = pt.notes.get(&id).cloned().unwrap_or_default();
        pt.room_field = id.to_string();
    }
    if let Some((_, t)) = pt.message.as_mut() {
        *t -= dt;
    }
    if pt.message.as_ref().map(|m| m.1 <= 0.0).unwrap_or(false) {
        pt.message = None;
    }
    if pt.dirty {
        pt.save_timer += dt;
        if pt.save_timer > 0.7 {
            pt.commit();
            pt.save();
        }
    }

    // capture in progress
    if let Capture::Pending(n) = pt.capture {
        let path = pt.dir.join("screenshots").join("_capture.png");
        let loaded = if path.exists() { image::open(&path).ok().map(|i| i.to_rgba8()) } else { None };
        match loaded {
            Some(img) => {
                let size = [img.width() as usize, img.height() as usize];
                let color = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
                let texture = ctx.load_texture("capture", color, egui::TextureOptions::NEAREST);
                let _ = std::fs::remove_file(&path);
                pt.capture = Capture::Selecting { texture, size, image: img, start: None, now: None };
            }
            None if n > 180 => {
                pt.capture = Capture::Idle;
                frozen.0 = false;
                pt.say("The capture failed");
            }
            None => pt.capture = Capture::Pending(n + 1),
        }
    }
    if matches!(pt.capture, Capture::Selecting { .. }) {
        return selection_ui(&ctx, &mut pt, &mut frozen, id, window.width());
    }

    let mut go: Option<i32> = None;
    let mut want_capture = false;
    let mut viewport_ui = egui::Ui::new(ctx.clone(), "viewport".into(), egui::UiBuilder::new().layer_id(egui::LayerId::background()).max_rect(ctx.viewport_rect()));
    egui::Panel::right("playtest").exact_size(PANEL_W).resizable(false).show(&mut viewport_ui, |ui| {
      egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        // < Room x >  description
        ui.horizontal(|ui| {
            if ui.button("<").clicked() {
                go = index.neighbour(id, -1);
            }
            let r = ui.add(egui::TextEdit::singleline(&mut pt.room_field).desired_width(46.0));
            if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                go = pt.room_field.trim().parse().ok();
            }
            if ui.button(">").clicked() {
                go = index.neighbour(id, 1);
            }
            ui.label(egui::RichText::new(room.room.file.path.rsplit('/').next().unwrap_or("")).strong());
        });
        ui.label(egui::RichText::new(&room.room.file.path).small().weak());
        // < HP x > >>   < EkeEke > >>   < Gold > >>
        ui.horizontal(|ui| {
            if ui.small_button("<").clicked() {
                let n = game.max_hp() - 1;
                game.set_max_hp(n);
            }
            ui.label(format!("HP {}/{}", game.hp(), game.max_hp()));
            if ui.small_button(">").clicked() {
                let n = game.max_hp() + 1;
                game.set_max_hp(n);
            }
            if ui.small_button(">>").on_hover_text("heal fully").clicked() {
                game.health = game.max_health;
            }
            ui.separator();
            if ui.small_button("<").clicked() && game.item_count(crate::game::ITEM_EKEEKE) > 0 {
                game.use_item(crate::game::ITEM_EKEEKE);
            }
            ui.label(format!("EkeEke {}", game.item_count(crate::game::ITEM_EKEEKE)));
            if ui.small_button(">").clicked() {
                game.add_item(crate::game::ITEM_EKEEKE);
            }
            if ui.small_button(">>").on_hover_text("+10").clicked() {
                for _ in 0..10 {
                    game.add_item(crate::game::ITEM_EKEEKE);
                }
            }
        });
        ui.horizontal(|ui| {
            if ui.small_button("<").clicked() {
                game.gold = (game.gold - 10).max(0);
            }
            ui.label(format!("Gold {}", game.gold));
            if ui.small_button(">").clicked() {
                game.gold += 10;
            }
            if ui.small_button(">>").on_hover_text("+100").clicked() {
                game.gold += 100;
            }
        });
        ui.separator();
        // X/Y - [You Play][Cutscene]
        ui.horizontal(|ui| {
            if let Ok(p) = players.single() {
                ui.label(format!("X {:.1} / Y {:.1}", p.x, p.y));
            }
            ui.label("-");
            let (play, cutscene) = (lock.0.is_none(), lock.0.is_some());
            let badge = |ui: &mut egui::Ui, text: &str, on: bool, color: egui::Color32| {
                let rich = egui::RichText::new(format!(" {} ", text)).strong();
                ui.label(if on { rich.color(egui::Color32::WHITE).background_color(color) } else { rich.weak() });
            };
            badge(ui, "Play", play, egui::Color32::from_rgb(40, 140, 70));
            badge(ui, "Cutscene", cutscene, egui::Color32::from_rgb(200, 50, 50));
            // who started the game: the player, or an LLM (LS_LLM=1, or any of the test aids LS_BOT / LS_AUTOPLAY)
            ui.label("-");
            let llm = ["LS_LLM", "LS_BOT", "LS_AUTOPLAY"].iter().any(|k| std::env::var(k).is_ok());
            badge(ui, "Human", !llm, egui::Color32::from_rgb(40, 100, 180));
            badge(ui, "LLM", llm, egui::Color32::from_rgb(150, 70, 190));
        });
        if let Some(why) = &lock.0 {
            ui.label(egui::RichText::new(why).small().weak());
        }
        ui.separator();
        // Load / Save / Capture
        ui.horizontal(|ui| {
            if ui.button("Load").clicked() {
                save_request.load = true;
            }
            if ui.button("Save").clicked() {
                save_request.save = true;
            }
            if ui.button("Capture (F12)").clicked() {
                want_capture = true;
            }
            // Record: the keys and the positions of a room go to recordings/room_<id>.txt (one file per room), from the first key pressed
            use crate::bot::RecMode;
            let red = egui::Color32::from_rgb(200, 50, 50);
            let button = |ui: &mut egui::Ui, mode: RecMode, off: &str, on: &str, tip: &str, recorder: &mut crate::bot::Recorder| -> bool {
                let active = recorder.mode == mode;
                let label = if active {
                    let text = if recorder.writing() { on.to_string() } else { format!("{} (waits)", on) };
                    egui::RichText::new(text).color(egui::Color32::WHITE).background_color(red)
                } else {
                    egui::RichText::new(off)
                };
                if ui.button(label).on_hover_text(tip).clicked() {
                    if active {
                        recorder.stop();
                    } else {
                        recorder.start(mode, id);
                    }
                    return true;
                }
                false
            };
            let a = button(ui, RecMode::Room, "Rec room", "Stop rec", "records this room, from your first key; stops when the room changes", &mut tools.recorder);
            let b = button(ui, RecMode::Continuous, "Rec / Stop", "Stop rec", "records every room, one file per room, until you stop", &mut tools.recorder);
            if a || b {
                let m = if tools.recorder.active() { format!("Recording ({:?}) from the first key", tools.recorder.mode) } else { "Recording stopped".to_string() };
                pt.say(m);
            }
        });
        ui.separator();
        // the story flags: every one with its value; a click sets or clears it (the rooms, the entities and the tile swaps follow)
        ui.label(egui::RichText::new(format!("Flags ({} set)", script.flags().len())).strong());
        {
            ui.horizontal_wrapped(|ui| {
                for (i, label) in ["0-127", "128-255", "256-383", "384-511", "512+"].iter().enumerate() {
                    if ui.selectable_label(pt.flags_block == i, *label).clicked() {
                        pt.flags_block = i;
                    }
                }
                ui.checkbox(&mut pt.flags_set_only, "set only");
            });
            let mut list: Vec<i32> = if pt.flags_block < 4 { (pt.flags_block as i32 * 128..(pt.flags_block as i32 + 1) * 128).collect() } else { script.flags().into_iter().filter(|f| *f >= 512).collect() };
            list.sort();
            let mut toggle: Option<(i32, bool)> = None;
            egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                egui::Grid::new("flags_grid").spacing([4.0, 2.0]).show(ui, |ui| {
                    let mut column = 0;
                    for n in list.iter().copied() {
                        let on = script.flag(n);
                        if pt.flags_set_only && !on {
                            continue;
                        }
                        let text = egui::RichText::new(format!("{:>4}", n)).monospace().size(12.0);
                        let button = if on { egui::Button::new(text.color(egui::Color32::BLACK)).fill(egui::Color32::from_rgb(110, 210, 120)) } else { egui::Button::new(text.weak()) };
                        if ui.add(button.min_size(egui::vec2(44.0, 0.0))).on_hover_text(format!("flag {} = {} (byte {}, bit {})", n, if on { "set" } else { "clear" }, n >> 3, n & 7)).clicked() {
                            toggle = Some((n, !on));
                        }
                        column += 1;
                        if column % 6 == 0 {
                            ui.end_row();
                        }
                    }
                });
            });
            if let Some((n, on)) = toggle {
                script.set_flag_to(n, on);
                pt.say(format!("Flag {} {}", n, if on { "set" } else { "cleared" }));
            }
        }
        ui.separator();
        // no text box for notes: a field that has the focus takes every key of the keyboard away from the game
        if let Some((m, _)) = &pt.message {
            ui.colored_label(egui::Color32::from_rgb(120, 230, 140), m);
        }
        ui.separator();
        crate::effects::effects_ui(ui, &mut tools.config, &mut tools.effects, id, &room.room.file.path, &mut tools.status);
        ui.separator();
        crate::tas::tas_ui(ui, id, &mut tools.recorder, &mut tools.tas);
      });
    });
    // a thin frame when the keyboard belongs to the panel (a field has the focus): the game does not hear the keys
    if ctx.egui_wants_keyboard_input() {
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("focus_frame")));
        painter.rect_stroke(ctx.viewport_rect().shrink(1.0), 0.0, egui::Stroke::new(2.0, egui::Color32::from_rgb(240, 160, 40)), egui::StrokeKind::Inside);
    }
    if want_capture {
        start_capture(&mut pt, &mut commands, &mut frozen);
    }
    if let Some(n) = go {
        if index.contains(n) {
            pt.commit();
            request.0 = Some(n);
            crate::versions::ask_directly();
            cut.abort = true;
        } else {
            pt.say(format!("No room {}", n));
            pt.room_field = id.to_string();
        }
    }
    Ok(())
}

fn selection_ui(ctx: &egui::Context, pt: &mut Playtest, frozen: &mut GameFrozen, room_id: i32, window_w: f32) -> Result {
    let mut finish: Option<egui::Rect> = None;
    let mut cancel = false;
    {
        let Capture::Selecting { texture, size, start, now, .. } = &mut pt.capture else { return Ok(()) };
        let screen = ctx.viewport_rect();
        egui::Area::new(egui::Id::new("capture_area")).order(egui::Order::Foreground).fixed_pos(egui::pos2(0.0, 0.0)).show(ctx, |ui| {
            let (rect, resp) = ui.allocate_exact_size(screen.size(), egui::Sense::click_and_drag());
            ui.painter().image(texture.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
            ui.painter().rect_filled(rect, 0.0, egui::Color32::from_black_alpha(90));
            if resp.drag_started() {
                *start = resp.interact_pointer_pos();
                *now = *start;
            }
            if resp.dragged() {
                *now = resp.interact_pointer_pos();
            }
            if let (Some(a), Some(b)) = (*start, *now) {
                let r = egui::Rect::from_two_pos(a, b);
                // show the chosen area without the dimming
                ui.painter().image(
                    texture.id(),
                    r,
                    egui::Rect::from_min_max(egui::pos2(r.min.x / screen.width(), r.min.y / screen.height()), egui::pos2(r.max.x / screen.width(), r.max.y / screen.height())),
                    egui::Color32::WHITE,
                );
                ui.painter().rect_stroke(r, 0.0, egui::Stroke::new(2.0, egui::Color32::YELLOW), egui::StrokeKind::Outside);
            }
        });
        let _ = size;
        egui::Window::new("Capture").fixed_pos(egui::pos2(12.0, 12.0)).collapsible(false).resizable(false).show(ctx, |ui| {
            ui.label("Drag a rectangle over the picture");
            ui.horizontal(|ui| {
                if ui.button("Save the area (Enter)").clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let (Some(a), Some(b)) = (*start, *now) {
                        finish = Some(egui::Rect::from_two_pos(a, b));
                    }
                }
                if ui.button("Whole game").clicked() {
                    finish = Some(egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(screen.width() - PANEL_W, screen.height())));
                }
                if ui.button("Cancel (Esc)").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    cancel = true;
                }
            });
        });
    }
    if cancel {
        pt.capture = Capture::Idle;
        frozen.0 = false;
        return Ok(());
    }
    if let Some(r) = finish {
        let Capture::Selecting { image, size, .. } = std::mem::replace(&mut pt.capture, Capture::Idle) else { return Ok(()) };
        frozen.0 = false;
        let k = size[0] as f32 / window_w.max(1.0);
        let x = (r.min.x * k).max(0.0) as u32;
        let y = (r.min.y * k).max(0.0) as u32;
        let w = ((r.width() * k) as u32).min(image.width().saturating_sub(x));
        let h = ((r.height() * k) as u32).min(image.height().saturating_sub(y));
        if w < 4 || h < 4 {
            pt.say("Area too small");
            return Ok(());
        }
        let crop = image::imageops::crop_imm(&image, x, y, w, h).to_image();
        let name = format!("screenshots/room_{:03}_{}.png", room_id, timestamp());
        match crop.save(pt.dir.join(&name)) {
            Ok(()) => {
                if !pt.text.is_empty() && !pt.text.ends_with('\n') {
                    pt.text.push('\n');
                }
                pt.text.push_str(&format!("[img {}]\n", name));
                pt.dirty = true;
                pt.commit();
                pt.save();
                pt.say(format!("Saved {}", name));
            }
            Err(e) => pt.say(format!("Saving failed: {}", e)),
        }
    }
    Ok(())
}
