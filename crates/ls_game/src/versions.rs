//! The versions of a room. The original has one room number per state of a place (Ver1 with the water, Ver2 without; the banquet, the empty castle...):
//! a door leads to the first of them, and the game goes to the replacing room once a story flag is set (data/room_versions.yaml: room, replaced by, flag).
//! The table is read in order and a replacement can be replaced again, like the original's.

use bevy::prelude::*;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Resource, Default)]
pub struct RoomVersions {
    /// (room, replaced by, flag)
    table: Vec<(i32, i32, i32)>,
}

/// A room asked for by hand (the panel, the O / P keys) is the one shown, not the version the flags would give.
static DIRECT: AtomicBool = AtomicBool::new(false);

pub fn ask_directly() {
    DIRECT.store(true, Ordering::Relaxed);
}

/// True once (and resets) when the room that is going to open was asked for by hand.
pub fn take_direct() -> bool {
    DIRECT.swap(false, Ordering::Relaxed)
}

impl RoomVersions {
    pub fn load(data: &Path) -> RoomVersions {
        let tree = crate::yamldata::read(&data.join("room_versions.yaml"));
        let table = tree
            .as_array()
            .map(|rows| rows.iter().filter_map(|r| Some((r.get("room")?.as_i64()? as i32, r.get("replaced_by")?.as_i64()? as i32, r.get("flag")?.as_i64()? as i32))).collect())
            .unwrap_or_default();
        RoomVersions { table }
    }

    pub fn from_table(table: Vec<(i32, i32, i32)>) -> RoomVersions {
        RoomVersions { table }
    }

    /// The room that opens when a door leads to `room`: the table in order, each replacement whose flag is set.
    pub fn redirect(&self, room: i32, flag: impl Fn(i32) -> bool) -> i32 {
        let mut room = room;
        for &(from, to, f) in &self.table {
            if from == room && flag(f) {
                room = to;
            }
        }
        room
    }

    /// The first version of a room (the one the doors lead to), or the room itself.
    pub fn original(&self, room: i32) -> i32 {
        let mut original = room;
        for &(from, to, _) in &self.table {
            if to == room {
                original = from;
            }
        }
        original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_flags_choose_the_version() {
        // 59 -> 60 (flag 145), 60 -> 61 (flag 191), 59 -> 61 (flag 191), 61 -> 62 (flag 261), 60 -> 62, 59 -> 62
        let v = RoomVersions::from_table(vec![(59, 60, 145), (60, 61, 191), (59, 61, 191), (61, 62, 261), (60, 62, 261), (59, 62, 261)]);
        assert_eq!(v.redirect(59, |_| false), 59);
        assert_eq!(v.redirect(59, |f| f == 145), 60);
        assert_eq!(v.redirect(59, |f| f == 145 || f == 191), 61);
        assert_eq!(v.redirect(59, |f| f == 261), 62);
        assert_eq!(v.redirect(7, |_| true), 7);
        assert_eq!(v.original(62), 59);
        assert_eq!(v.original(59), 59);
    }
}
