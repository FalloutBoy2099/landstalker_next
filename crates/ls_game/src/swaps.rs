//! Tile swaps: parts of a room's map (blocks and heightmap) that are replaced by other parts of the map once a story flag is
//! set (a bridge appears, a wall opens...). Port of RoomView::ApplyTileSwaps of the C++ engine.

use ls_data::Room;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Swap {
    pub trigger: i32,
    pub mode: i32,
    pub map: [i32; 6],
    pub hm: [i32; 6],
}

#[derive(Clone, Debug)]
pub struct SwapFlag {
    pub trigger: i32,
    pub flag: i32,
    pub always: bool,
}

fn ints<const N: usize>(v: Option<&serde_yaml::Value>) -> [i32; N] {
    let mut out = [0; N];
    if let Some(seq) = v.and_then(|v| v.as_sequence()) {
        for (i, x) in seq.iter().take(N).enumerate() {
            out[i] = x.as_i64().unwrap_or(0) as i32;
        }
    }
    out
}

pub fn parse(room: &Room) -> (Vec<Swap>, Vec<SwapFlag>) {
    let swaps = room
        .file
        .tile_swaps
        .iter()
        .map(|t| Swap { trigger: t.get("trigger").and_then(|v| v.as_i64()).unwrap_or(0) as i32, mode: t.get("mode").and_then(|v| v.as_i64()).unwrap_or(0) as i32, map: ints::<6>(t.get("map")), hm: ints::<6>(t.get("hm")) })
        .collect();
    let flags = room
        .file
        .swap_flags
        .iter()
        .map(|t| SwapFlag { trigger: t.get("trigger").and_then(|v| v.as_i64()).unwrap_or(0) as i32, flag: t.get("flag").and_then(|v| v.as_i64()).unwrap_or(0) as i32, always: t.get("always").and_then(|v| v.as_bool()).unwrap_or(false) })
        .collect();
    (swaps, flags)
}

/// Is block (x, y) of this layer inside the destination area of the swap?
fn inside(sw: &Swap, layer: usize, x: i32, y: i32, hm_left: i32, hm_top: i32) -> bool {
    let (dst_x, dst_y) = (sw.map[2], sw.map[3]);
    let (mw, mh) = (sw.map[4], sw.map[5]);
    let (xo, yo) = (x - dst_x + hm_left, y - dst_y + hm_top);
    match sw.mode {
        0 => xo >= 0 && xo < mw && yo >= 0 && yo < mh,
        1 => {
            if layer == 0 {
                xo - yo >= 0 && xo - yo < mw && yo >= 0 && yo < mh
            } else {
                xo - yo - 1 >= 0 && xo - yo - 1 < mw && yo >= 0 && yo < mh
            }
        }
        2 => {
            if layer == 0 {
                xo >= 0 && xo < mh && yo > xo && yo <= xo + mw
            } else {
                xo >= 0 && xo < mh && yo >= xo && yo < xo + mw
            }
        }
        _ => false,
    }
}

const LAYERS: [&str; 2] = ["background", "foreground"];

/// Every block (layer, bx, by) any swap can write.
pub fn affected(room: &Room, swaps: &[Swap], hm_left: i32, hm_top: i32) -> HashSet<(usize, i32, i32)> {
    let size = room.file.map.blocks.size;
    let mut out = HashSet::new();
    for sw in swaps {
        for layer in 0..2 {
            for y in 0..size[1] {
                for x in 0..size[0] {
                    if inside(sw, layer, x, y, hm_left, hm_top) {
                        out.insert((layer, x, y));
                    }
                }
            }
        }
    }
    out
}

/// The room's layers as they were loaded.
pub type Base = HashMap<String, Vec<u32>>;

/// Puts the base map back, then applies the swaps whose flag is set (or that are always on).
pub fn apply(room: &mut Room, base: &Base, swaps: &[Swap], swap_flags: &[SwapFlag], flag: &dyn Fn(i32) -> bool, hm_left: i32, hm_top: i32) {
    room.tmx.layers = base.clone();
    let active: Vec<i32> = swap_flags.iter().filter(|s| s.always || flag(s.flag)).map(|s| s.trigger).collect();
    let size = room.file.map.blocks.size;
    let hm = room.file.map.heightmap.clone();
    for trigger in active {
        let Some(sw) = swaps.iter().find(|s| s.trigger == trigger) else { continue };
        let (dst_x, dst_y, src_x, src_y) = (sw.map[2], sw.map[3], sw.map[0], sw.map[1]);
        for layer in 0..2 {
            let before: Vec<Option<u32>> = (0..size[1]).flat_map(|y| (0..size[0]).map(move |x| (x, y))).map(|(x, y)| room.block(LAYERS[layer], x, y)).collect();
            for y in 0..size[1] {
                for x in 0..size[0] {
                    if !inside(sw, layer, x, y, hm_left, hm_top) {
                        continue;
                    }
                    let (sx, sy) = (src_x - dst_x + x, src_y - dst_y + y);
                    let v = if sx >= 0 && sx < size[0] && sy >= 0 && sy < size[1] { before[(sy * size[0] + sx) as usize] } else { None };
                    room.set_block(LAYERS[layer], x, y, v);
                }
            }
        }
        // the heightmap
        let before: HashMap<(i32, i32), ls_data::Cell> = (hm.origin[1]..hm.origin[1] + hm.size[1]).flat_map(|y| (hm.origin[0]..hm.origin[0] + hm.size[0]).map(move |x| (x, y))).filter_map(|(x, y)| room.cell(x, y).map(|c| ((x, y), c))).collect();
        for y in 0..sw.hm[5] {
            for x in 0..sw.hm[4] {
                let (sx, sy) = (x + sw.hm[0], y + sw.hm[1]);
                let (dx, dy) = (x + sw.hm[2], y + sw.hm[3]);
                if let Some(c) = before.get(&(sx, sy)) {
                    if before.contains_key(&(dx, dy)) {
                        room.set_cell(dx, dy, *c);
                    }
                }
            }
        }
    }
}
