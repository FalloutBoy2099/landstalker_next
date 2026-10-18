//! 2.5D physics: a box (radius x height) walking, jumping and falling on the heightmap cells and on solid entities.
//! Rules of the original, as the C++ engine reproduces them:
//!  * floor cells are walked over up to `step_up` height units (stair cells: STAIR_STEP), never more (a jump is needed);
//!  * entities (crates, buttons, statues...) are never stepped onto: they block unless the feet are already at their top;
//!    a jump (or a fall) lands on top of them;
//!  * a closed cell (wall / void) or a missing cell blocks.
//! Units: x, y in cells, z in height units (1 unit = 16 pixels of height on screen).

use ls_data::Cell;

/// Values that can be tuned (data/physics.yaml, reloaded with R in the game).
#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Tuning {
    /// height units / s^2
    pub gravity: f32,
    /// height of the top of a jump, in height units (a crate is 1.0 high): the take-off speed follows from it
    pub jump_height: f32,
    pub max_fall_speed: f32,
    /// cells / s
    pub walk_speed: f32,
    /// highest rise of the floor walked over without a jump (the steps of a staircase need a jump), in height units;
    /// a rise onto or off a stair cell (floor type 5) is walked up to STAIR_STEP
    pub step_up: f32,
}

impl Default for Tuning {
    fn default() -> Self {
        // The C++ engine used gravity 30 / jump speed 9 (top 1.35, 0.6 s in the air): too floaty. Same top, faster.
        Tuning { gravity: 50.0, jump_height: 1.35, max_fall_speed: 30.0, walk_speed: 4.5, step_up: 0.5 }
    }
}

impl Tuning {
    pub fn jump_speed(&self) -> f32 {
        (2.0 * self.gravity * self.jump_height).sqrt()
    }
}
pub const PLAYER_HEIGHT: f32 = 2.0;
pub const PLAYER_RADIUS: f32 = 0.4;
/// Rise walked over onto a stair cell (floor type 5).
pub const STAIR_STEP: f32 = 1.0;
/// An entity this low (or lower) above the feet counts as the floor to stand on.
pub const PLATFORM_STEP: f32 = 0.05;
pub const CLIMB_SPEED: f32 = 8.0;

pub trait Terrain {
    /// Cell at world cell coordinates.
    fn cell(&self, cx: i32, cy: i32) -> Option<Cell>;
}

impl Terrain for ls_data::Room {
    fn cell(&self, cx: i32, cy: i32) -> Option<Cell> {
        self.cell_at_world(cx, cy)
    }
}

/// A solid entity: a square footprint (half side `half`, centred on x, y) from height z up by `height`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Solid {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub half: f32,
    pub height: f32,
    /// index of the room entity it comes from (usize::MAX: none)
    pub entity: usize,
}

