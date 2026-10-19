//! Sound: the songs and sound effects of the game. A song is a VGM file (data/audio/vgz/music_XX.vgz, tools/render_vgm.py: the
//! writes of the game's own sound driver to the YM2612 and the PSG, played here by the chips emulated in ls_data::vgm), or
//! an OGG rendered once from the engine (tools/render_sounds.py); the effects are FLAC files. A song is a custom audio source (`Pcm`) that knows where it loops, so the intro plays once and the loop never
//! has a gap. Volumes come from landstalker.ini ([audio]).

use bevy::audio::{ChannelCount, Decodable, SampleRate, Source, Volume};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

/// A sound: interleaved samples, optionally with a loop; or a VGM song, played by the emulated chips as it goes.
#[derive(Asset, TypePath)]
pub struct Pcm {
    samples: Arc<Vec<i16>>,
    channels: u16,
    rate: u32,
    /// loop start and end, in samples (not frames)
    looping: Option<(usize, usize)>,
    chip: Option<Arc<ls_data::vgm::Vgm>>,
}

pub struct PcmDecoder {
    samples: Arc<Vec<i16>>,
    pos: usize,
    channels: u16,
    rate: u32,
    looping: Option<(usize, usize)>,
    /// a song played by the chips: the stream, and the block of sound made last
    chip: Option<ls_data::vgm::VgmStream>,
    block: Vec<i16>,
}

impl Iterator for PcmDecoder {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if let Some(stream) = self.chip.as_mut() {
            if self.pos >= self.block.len() {
                // the chips run in blocks of 512 frames (a hundredth of a second)
                if stream.finished() {
                    return None;
                }
                self.block.clear();
                stream.fill(512, &mut self.block);
                self.pos = 0;
            }
            let v = self.block[self.pos] as f32 / 32768.0;
            self.pos += 1;
            return Some(v);
        }
        if let Some((start, end)) = self.looping {
            if self.pos >= end {
                self.pos = start;
            }
        }
        let v = self.samples.get(self.pos).map(|s| *s as f32 / 32768.0);
        self.pos += 1;
        v
    }
}

