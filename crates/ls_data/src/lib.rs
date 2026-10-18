//! Data formats of landstalker_next: room files (`room.yaml` + `map.tmx`), index, and helpers to read cells.
//! See docs/data_formats.md.

pub mod lsa;
pub mod sprites;
pub mod vgm;
pub mod ym2612;

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// One tile of the heightmap layers decoded back to the original 16-bit cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// bits 12..15: 0x4 closed to everybody, 0x2 closed to NPCs, ...
    pub flags: u8,
    /// bits 8..11
    pub height: u8,
    /// bits 6..7
    pub extra: u8,
    /// bits 0..5 (0 normal, 1-4 door directions, 5 stairs, 6 door, 7 pit, 8 warp pad, 0x1A locked door ...)
    pub floor_type: u8,
}

impl Cell {
    /// The cell as stored in the ROM and in the legacy `heightmap.csv`.
    pub fn raw(&self) -> u16 {
        ((self.flags as u16) << 12) | ((self.height as u16) << 8) | ((self.extra as u16) << 6) | self.floor_type as u16
    }
    pub fn blocked(&self) -> bool {
        self.flags & 0x4 != 0
    }
    pub fn closed_to_npcs(&self) -> bool {
        self.flags & 0x2 != 0
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TilesetInfo {
    pub id: i32,
    pub palette: i32,
    pub primary_blockset: i32,
    pub secondary_blockset: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Heights {
    pub z_begin: i32,
    pub z_end: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GridInfo {
    pub origin: [i32; 2],
    pub size: [i32; 2],
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlocksInfo {
    pub origin: [i32; 2],
    pub size: [i32; 2],
    pub tile_count: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MapInfo {
    pub file: String,
    pub preview: String,
    pub palette: String,
    pub block_size_px: i32,
    /// world cell = grid index + cell_offset
    pub cell_offset: i32,
    pub grid: GridInfo,
    pub heightmap: GridInfo,
    pub blocks: BlocksInfo,
    pub legacy: LegacyMapInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LegacyMapInfo {
    pub heightmap_left: i32,
    pub heightmap_top: i32,
    pub pixel_size: [i32; 2],
}

#[derive(Debug, Deserialize, Clone)]
pub struct EntityFlags {
    pub hostile: bool,
    pub solid: bool,
    pub visible: bool,
    pub gravity: bool,
    pub pickable: bool,
    pub talks: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Entity {
    pub id: i32,
    #[serde(rename = "type")]
    pub kind: i32,
    pub type_name: String,
    /// world cell coordinates + height
    pub position: [f64; 3],
    pub orientation: String,
    pub speed: i32,
    pub palette: i32,
    /// legacy script number (data/text/behaviours.txt)
    pub behaviour: i32,
    /// Lua script (replaces `behaviour`), e.g. objects/button
    pub script: Option<String>,
    pub dialogue: Option<i32>,
    pub flags: EntityFlags,
    /// pictures drawn for this entity in this room (palette slots that depend on the room): files are in the room folder;
    /// `idle` only (see docs/sprites_format.md)
    #[serde(default)]
    pub poses: Option<sprites::Poses>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Warp {
    pub kind: String,
    /// [x, y, width, height] in world cells
    pub area: [i32; 4],
    pub target_room: i32,
    pub target_cell: [i32; 2],
}

#[derive(Debug, Deserialize, Clone)]
pub struct Chests {
    pub enabled: bool,
    pub contents: Vec<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoomFile {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub tileset: TilesetInfo,
    pub music: i32,
    pub heights: Heights,
    pub fall_destination: Option<i32>,
    pub climb_destination: Option<i32>,
    pub map: MapInfo,
    pub entities: Vec<Entity>,
    pub warps: Vec<Warp>,
    pub animations: Vec<serde_yaml::Value>,
    pub chests: Chests,
    pub characters: Vec<i32>,
    pub entity_flags: Vec<serde_yaml::Value>,
    pub tile_swaps: Vec<serde_yaml::Value>,
    pub swap_flags: Vec<serde_yaml::Value>,
    pub locked_doors: Vec<serde_yaml::Value>,
    pub switches: Vec<serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IndexEntry {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub entities: usize,
    pub warps: usize,
    pub grid: [i32; 2],
}

#[derive(Debug, Deserialize, Clone)]
pub struct Index {
    pub rooms: Vec<IndexEntry>,
}

/// The decoded tile layers of a `map.tmx` (gids as written in the file, 0 = empty).
#[derive(Debug, Clone)]
pub struct Tmx {
    pub width: usize,
    pub height: usize,
    pub layers: HashMap<String, Vec<u32>>,
    /// first gid of each tileset, by name (`blocks`, `height`, `floor_type`, `cell_flags`, `cell_extra`)
    pub first_gid: HashMap<String, u32>,
}

impl Tmx {
    pub fn parse(text: &str) -> Result<Tmx> {
        let doc = roxmltree::Document::parse(text)?;
        let map = doc.root_element();
        let width: usize = map.attribute("width").ok_or_else(|| anyhow!("no width"))?.parse()?;
        let height: usize = map.attribute("height").ok_or_else(|| anyhow!("no height"))?.parse()?;
        let mut first_gid = HashMap::new();
        let mut layers = HashMap::new();
        for node in map.children().filter(|n| n.is_element()) {
            match node.tag_name().name() {
                "tileset" => {
                    let first: u32 = node.attribute("firstgid").ok_or_else(|| anyhow!("tileset without firstgid"))?.parse()?;
                    let name = match node.attribute("name") {
                        Some(n) => n.to_string(),
                        None => Path::new(node.attribute("source").unwrap_or(""))
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_default(),
                    };
                    first_gid.insert(name, first);
                }
                "layer" => {
                    let name = node.attribute("name").unwrap_or("").to_string();
                    let data = node
                        .children()
                        .find(|c| c.has_tag_name("data"))
                        .and_then(|d| d.text())
                        .ok_or_else(|| anyhow!("layer {} has no data", name))?;
                    let values: Vec<u32> = data
                        .split(',')
                        .map(|v| v.trim().parse::<u32>())
                        .collect::<std::result::Result<_, _>>()
                        .with_context(|| format!("layer {}", name))?;
                    if values.len() != width * height {
                        bail!("layer {} has {} cells, expected {}", name, values.len(), width * height);
                    }
                    layers.insert(name, values);
                }
                _ => {}
            }
        }
        Ok(Tmx { width, height, layers, first_gid })
    }
}

/// A room with its map layers.
#[derive(Debug, Clone)]
pub struct Room {
    pub file: RoomFile,
    pub dir: PathBuf,
    pub tmx: Tmx,
}

impl Room {
    pub fn load(dir: &Path) -> Result<Room> {
        let text = std::fs::read_to_string(dir.join("room.yaml")).with_context(|| format!("{}", dir.display()))?;
        let file: RoomFile = serde_yaml::from_str(&text).with_context(|| format!("{}/room.yaml", dir.display()))?;
        let tmx_text = std::fs::read_to_string(dir.join(&file.map.file))?;
        let tmx = Tmx::parse(&tmx_text).with_context(|| format!("{}/{}", dir.display(), file.map.file))?;
        Ok(Room { file, dir: dir.to_path_buf(), tmx })
    }

    fn tile(&self, layer: &str, tileset: &str, gx: usize, gy: usize) -> Option<u32> {
        let gid = *self.tmx.layers.get(layer)?.get(gy * self.tmx.width + gx)?;
        if gid == 0 {
            return None;
        }
        Some(gid - self.tmx.first_gid.get(tileset)?)
    }

    /// The heightmap cell at a heightmap index (0, 0 = first cell of the original heightmap), if there is one.
    pub fn cell(&self, ix: i32, iy: i32) -> Option<Cell> {
        let gx = ix - self.file.map.grid.origin[0];
        let gy = iy - self.file.map.grid.origin[1];
        if gx < 0 || gy < 0 || gx as usize >= self.tmx.width || gy as usize >= self.tmx.height {
            return None;
        }
        let (gx, gy) = (gx as usize, gy as usize);
        Some(Cell {
            height: self.tile("height", "height", gx, gy)? as u8,
            floor_type: self.tile("floor_type", "floor_type", gx, gy)? as u8,
            flags: self.tile("cell_flags", "cell_flags", gx, gy)? as u8,
            extra: self.tile("cell_extra", "cell_extra", gx, gy)? as u8,
        })
    }

    /// The block index of a layer at a map block position written back (`None` = empty).
    pub fn set_block(&mut self, layer: &str, bx: i32, by: i32, index: Option<u32>) {
        let b = self.file.map.blocks.clone();
        if bx < 0 || by < 0 || bx >= b.size[0] || by >= b.size[1] {
            return;
        }
        let gx = (bx + b.origin[0] - self.file.map.grid.origin[0]) as usize;
        let gy = (by + b.origin[1] - self.file.map.grid.origin[1]) as usize;
        let first = self.tmx.first_gid.get("blocks").copied().unwrap_or(1);
        let width = self.tmx.width;
        if let Some(l) = self.tmx.layers.get_mut(layer) {
            if let Some(v) = l.get_mut(gy * width + gx) {
                *v = index.map_or(0, |i| i + first);
            }
        }
    }

    /// A heightmap cell written back (heightmap index, as `cell`).
    pub fn set_cell(&mut self, ix: i32, iy: i32, c: Cell) {
        let gx = ix - self.file.map.grid.origin[0];
        let gy = iy - self.file.map.grid.origin[1];
        if gx < 0 || gy < 0 || gx as usize >= self.tmx.width || gy as usize >= self.tmx.height {
            return;
        }
        let (gx, gy, width) = (gx as usize, gy as usize, self.tmx.width);
        for (layer, value) in [("height", c.height), ("floor_type", c.floor_type), ("cell_flags", c.flags), ("cell_extra", c.extra)] {
            let first = self.tmx.first_gid.get(layer).copied().unwrap_or(1);
            if let Some(l) = self.tmx.layers.get_mut(layer) {
                if let Some(v) = l.get_mut(gy * width + gx) {
                    *v = first + value as u32;
                }
            }
        }
    }

    /// The cell at world cell coordinates (what entities and warps use).
    pub fn cell_at_world(&self, wx: i32, wy: i32) -> Option<Cell> {
        self.cell(wx - self.file.map.cell_offset, wy - self.file.map.cell_offset)
    }

    /// Block index (into blocks.png, 32 per row) at a map block position, `None` outside the block area.
    pub fn block(&self, layer: &str, bx: i32, by: i32) -> Option<u32> {
        let b = &self.file.map.blocks;
        if bx < 0 || by < 0 || bx >= b.size[0] || by >= b.size[1] {
            return None;
        }
        let gx = bx + b.origin[0] - self.file.map.grid.origin[0];
        let gy = by + b.origin[1] - self.file.map.grid.origin[1];
        self.tile(layer, "blocks", gx as usize, gy as usize)
    }
}

pub fn load_index(data_dir: &Path) -> Result<Index> {
    let text = std::fs::read_to_string(data_dir.join("index.yaml"))?;
    Ok(serde_yaml::from_str(&text)?)
}

pub fn room_dir(data_dir: &Path, id: i32) -> PathBuf {
    data_dir.join("rooms").join(format!("room_{:03}", id))
}
