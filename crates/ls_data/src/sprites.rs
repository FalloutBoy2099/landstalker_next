//! Pictures of an entity or a sprite: poses (idle, walk, attack, hurt, sword...), each seen from one or more views, each view
//! a list of pictures. Which view is drawn for a direction of the entity, and whether it is mirrored, is a table of the sprite
//! (`directions`) whose default is the console's: NE and NW are seen from behind, SE and SW from the front, NW and SE mirrored.
//! See docs/sprites_format.md.

use serde::Deserialize;
use std::collections::BTreeMap;

/// One picture: the file (relative to the folder of the entity, the sprite or the room), its size and the pixel that sits on the
/// position of the entity.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Pic {
    pub file: String,
    pub w: i32,
    pub h: i32,
    pub ax: i32,
    pub ay: i32,
}

/// The four directions of the isometric view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    NE,
    SE,
    SW,
    NW,
}

impl Dir {
    /// An entity's orientation: 0 NE, 1 SE, 2 SW, 3 NW.
    pub fn from_orientation(o: i32) -> Dir {
        [Dir::NE, Dir::SE, Dir::SW, Dir::NW][(o & 3) as usize]
    }

    /// Nigel's facing: 0 NW, 1 NE, 2 SE, 3 SW.
    pub fn from_facing(f: u8) -> Dir {
        [Dir::NW, Dir::NE, Dir::SE, Dir::SW][(f & 3) as usize]
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// What the pictures of a pose are seen from: `back` and `front` are the console's two views, `any` a picture that serves for
/// every direction; `ne`, `se`, `sw`, `nw` are pictures drawn for one direction only (never mirrored).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Views {
    #[serde(default)]
    pub any: Vec<Pic>,
    #[serde(default)]
    pub back: Vec<Pic>,
    #[serde(default)]
    pub front: Vec<Pic>,
    #[serde(default)]
    pub ne: Vec<Pic>,
    #[serde(default)]
    pub se: Vec<Pic>,
    #[serde(default)]
    pub sw: Vec<Pic>,
    #[serde(default)]
    pub nw: Vec<Pic>,
}

/// For a direction: the view to draw ("back", "front" or "any") and whether to mirror it.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DirSpec {
    pub view: String,
    #[serde(default)]
    pub mirror: bool,
}

/// The table of the four directions (`directions:` of a sprite; the console's when there is none).
#[derive(Debug, Clone, PartialEq)]
pub struct Directions([DirSpec; 4]);

impl Default for Directions {
    fn default() -> Self {
        let s = |v: &str, m: bool| DirSpec { view: v.to_string(), mirror: m };
        Directions([s("back", false), s("front", true), s("front", false), s("back", true)])
    }
}

impl<'de> Deserialize<'de> for Directions {
    /// `{NE: {view: back, mirror: false}, ...}`: the directions that are not given keep the console's.
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let map = BTreeMap::<String, DirSpec>::deserialize(d)?;
        let mut out = Directions::default();
        for (name, spec) in map {
            let dir = match name.to_uppercase().as_str() {
                "NE" => Dir::NE,
                "SE" => Dir::SE,
                "SW" => Dir::SW,
                "NW" => Dir::NW,
                other => return Err(serde::de::Error::custom(format!("unknown direction {}", other))),
            };
            out.0[dir.index()] = spec;
        }
        Ok(out)
    }
}

impl Directions {
    pub fn get(&self, dir: Dir) -> &DirSpec {
        &self.0[dir.index()]
    }
}

impl Views {
    /// Every picture of the pose, whatever the view.
    pub fn pictures(&self) -> impl Iterator<Item = &Pic> {
        self.any.iter().chain(&self.back).chain(&self.front).chain(&self.ne).chain(&self.se).chain(&self.sw).chain(&self.nw)
    }

    /// The pictures to draw for a direction, and whether to mirror them: a picture drawn for that direction; else the view the
    /// table of directions says; else the pictures for any direction. None when the pose has no picture for it.
    pub fn pick(&self, dir: Dir, dirs: &Directions) -> Option<(&[Pic], bool)> {
        let own = match dir {
            Dir::NE => &self.ne,
            Dir::SE => &self.se,
            Dir::SW => &self.sw,
            Dir::NW => &self.nw,
        };
        if !own.is_empty() {
            return Some((own, false));
        }
        let spec = dirs.get(dir);
        let list = match spec.view.as_str() {
            "back" => &self.back,
            "front" => &self.front,
            _ => &self.any,
        };
        if !list.is_empty() {
            return Some((list, spec.mirror));
        }
        if !self.any.is_empty() {
            return Some((&self.any, spec.mirror));
        }
        None
    }
}

/// The poses of an entity or a sprite, by name.
pub type Poses = BTreeMap<String, Views>;

/// What `sprite.yaml` holds that the game reads: the poses (the other keys are the exporter's notes).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SpriteDef {
    #[serde(default)]
    pub poses: Poses,
    #[serde(default)]
    pub directions: Directions,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pic(f: &str) -> Pic {
        Pic { file: f.to_string(), w: 32, h: 48, ax: 16, ay: 40 }
    }

    #[test]
    fn the_console_shows_the_back_for_the_north_and_mirrors_nw_and_se() {
        let v = Views { back: vec![pic("b")], front: vec![pic("f")], ..Default::default() };
        let d = Directions::default();
        assert_eq!(v.pick(Dir::NE, &d).map(|(p, m)| (p[0].file.as_str(), m)), Some(("b", false)));
        assert_eq!(v.pick(Dir::NW, &d).map(|(p, m)| (p[0].file.as_str(), m)), Some(("b", true)));
        assert_eq!(v.pick(Dir::SE, &d).map(|(p, m)| (p[0].file.as_str(), m)), Some(("f", true)));
        assert_eq!(v.pick(Dir::SW, &d).map(|(p, m)| (p[0].file.as_str(), m)), Some(("f", false)));
    }

    #[test]
    fn a_picture_for_any_direction_is_mirrored_like_the_others_and_a_drawn_one_never() {
        let v = Views { any: vec![pic("a")], sw: vec![pic("own")], ..Default::default() };
        let d = Directions::default();
        assert_eq!(v.pick(Dir::SE, &d).map(|(p, m)| (p[0].file.as_str(), m)), Some(("a", true)));
        assert_eq!(v.pick(Dir::SW, &d).map(|(p, m)| (p[0].file.as_str(), m)), Some(("own", false)));
        assert!(Views::default().pick(Dir::NE, &d).is_none());
    }

    #[test]
    fn a_sprite_can_change_the_table() {
        let def: SpriteDef = serde_yaml::from_str("poses:\n  idle:\n    back: [{file: b, w: 1, h: 1, ax: 0, ay: 0}]\ndirections:\n  NW: {view: back, mirror: false}\n").unwrap();
        let (p, m) = def.poses["idle"].pick(Dir::NW, &def.directions).unwrap();
        assert_eq!((p[0].file.as_str(), m), ("b", false));
        // the other directions keep the console's
        assert!(def.poses["idle"].pick(Dir::NE, &def.directions).is_some());
    }
}