impl Source for PcmDecoder {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> ChannelCount {
        ChannelCount::new(self.channels).unwrap()
    }
    fn sample_rate(&self) -> SampleRate {
        SampleRate::new(self.rate).unwrap()
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Decodable for Pcm {
    type Decoder = PcmDecoder;
    fn decoder(&self) -> PcmDecoder {
        PcmDecoder { samples: self.samples.clone(), pos: 0, channels: self.channels, rate: self.rate, looping: self.looping, chip: self.chip.as_ref().map(|c| c.stream()), block: Vec::new() }
    }
}

#[derive(Deserialize, Default)]
struct MusicInfo {
    #[serde(default)]
    r#loop: bool,
    #[serde(default)]
    loop_start: usize,
    #[serde(default)]
    loop_end: usize,
}

/// Volumes and switches (landstalker.ini, [audio]).
#[derive(Resource, Clone)]
pub struct AudioSettings {
    pub enabled: bool,
    pub master: f32,
    pub music: f32,
    pub sfx: f32,
    /// the songs: "vgm" (the chips emulated), "ogg" (rendered once), "auto" (vgm when there is a file)
    pub music_format: String,
    /// the fanfare of a chest is played over the room's music, which goes on (false: it takes the music's place for a moment, like the original)
    pub chest_keeps_music: bool,
    /// the volume of some sound effects, by id (`[sfx]` of landstalker.ini: only the ones that are written; the others are at the normal volume)
    pub sfx_volumes: HashMap<u8, f32>,
}

/// The sound effects by name (`SND_SwordHit` is `sword_hit`: the names of landstalker.ini, [sfx], are read without the case, the underscores and the dashes).
pub const SFX_NAMES: [(u8, &str); 58] = [
    (0x41, "skeleton_talk"), (0x42, "cursor_move"), (0x43, "cursor_select"), (0x44, "error_buzz"), (0x45, "speaker_pitch_7"), (0x46, "speaker_pitch_6"),
    (0x47, "speaker_pitch_5"), (0x48, "speaker_pitch_4"), (0x49, "speaker_pitch_3"), (0x4A, "speaker_pitch_2"), (0x4B, "speaker_pitch_1"), (0x4C, "speaker_pitch_0"),
    (0x4D, "warp_pad"), (0x4E, "drum"), (0x4F, "cymbal"), (0x50, "snare_drum"), (0x51, "sword_swing"), (0x52, "sword_hit"), (0x53, "jump"), (0x54, "fall"),
    (0x55, "door"), (0x56, "throw"), (0x57, "enemy_noise"), (0x58, "door_lock"), (0x59, "rumble"), (0x5A, "health_recover_2"), (0x5B, "dog_transform"),
    (0x5C, "ghost_absorb_hp"), (0x5D, "jump_land"), (0x5E, "ladder_climb"), (0x5F, "throw_1"), (0x60, "rebound"), (0x61, "nigel_hit_1"), (0x62, "nigel_drop_obj_2"),
    (0x63, "nigel_drop_obj_1"), (0x64, "nigel_hit_2"), (0x65, "nigel_die"), (0x66, "enemy_die_1"), (0x67, "enemy_hit_1"), (0x68, "menu_open"),
    (0x69, "health_recover_1"), (0x6A, "pickup_money"), (0x6B, "kazalt_warp"), (0x6C, "switch"), (0x6D, "explosion"), (0x6E, "thud"), (0x6F, "enemy_die_2"),
    (0x70, "enemy_hit_2"), (0x71, "fireball_1"), (0x72, "fireball_2"), (0x73, "short_rumble"), (0x74, "permanent_switch"), (0x75, "latch"), (0x76, "friday_magic"),
    (0x77, "slash_1"), (0x78, "slash_2"), (0x79, "mir_warp"), (0x7A, "poison"),
];

/// The id of a sound effect from what is written in the ini: its name (`sword_hit`, `SwordHit`...) or its number (`52`, `0x52`).
pub fn sfx_id(key: &str) -> Option<u8> {
    let plain = |s: &str| s.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_lowercase();
    let k = plain(key);
    SFX_NAMES.iter().find(|(_, n)| plain(n) == k).map(|(id, _)| *id).or_else(|| u8::from_str_radix(key.trim().trim_start_matches("0x"), 16).ok())
}

impl Default for AudioSettings {
    fn default() -> Self {
        AudioSettings { enabled: true, master: 1.0, music: 0.6, sfx: 0.8, music_format: "auto".to_string(), chest_keeps_music: true, sfx_volumes: HashMap::new() }
    }
}

/// Sound effects to play at the next frame (filled by the game and by the scripts).
#[derive(Resource, Default)]
pub struct SfxQueue(pub Vec<u8>);

impl SfxQueue {
    pub fn play(&mut self, id: u8) {
        self.0.push(id);
    }
}

/// What plays now.
#[derive(Resource, Default)]
pub struct Sound {
    dir: PathBuf,
    cache: HashMap<String, Option<Handle<Pcm>>>,
    /// song id playing (None: silence)
    pub music: Option<u8>,
    music_entity: Option<Entity>,
    /// the song wanted by the room, played unless a jingle is on
    pub room_song: Option<u8>,
    /// seconds left of a jingle played over the room music
    pub jingle: f32,
    /// a jingle to start at the next frame: (song id, seconds)
    pending_jingle: Option<(u8, f32)>,
    /// a fanfare to play over the music at the next frame
    pending_overlay: Option<u8>,
    /// songs being decoded in a thread
    pending: HashMap<String, std::sync::Mutex<std::sync::mpsc::Receiver<Option<Decoded>>>>,
}

impl Sound {
    /// Plays a short song once over the room's music (fanfare, chest opening...); the room's song starts again after `seconds`.
    pub fn request_jingle(&mut self, id: u8, seconds: f32) {
        self.pending_jingle = Some((id, seconds));
    }

    /// Plays a short song over the room's music, which is not touched.
    pub fn request_over(&mut self, id: u8) {
        self.pending_overlay = Some(id);
    }
}

#[derive(Component)]
pub struct Music;

/// Index of the room's "bgm" in the game's table of songs (code/maps/mapload.asm RoomMusicLUT).
pub fn room_song(bgm: i32, evil_village_cured: bool) -> Option<u8> {
    const LUT: [u8; 19] = [0x25, 0x03, 0x13, 0x01, 0x02, 0x21, 0x23, 0x1D, 0x13, 0x26, 0x27, 0x0A, 0x14, 0x2B, 0x2A, 0x22, 0x15, 0x29, 0xFD];
    let song = *LUT.get(usize::try_from(bgm).ok()?)?;
    Some(if song == 0x23 && evil_village_cured { 0x24 } else { song })
}

/// A sound read from its file (done in a thread for the songs, which take a moment).
struct Decoded {
    samples: Vec<i16>,
    channels: u16,
    rate: u32,
    looping: Option<(usize, usize)>,
    chip: Option<Arc<ls_data::vgm::Vgm>>,
}

/// A song as a VGM file: the chips play it when it is heard (nothing to prepare: reading the file is all it takes).
fn decode_vgm(path: &std::path::Path) -> Option<Decoded> {
    let bytes = std::fs::read(path).ok()?;
    let song = ls_data::vgm::Vgm::parse(&bytes).map_err(|e| println!("sound: {}: {:#}", path.display(), e)).ok()?;
    let rate = ls_data::vgm::Synth::sample_rate(song.ym_clock);
    Some(Decoded { samples: Vec::new(), channels: 2, rate, looping: None, chip: Some(Arc::new(song)) })
}

/// Reads data/audio/vgz/<name>.vgz (songs), flac/<name>.flac (sound effects) or ogg/<name>.ogg (and ogg/<name>.yaml for the loop of an ogg). `format`: see AudioSettings.
fn decode_named(dir: &std::path::Path, name: &str, format: &str) -> Option<Decoded> {
    for ext in ["vgz", "vgm"] {
        let vgm = dir.join("vgz").join(format!("{}.{}", name, ext));
        if format != "ogg" && vgm.exists() {
            if let Some(d) = decode_vgm(&vgm) {
                return Some(d);
            }
        }
    }
    // a sound is a FLAC (sound effects) or an OGG (songs)
    let flac = dir.join("flac").join(format!("{}.flac", name));
    let file = if flac.exists() { flac } else { dir.join("ogg").join(format!("{}.ogg", name)) };
    let decoded = if file.extension().map_or(false, |e| e == "flac") { decode_flac(&file) } else { decode_ogg(&file) };
    let (samples, channels, rate) = decoded?;
    let looping = std::fs::read_to_string(dir.join("ogg").join(format!("{}.yaml", name)))
        .ok()
        .and_then(|t| serde_yaml::from_str::<MusicInfo>(&t).ok())
        .filter(|m| m.r#loop)
        .map(|m| (m.loop_start * channels as usize, m.loop_end.min(samples.len() / channels as usize) * channels as usize));
    Some(Decoded { samples, channels, rate, looping, chip: None })
}

fn decode_flac(path: &std::path::Path) -> Option<(Vec<i16>, u16, u32)> {
    let mut reader = claxon::FlacReader::open(path).ok()?;
    let info = reader.streaminfo();
    let bits = info.bits_per_sample as i32;
    let samples: Vec<i16> = reader.samples().filter_map(|s| s.ok()).map(|s| if bits >= 16 { (s >> (bits - 16)) as i16 } else { (s << (16 - bits)) as i16 }).collect();
    Some((samples, info.channels as u16, info.sample_rate))
}

fn decode_ogg(path: &std::path::Path) -> Option<(Vec<i16>, u16, u32)> {
    let file = std::fs::File::open(path).ok()?;
    let mut reader = lewton::inside_ogg::OggStreamReader::new(file).ok()?;
    let channels = reader.ident_hdr.audio_channels as u16;
    let rate = reader.ident_hdr.audio_sample_rate;
    let mut out = Vec::new();
    while let Ok(Some(packet)) = reader.read_dec_packet_itl() {
        out.extend_from_slice(&packet);
    }
    Some((out, channels, rate))
}

impl Sound {
    pub fn new(data_dir: &std::path::Path) -> Sound {
        Sound { dir: data_dir.join("audio"), ..default() }
    }

    fn finish(&mut self, name: &str, d: Option<Decoded>, pcms: &mut Assets<Pcm>) -> Option<Handle<Pcm>> {
        let handle = d.map(|d| pcms.add(Pcm { samples: Arc::new(d.samples), channels: d.channels, rate: d.rate, looping: d.looping, chip: d.chip }));
        if handle.is_none() {
            println!("sound: {} not found in {} (tools/render_sounds.py)", name, self.dir.display());
        }
        self.cache.insert(name.to_string(), handle.clone());
        handle
    }

    /// A short sound (an effect), read at once.
    fn load(&mut self, name: &str, pcms: &mut Assets<Pcm>) -> Option<Handle<Pcm>> {
        if let Some(h) = self.cache.get(name) {
            return h.clone();
        }
        let d = decode_named(&self.dir, name, "auto");
        self.finish(name, d, pcms)
    }

    /// A song: read in a thread (a song takes a tenth of a second or more to decode); None while it is not ready.
    fn load_in_background(&mut self, name: &str, format: &str, pcms: &mut Assets<Pcm>) -> Option<Handle<Pcm>> {
        if let Some(h) = self.cache.get(name) {
            return h.clone();
        }
        if let Some(rx) = self.pending.get(name) {
            let result = rx.lock().ok().map(|r| r.try_recv());
            return match result.unwrap_or(Err(std::sync::mpsc::TryRecvError::Disconnected)) {
                Ok(d) => {
                    self.pending.remove(name);
                    self.finish(name, d, pcms)
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => None,
                Err(_) => {
                    self.pending.remove(name);
                    self.finish(name, None, pcms)
                }
            };
        }
        let (tx, rx) = std::sync::mpsc::channel();
        let (dir, n, f) = (self.dir.clone(), name.to_string(), format.to_string());
        std::thread::spawn(move || {
            let _ = tx.send(decode_named(&dir, &n, &f));
        });
        self.pending.insert(name.to_string(), std::sync::Mutex::new(rx));
        None
    }
}

/// Plays the queued sound effects.
pub fn play_sfx(mut commands: Commands, mut queue: ResMut<SfxQueue>, mut sound: ResMut<Sound>, mut pcms: ResMut<Assets<Pcm>>, settings: Res<AudioSettings>) {
    let ids = std::mem::take(&mut queue.0);
    if !settings.enabled {
        return;
    }
    for id in ids {
        if std::env::var("LS_LOG_AUDIO").is_ok() {
            println!("[audio] sfx {:#04x}", id);
        }
        if let Some(h) = sound.load(&format!("sfx_{:02X}", id), &mut pcms) {
            commands.spawn((AudioPlayer(h), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.sfx * settings.master * settings.sfx_volumes.get(&id).copied().unwrap_or(1.0)))));
        }
    }
}

/// Starts the song the room wants (or a jingle) and stops the previous one.
pub fn play_music(mut commands: Commands, time: Res<Time>, mut sound: ResMut<Sound>, mut pcms: ResMut<Assets<Pcm>>, settings: Res<AudioSettings>, music: Query<Entity, With<Music>>) {
    if let Some(id) = sound.pending_overlay.take() {
        if settings.enabled {
            if let Some(h) = sound.load_in_background(&format!("music_{:02X}", id), &settings.music_format, &mut pcms) {
                commands.spawn((AudioPlayer(h), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.music * settings.master))));
            }
        }
    }
    if let Some((id, seconds)) = sound.pending_jingle.take() {
        jingle(&mut commands, &mut sound, &mut pcms, &settings, &music, id, seconds);
        return;
    }
    if sound.jingle > 0.0 {
        sound.jingle -= time.delta_secs();
        if sound.jingle <= 0.0 {
            sound.music = None; // the room's song starts again
        }
    }
    if sound.jingle > 0.0 {
        return;
    }
    let want = if settings.enabled { sound.room_song.filter(|s| *s != 0xFD) } else { None };
    if want == sound.music {
        return;
    }
    let Some(song) = want else {
        for e in &music {
            commands.entity(e).despawn();
        }
        sound.music_entity = None;
        sound.music = None;
        return;
    };
    // the song is read in a thread: the old one goes on until the new one is ready
    let Some(h) = sound.load_in_background(&format!("music_{:02X}", song), &settings.music_format, &mut pcms) else { return };
    for e in &music {
        commands.entity(e).despawn();
    }
    if std::env::var("LS_LOG_AUDIO").is_ok() {
        println!("[audio] music {:#04x}", song);
    }
    sound.music = Some(song);
    let e = commands.spawn((Music, AudioPlayer(h), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.music * settings.master)))).id();
    sound.music_entity = Some(e);
}