impl Solid {
    pub fn top(&self) -> f32 {
        self.z + self.height
    }
    fn overlaps_xy(&self, x: f32, y: f32, radius: f32) -> bool {
        (x - self.x).abs() < self.half + radius && (y - self.y).abs() < self.half + radius
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Body {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vz: f32,
    pub grounded: bool,
    /// highest point of the current flight (landing sounds, fall damage)
    pub air_peak: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Events {
    pub jumped: bool,
    pub landed: bool,
    pub hit_head: bool,
}

pub struct World<'a> {
    pub terrain: &'a dyn Terrain,
    pub solids: &'a [Solid],
    pub tuning: Tuning,
}

impl<'a> World<'a> {
    /// The highest floor under the box at (x, y) that can be reached from height z, `None` when a cell blocks it.
    pub fn cells_floor(&self, x: f32, y: f32, z: f32) -> Option<f32> {
        let mut floor = f32::MIN;
        for (dx, dy) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
            let c = self.terrain.cell((x + dx * PLAYER_RADIUS).floor() as i32, (y + dy * PLAYER_RADIUS).floor() as i32)?;
            if c.blocked() {
                return None;
            }
            let h = c.height as f32;
            let allowed = if c.floor_type == 5 { STAIR_STEP.max(self.tuning.step_up) } else { self.tuning.step_up };
            if h > z + allowed + 1e-4 {
                return None;
            }
            floor = floor.max(h);
        }
        Some(floor)
    }

    /// Does a solid entity overlap the box standing at (x, y, z)? Entities whose top is within PLATFORM_STEP of the feet
    /// do not (the box stands on them).
    pub fn solid_blocks(&self, x: f32, y: f32, z: f32) -> bool {
        self.solids.iter().any(|s| {
            s.overlaps_xy(x, y, PLAYER_RADIUS) && z < s.top() - PLATFORM_STEP && z + PLAYER_HEIGHT > s.z
        })
    }

    pub fn can_stand(&self, x: f32, y: f32, z: f32) -> bool {
        self.cells_floor(x, y, z).is_some() && !self.solid_blocks(x, y, z)
    }

    /// Height of what the box would stand on at (x, y) given feet at height z.
    pub fn floor_under(&self, x: f32, y: f32, z: f32) -> f32 {
        let mut floor = self.cells_floor(x, y, z).unwrap_or(z);
        for s in self.solids {
            if s.overlaps_xy(x, y, PLAYER_RADIUS) && s.top() <= z + PLATFORM_STEP && s.top() > floor {
                floor = s.top();
            }
        }
        floor
    }

    /// The entity a box standing at (x, y) with its feet at z rests on, if any.
    pub fn support_of(&self, x: f32, y: f32, z: f32) -> Option<usize> {
        self.solids.iter().find(|s| s.overlaps_xy(x, y, PLAYER_RADIUS) && (s.top() - z).abs() < 0.1 && s.entity != usize::MAX).map(|s| s.entity)
    }

    /// One step of `dt` seconds: horizontal displacement (dx, dy) in cells, jump request.
    pub fn step(&self, b: &mut Body, dx: f32, dy: f32, jump: bool, dt: f32) -> Events {
        let mut ev = Events::default();
        // horizontal, axis by axis so that the box slides along walls
        if dx != 0.0 && self.can_stand(b.x + dx, b.y, b.z) {
            b.x += dx;
        }
        if dy != 0.0 && self.can_stand(b.x, b.y + dy, b.z) {
            b.y += dy;
        }
        // vertical
        if b.grounded && jump {
            b.vz = self.tuning.jump_speed();
            b.grounded = false;
            b.air_peak = b.z;
            ev.jumped = true;
        }
        if b.grounded {
            let floor = self.floor_under(b.x, b.y, b.z);
            if floor > b.z {
                b.z = floor.min(b.z + CLIMB_SPEED * dt); // climbing a stair
            } else if floor < b.z - 1e-4 {
                b.grounded = false; // walked off an edge
                b.vz = 0.0;
                b.air_peak = b.z;
            } else {
                b.z = floor;
            }
        }
        if !b.grounded {
            let previous = b.z;
            b.vz = (b.vz - self.tuning.gravity * dt).max(-self.tuning.max_fall_speed);
            b.z += b.vz * dt;
            b.air_peak = b.air_peak.max(b.z);
            // head against an entity above
            if b.vz > 0.0 {
                let head = self.solids.iter().any(|s| {
                    s.overlaps_xy(b.x, b.y, PLAYER_RADIUS) && s.z >= previous + PLAYER_HEIGHT - 1e-3 && b.z + PLAYER_HEIGHT > s.z
                });
                if head {
                    b.z = previous;
                    b.vz = 0.0;
                    ev.hit_head = true;
                }
            }
            // landing is looked for from where the feet were, so that a thin object cannot be crossed in one frame
            let floor = self.floor_under(b.x, b.y, previous.max(b.z));
            if b.vz <= 0.0 && b.z <= floor {
                b.z = floor;
                b.vz = 0.0;
                b.grounded = true;
                ev.landed = b.air_peak - floor > 0.5;
                b.air_peak = f32::MIN;
            } else if b.vz > 0.0 && b.z < floor {
                b.z = floor; // rose into a ledge within step range
            }
        }
        ev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct Grid(HashMap<(i32, i32), Cell>);
    impl Terrain for Grid {
        fn cell(&self, cx: i32, cy: i32) -> Option<Cell> {
            self.0.get(&(cx, cy)).copied()
        }
    }

    fn cell(height: u8) -> Cell {
        Cell { flags: 0, height, extra: 0, floor_type: 0 }
    }

    /// A 20x20 floor at height 0 with a wall on x = 10, stairs (heights 1, 2) on x = 12, 13, 14 and a ledge of 4 on x >= 15.
    fn grid() -> Grid {
        let mut m = HashMap::new();
        for x in 0..20 {
            for y in 0..20 {
                let h = if x >= 15 { 4 } else if x == 13 || x == 14 { 2 } else if x == 12 { 1 } else { 0 };
                let mut c = cell(h);
                if x == 10 {
                    c.flags = 4;
                }
                m.insert((x, y), c);
            }
        }
        Grid(m)
    }

    fn body(x: f32, y: f32, z: f32) -> Body {
        Body { x, y, z, vz: 0.0, grounded: true, air_peak: f32::MIN }
    }

    fn walk(w: &World, b: &mut Body, dx: f32, dy: f32, seconds: f32) {
        let dt = 1.0 / 60.0;
        let mut t = 0.0;
        while t < seconds {
            w.step(b, dx * dt, dy * dt, false, dt);
            t += dt;
        }
    }

    #[test]
    fn walls_block_and_stairs_are_climbed() {
        let g = grid();
        let w = World { terrain: &g, solids: &[], tuning: Tuning { step_up: 1.0, ..Tuning::default() } };
        let mut b = body(8.5, 5.5, 0.0);
        walk(&w, &mut b, 4.5, 0.0, 1.0);
        assert!(b.x < 10.0, "stopped by the wall, x = {}", b.x);
        let mut b = body(11.5, 5.5, 0.0);
        walk(&w, &mut b, 4.5, 0.0, 0.5);
        assert!((b.z - 2.0).abs() < 1e-3 && b.x > 13.0, "climbed the stairs: {:?}", b);
        // the ledge of 4 is two units too high from the stairs' top: blocked
        walk(&w, &mut b, 4.5, 0.0, 1.0);
        assert!(b.x < 15.0 && (b.z - 2.0).abs() < 1e-3, "blocked by the ledge: {:?}", b);
    }

    #[test]
    fn jump_peak_is_about_1_35() {
        let g = grid();
        let w = World { terrain: &g, solids: &[], tuning: Tuning::default() };
        let mut b = body(5.5, 5.5, 0.0);
        let dt = 1.0 / 60.0;
        w.step(&mut b, 0.0, 0.0, true, dt);
        let mut peak: f32 = 0.0;
        for _ in 0..120 {
            w.step(&mut b, 0.0, 0.0, false, dt);
            peak = peak.max(b.z);
        }
        assert!(peak > 1.25 && peak < 1.45, "peak {}", peak);
        assert!(b.grounded && b.z == 0.0);
    }

    #[test]
    fn a_button_is_not_stepped_onto_but_jumped_on() {
        let g = grid();
        let button = Solid { x: 5.5, y: 5.5, z: 0.0, half: 0.5, height: 0.125, entity: usize::MAX };
        let solids = [button];
        let w = World { terrain: &g, solids: &solids, tuning: Tuning::default() };
        // walking into it: blocked
        let mut b = body(5.5, 3.5, 0.0);
        walk(&w, &mut b, 0.0, 4.5, 1.0);
        assert!(b.y < 5.0 - 0.4 + 0.01 + 0.5 && b.z == 0.0, "blocked: {:?}", b);
        assert!(b.y < 5.2, "did not cross the button: {:?}", b);
        // jumping towards it: lands on top
        let mut b = body(5.5, 4.0, 0.0);
        let dt = 1.0 / 60.0;
        w.step(&mut b, 0.0, 0.0, true, dt);
        for _ in 0..40 {
            w.step(&mut b, 0.0, 4.5 * dt, false, dt);
        }
        assert!(b.grounded, "{:?}", b);
        assert!(b.y > 5.0, "went over the edge of the button: {:?}", b);
    }

    #[test]
    fn a_crate_can_be_stood_on_after_a_jump_and_a_fall_lands_on_it() {
        let g = grid();
        let crate_box = Solid { x: 5.5, y: 5.5, z: 0.0, half: 0.5, height: 1.0, entity: usize::MAX };
        let solids = [crate_box];
        let w = World { terrain: &g, solids: &solids, tuning: Tuning::default() };
        // standing on it: dropped from above onto the crate
        let mut b = Body { x: 5.5, y: 5.5, z: 3.0, vz: 0.0, grounded: false, air_peak: 3.0 };
        for _ in 0..90 {
            w.step(&mut b, 0.0, 0.0, false, 1.0 / 60.0);
        }
        assert!(b.grounded && (b.z - 1.0).abs() < 1e-4, "landed on the crate: {:?}", b);
        // walking off the crate: falls to the ground
        for _ in 0..60 {
            w.step(&mut b, 0.0, 4.5 / 60.0, false, 1.0 / 60.0);
        }
        assert!(b.grounded && b.z == 0.0, "back on the floor: {:?}", b);
    }

    #[test]
    fn steps_of_a_unit_need_a_jump() {
        let g = grid();
        let w = World { terrain: &g, solids: &[], tuning: Tuning::default() };
        let mut b = body(11.5, 5.5, 0.0);
        walk(&w, &mut b, 4.5, 0.0, 0.5);
        assert!(b.x < 12.0 && b.z == 0.0, "stopped by a step of 1: {:?}", b);
        // a jump gets onto it
        let dt = 1.0 / 60.0;
        w.step(&mut b, 0.0, 0.0, true, dt);
        for _ in 0..40 {
            w.step(&mut b, 4.5 * dt, 0.0, false, dt);
        }
        assert!(b.grounded && b.x > 12.0 && b.z >= 1.0, "jumped onto the step: {:?}", b);
    }

    #[test]
    fn walking_off_a_ledge_falls() {
        let g = grid();
        let w = World { terrain: &g, solids: &[], tuning: Tuning::default() };
        let mut b = body(15.5, 5.5, 4.0);
        walk(&w, &mut b, -4.5, 0.0, 0.5);
        assert!(b.grounded && b.z == 2.0 && b.x < 15.0, "fell from the ledge (4) onto the stairs (2): {:?}", b);
    }
}
