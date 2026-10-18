//! Cutscenes recorded from the original: the RAM of the ROM running in EmuHawk (tools/emuhawk, make_track.py) gives, frame after frame
//! (60 Hz), where the player and every entity of the room are, the fades and the captions. The port plays that instead of guessing
//! what the original's scripts did. Format of a `.track` file: see tools/emuhawk/make_track.py.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sample {
    pub f: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    /// 0 NE, 1 SE, 2 SW, 3 NW
    pub rot: u8,
    pub hidden: bool,
}

#[derive(Default, Debug)]
pub struct Track {
    /// (frame, room): the room that is loaded while the screen is black
    pub rooms: Vec<(f32, i32)>,
    /// (frame, level 0..1): how black the screen is
    pub fades: Vec<(f32, f32)>,
    /// (frame, caption id (0 none), frames it lasts)
    pub strs: Vec<(f32, i32, f32)>,
    /// (frame, Friday's place on the screen in pixels of the 320x224 picture, None when she is not there)
    pub friday: Vec<(f32, Option<(f32, f32)>)>,
    /// (frame, action): a story action of cutscenes.json that starts (the conversations); the recording waits while it runs
    pub actions: Vec<(f32, i32)>,
    /// (frame, text of the message box, empty to close it)
    pub msgs: Vec<(f32, String)>,
    /// per slot (0 the player, n entity n - 1 of the room): the samples, in the order of the frames
    pub slots: Vec<Vec<Sample>>,
    pub length: f32,
    /// (frame, room): the recording ends here and the game's scripts take over in this room
    pub handover: Option<(f32, i32)>,
}

impl Track {
    pub fn parse(text: &str) -> Track {
        let mut t = Track::default();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (kind, rest) = line.split_once(' ').unwrap_or((line, ""));
            let num = |s: &str| s.parse::<f32>().unwrap_or(0.0);
            let f: Vec<&str> = rest.split_whitespace().collect();
            match kind {
                "room" if f.len() >= 2 => t.rooms.push((num(f[0]), num(f[1]) as i32)),
                "fade" if f.len() >= 2 => t.fades.push((num(f[0]), num(f[1]) / 1000.0)),
                "str" if f.len() >= 3 => t.strs.push((num(f[0]), num(f[1]) as i32, num(f[2]))),
                "msg" if !f.is_empty() => {
                    let (frame, text) = rest.split_once(' ').unwrap_or((rest, ""));
                    t.msgs.push((num(frame), text.replace('|', "\n")));
                }
                "fr" if f.len() >= 3 => t.friday.push((num(f[0]), if num(f[1]) < 0.0 { None } else { Some((num(f[1]), num(f[2]))) })),
                "action" if f.len() >= 2 => t.actions.push((num(f[0]), num(f[1]) as i32)),
                "handover" if f.len() >= 2 => t.handover = Some((num(f[0]), num(f[1]) as i32)),
                "e" if f.len() >= 7 => {
                    let slot = num(f[1]) as usize;
                    while t.slots.len() <= slot {
                        t.slots.push(Vec::new());
                    }
                    t.slots[slot].push(Sample { f: num(f[0]), x: num(f[2]) / 16.0, y: num(f[3]) / 16.0, z: num(f[4]) / 16.0, rot: num(f[5]) as u8 & 3, hidden: num(f[6]) as i32 & 1 != 0 });
                }
                _ => {}
            }
            if let Some(Ok(frame)) = f.first().map(|s| s.parse::<f32>().ok().ok_or(())) {
                t.length = t.length.max(frame);
            }
        }
        t
    }

    /// Where a slot is at a frame: the last sample, moved towards the next one when they are a few frames apart and the slot moves
    /// steadily (the recording keeps one frame out of two).
    pub fn at(&self, slot: usize, frame: f32) -> Option<Sample> {
        let list = self.slots.get(slot)?;
        let i = list.partition_point(|s| s.f <= frame);
        if i == 0 {
            return None;
        }
        let a = list[i - 1];
        if let Some(b) = list.get(i) {
            if b.f - a.f <= 3.0 && b.rot == a.rot && b.hidden == a.hidden && (b.x - a.x).abs() < 1.0 && (b.y - a.y).abs() < 1.0 {
                let k = (frame - a.f) / (b.f - a.f);
                return Some(Sample { f: frame, x: a.x + (b.x - a.x) * k, y: a.y + (b.y - a.y) * k, z: a.z + (b.z - a.z) * k, ..a });
            }
        }
        Some(a)
    }

    /// The last value of a list of (frame, v) at a frame.
    fn last<T: Copy>(list: &[(f32, T)], frame: f32) -> Option<T> {
        let i = list.partition_point(|s| s.0 <= frame);
        if i == 0 { None } else { Some(list[i - 1].1) }
    }

    pub fn friday_at(&self, frame: f32) -> Option<(f32, f32)> {
        Self::last(&self.friday, frame).flatten()
    }

    pub fn fade_at(&self, frame: f32) -> f32 {
        Self::last(&self.fades, frame).unwrap_or(0.0)
    }

    pub fn room_at(&self, frame: f32) -> Option<i32> {
        Self::last(&self.rooms, frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "# test\nroom 0 139\nfade 0 1000\nfade 20 0\nstr 54 1 120\nstr 400 0 0\ne 0 0 1000 392 144 0 0\ne 2 0 996 392 144 3 0\ne 4 0 992 392 144 3 0\ne 0 1 992 388 240 0 1\nroom 864 140\n";

    #[test]
    fn a_track_reads_and_is_played_back() {
        let t = Track::parse(TEXT);
        assert_eq!(t.room_at(10.0), Some(139));
        assert_eq!(t.room_at(900.0), Some(140));
        assert_eq!(t.fade_at(5.0), 1.0);
        assert_eq!(t.fade_at(30.0), 0.0);
        // in between two samples a steady move is smoothed (one frame out of two is recorded)
        let a = t.at(0, 3.0).unwrap();
        assert!((a.x - 62.125).abs() < 0.01, "{:?}", a);
        assert_eq!(t.at(0, 0.0).unwrap().rot, 0);
        assert_eq!(t.at(0, 5.0).unwrap().rot, 3);
        assert!(t.at(1, 0.0).unwrap().hidden);
        assert!(t.at(5, 0.0).is_none());
    }
}