/// Plays a short song once over the room's music (fanfares, chest opening...) for `seconds`.
pub fn jingle(commands: &mut Commands, sound: &mut Sound, pcms: &mut Assets<Pcm>, settings: &AudioSettings, music: &Query<Entity, With<Music>>, id: u8, seconds: f32) {
    for e in music {
        commands.entity(e).despawn();
    }
    sound.music = None;
    sound.jingle = seconds;
    if settings.enabled {
        if let Some(h) = sound.load_in_background(&format!("music_{:02X}", id), &settings.music_format, pcms) {
            let e = commands.spawn((Music, AudioPlayer(h), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.music * settings.master)))).id();
            sound.music_entity = Some(e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ls_data::vgm::{write_vgm, Vgm, Write};

    #[test]
    fn a_vgm_song_is_played_by_the_chips_for_ever() {
        // an FM voice keyed on, a loop that starts after the first write
        let mut events = Vec::new();
        for (i, (a, d)) in [(0x30u8, 0x01u8), (0x40, 0x00), (0x50, 0x1f), (0x80, 0x0f), (0xb0, 0x07), (0xb4, 0xc0), (0xa4, 0x22), (0xa0, 0x69), (0x28, 0xf0)].iter().enumerate() {
            events.push((i as u32 * 10, Write::Ym(0, *a)));
            events.push((i as u32 * 10, Write::Ym(1, *d)));
        }
        let bytes = write_vgm(&events, 2000, Some(500));
        let song = Arc::new(Vgm::parse(&bytes).unwrap());
        let pcm = Pcm { samples: Arc::new(Vec::new()), channels: 2, rate: 53267, looping: None, chip: Some(song) };
        let mut dec = pcm.decoder();
        // 3 s of sound: well past the end of the 2000 samples of the song, which goes on with its loop
        let heard: Vec<f32> = (0..53267 * 2 * 3).map_while(|_| dec.next()).collect();
        assert_eq!(heard.len(), 53267 * 2 * 3);
        assert!(heard.iter().any(|v| v.abs() > 0.001));
        assert_eq!(dec.channels().get(), 2);
    }
}
