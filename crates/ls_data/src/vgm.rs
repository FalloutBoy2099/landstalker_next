//! VGM (Video Game Music) files: a log of the writes to the console's sound chips, YM2612 (FM) and SN76489 (PSG), with the time
//! between them. The music of the game is stored this way: the notes are not an approximation (MIDI, MOD), they are what the
//! original sound driver told the chips; playing a song is running the chips again (ym2612.rs, the PSG below) and mixing them
//! like the C++ engine does (SoundEngine::Render), so it sounds the same as the files rendered with that engine.
//!
//! Supported commands: 0x50 (PSG), 0x52 / 0x53 (YM2612 ports 0 / 1), 0x61 / 0x62 / 0x63 / 0x7n (waits), 0x66 (end), the
//! data blocks and PCM seeks of other chips are skipped. The loop point is the header's.

use crate::ym2612::Ym2612;
use anyhow::{bail, Result};
use std::collections::VecDeque;

/// A write to a chip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    /// YM2612 port (0 address, 1 data, 2 address of part 2, 3 data of part 2) and value
    Ym(u8, u8),
    Psg(u8),
}

// -------------------------------------------------------------------------------------------------------------------
// SN76489, as the C++ engine's (an approximation: square waves and a noise register, no analogue filtering)
// -------------------------------------------------------------------------------------------------------------------

const PSG_VOLUME: [f64; 16] = [1.0, 0.794, 0.631, 0.501, 0.398, 0.316, 0.251, 0.200, 0.158, 0.126, 0.100, 0.079, 0.063, 0.050, 0.040, 0.0];

#[derive(Clone)]
pub struct Psg {
    tone_reg: [u16; 3],
    tone_counter: [i32; 3],
    tone_out: [i32; 3],
    atten: [u8; 4],
    noise_reg: u8,
    noise_counter: i32,
    noise_out: i32,
    lfsr: u16,
    latched_channel: usize,
    latched_volume: bool,
}

impl Default for Psg {
    fn default() -> Self {
        Psg { tone_reg: [0; 3], tone_counter: [0; 3], tone_out: [1; 3], atten: [15; 4], noise_reg: 0, noise_counter: 0, noise_out: 1, lfsr: 0x8000, latched_channel: 0, latched_volume: false }
    }
}

impl Psg {
    pub fn write(&mut self, v: u8) {
        if v & 0x80 != 0 {
            self.latched_channel = ((v >> 5) & 3) as usize;
            self.latched_volume = v & 0x10 != 0;
            let data = v & 0x0f;
            if self.latched_volume {
                self.atten[self.latched_channel] = data;
            } else if self.latched_channel < 3 {
                self.tone_reg[self.latched_channel] = (self.tone_reg[self.latched_channel] & 0x3f0) | data as u16;
            } else {
                self.noise_reg = data & 7;
                self.lfsr = 0x8000;
            }
        } else if self.latched_volume {
            self.atten[self.latched_channel] = v & 0x0f;
        } else if self.latched_channel < 3 {
            self.tone_reg[self.latched_channel] = (self.tone_reg[self.latched_channel] & 0x00f) | (((v & 0x3f) as u16) << 4);
        } else {
            self.noise_reg = v & 7;
            self.lfsr = 0x8000;
        }
    }

    pub fn tick(&mut self) {
        for c in 0..3 {
            self.tone_counter[c] -= 1;
            if self.tone_counter[c] <= 0 {
                self.tone_counter[c] = (self.tone_reg[c] as i32).max(1);
                self.tone_out[c] ^= 1;
            }
        }
        self.noise_counter -= 1;
        if self.noise_counter <= 0 {
            const RATES: [i32; 3] = [0x10, 0x20, 0x40];
            let r = (self.noise_reg & 3) as usize;
            self.noise_counter = if r < 3 { RATES[r] } else { (self.tone_reg[2] as i32).max(1) };
            self.noise_out ^= 1;
            if self.noise_out != 0 {
                // the shift register advances on a rising edge
                let white = self.noise_reg & 4 != 0;
                let fb = if white { ((self.lfsr & 1) ^ ((self.lfsr >> 3) & 1)) & 1 } else { self.lfsr & 1 };
                self.lfsr = (self.lfsr >> 1) | (fb << 15);
            }
        }
    }

    pub fn sample(&self) -> i32 {
        let mut sum = 0.0;
        for c in 0..3 {
            sum += (if self.tone_out[c] != 0 { 1.0 } else { -1.0 }) * PSG_VOLUME[self.atten[c] as usize];
        }
        sum += (if self.lfsr & 1 != 0 { 1.0 } else { -1.0 }) * PSG_VOLUME[self.atten[3] as usize];
        (sum * 1300.0) as i32
    }
}

// -------------------------------------------------------------------------------------------------------------------
// the two chips and the mixer (SoundEngine::Render)
// -------------------------------------------------------------------------------------------------------------------

/// The master clock of the console the music was made on: the YM2612 runs at 7670454 Hz, a sample takes 144 of those clocks.
pub const YM_CLOCK: u32 = 7_670_454;
pub const SN_CLOCK: u32 = 3_579_545;
const FM_GAIN: i32 = 15;
/// the PSG ticks every 16 Z80 cycles (3.58 MHz): PSG clocks per output sample
const PSG_CLOCKS_PER_SAMPLE: f64 = (3_579_545.0 / 16.0) / (YM_CLOCK as f64 / 144.0);

/// Both chips and the mixing: stereo 16-bit at `YM_CLOCK / 144` Hz (53267).
pub struct Synth {
    ym: Ym2612,
    psg: Psg,
    /// OPN clocks (6 master clocks each) run so far
    pub clock_time: u64,
    psg_acc: f64,
    dc_prev_in: [f64; 2],
    dc_prev_out: [f64; 2],
    pub volume: f64,
}

impl Default for Synth {
    fn default() -> Self {
        Synth { ym: Ym2612::new(), psg: Psg::default(), clock_time: 0, psg_acc: 0.0, dc_prev_in: [0.0; 2], dc_prev_out: [0.0; 2], volume: 1.0 }
    }
}

impl Synth {
    pub fn sample_rate(ym_clock: u32) -> u32 {
        (ym_clock as f64 / 144.0).round() as u32
    }

    fn apply(&mut self, w: Write) {
        match w {
            Write::Ym(port, v) => self.ym.write(port as u32, v),
            Write::Psg(v) => self.psg.write(v),
        }
    }

    /// Renders `frames` stereo frames into `out` (interleaved). `schedule` holds the writes with the OPN clock they happen at
    /// (sorted); the ones due are given to the chips just before that clock runs.
    pub fn render(&mut self, frames: usize, schedule: &mut VecDeque<(u64, Write)>, out: &mut Vec<i16>) {
        out.reserve(frames * 2);
        for _ in 0..frames {
            let (mut fm_l, mut fm_r) = (0i32, 0i32);
            for _ in 0..24 {
                self.clock_time += 1;
                while let Some(&(t, w)) = schedule.front() {
                    if t > self.clock_time {
                        break;
                    }
                    schedule.pop_front();
                    self.apply(w);
                }
                let (l, r) = self.ym.clock();
                fm_l += l as i32;
                fm_r += r as i32;
            }
            self.psg_acc += PSG_CLOCKS_PER_SAMPLE;
            while self.psg_acc >= 1.0 {
                self.psg.tick();
                self.psg_acc -= 1.0;
            }
            let p = self.psg.sample();
            // the YM2612's DAC rests off zero: a one-pole high-pass (about 4 Hz) takes the offset out
            let input = [(fm_l * FM_GAIN + p) as f64, (fm_r * FM_GAIN + p) as f64];
            for c in 0..2 {
                let y = input[c] - self.dc_prev_in[c] + 0.9995 * self.dc_prev_out[c];
                self.dc_prev_in[c] = input[c];
                self.dc_prev_out[c] = y;
                out.push(((y * self.volume) as i32).clamp(-32768, 32767) as i16);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------------------------
// the file
// -------------------------------------------------------------------------------------------------------------------

/// A song: its writes with the time (in 44100 Hz samples) they happen at.
#[derive(Clone, Debug)]
pub struct Vgm {
    pub ym_clock: u32,
    pub psg_clock: u32,
    pub events: Vec<(u32, Write)>,
    /// length of the song in samples (44100 Hz), and where the loop starts (None: it plays once)
    pub total_samples: u32,
    pub loop_sample: Option<u32>,
    /// index in `events` of the first event of the loop
    pub loop_event: usize,
}

/// A song rendered to sound (see `Vgm::render`).
pub struct Rendered {
    /// interleaved stereo
    pub pcm: Vec<i16>,
    pub sample_rate: u32,
}

fn u32_at(d: &[u8], at: usize) -> u32 {
    d.get(at..at + 4).map_or(0, |b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

impl Vgm {
    /// Reads a .vgm, or a .vgz (the same, compressed with gzip).
    pub fn parse(d: &[u8]) -> Result<Vgm> {
        if d.starts_with(&[0x1f, 0x8b]) {
            let mut raw = Vec::new();
            std::io::Read::read_to_end(&mut flate2::read::GzDecoder::new(d), &mut raw)?;
            return Vgm::parse(&raw);
        }
        if d.len() < 0x40 || &d[0..4] != b"Vgm " {
            bail!("not a VGM file (bad magic)");
        }
        let version = u32_at(d, 0x08);
        let psg_clock = u32_at(d, 0x0C) & 0x3fff_ffff;
        let ym_clock = if version >= 0x110 { u32_at(d, 0x2C) & 0x3fff_ffff } else { 0 };
        let ym_clock = if ym_clock == 0 { YM_CLOCK } else { ym_clock };
        let data_start = if version >= 0x150 && u32_at(d, 0x34) != 0 { 0x34 + u32_at(d, 0x34) as usize } else { 0x40 };
        let loop_pos = match u32_at(d, 0x1C) {
            0 => None,
            o => Some(0x1C + o as usize),
        };
        let mut events = Vec::new();
        let mut t: u32 = 0;
        let mut loop_sample = None;
        let mut loop_event = 0;
        let mut at = data_start;
        while at < d.len() {
            if loop_pos == Some(at) {
                loop_sample = Some(t);
                loop_event = events.len();
            }
            let cmd = d[at];
            let arg = |i: usize| d.get(at + i).copied().unwrap_or(0);
            match cmd {
                0x50 => {
                    events.push((t, Write::Psg(arg(1))));
                    at += 2;
                }
                0x52 | 0x53 => {
                    let base = if cmd == 0x52 { 0 } else { 2 };
                    events.push((t, Write::Ym(base, arg(1))));
                    events.push((t, Write::Ym(base + 1, arg(2))));
                    at += 3;
                }
                0x61 => {
                    t += arg(1) as u32 | (arg(2) as u32) << 8;
                    at += 3;
                }
                0x62 => {
                    t += 735;
                    at += 1;
                }
                0x63 => {
                    t += 882;
                    at += 1;
                }
                0x66 => break,
                0x70..=0x7f => {
                    t += (cmd & 0x0f) as u32 + 1;
                    at += 1;
                }
                0x80..=0x8f => {
                    // YM2612 DAC write (reg 0x2A) from a data bank, then a wait: banks are not supported, the wait is kept
                    t += (cmd & 0x0f) as u32;
                    at += 1;
                }
                0x67 => {
                    // data block: 0x67 0x66 type size(4) data
                    at += 7 + u32_at(d, at + 3) as usize;
                }
                0x30..=0x3f | 0x4f | 0x50..=0x5f => at += 2,
                0x40..=0x4e | 0xa0..=0xbf => at += 3,
                0xc0..=0xdf => at += 4,
                0xe0..=0xff => at += 5,
                _ => at += 1,
            }
        }
        Ok(Vgm { ym_clock, psg_clock, events, total_samples: t, loop_sample, loop_event })
    }

    /// The song as it is played: the chips run, block by block, as long as the game wants sound (a song with a loop never
    /// ends). See `VgmStream`.
    pub fn stream(self: &std::sync::Arc<Self>) -> VgmStream {
        VgmStream::new(self.clone())
    }

    /// The intro and two turns of the loop (the others, the song and two seconds of silence), rendered at once: for tools and
    /// tests.
    pub fn render(self: &std::sync::Arc<Self>) -> Rendered {
        let mut st = self.stream();
        let rate = st.sample_rate();
        let samples_to_frames = |s: u64| s * rate as u64 / 44100;
        let frames = match self.loop_sample {
            Some(l) => samples_to_frames(l as u64 + 2 * (self.total_samples - l) as u64),
            None => samples_to_frames(self.total_samples as u64) + 2 * rate as u64,
        } as usize;
        let mut pcm = Vec::new();
        st.fill(frames, &mut pcm);
        Rendered { pcm, sample_rate: rate }
    }
}

/// A song being played: the chips and the writes still to come. A loop is played again and again by queueing its writes once
/// more when the one before is almost over, so the chips go on without a break (the sound of the end of a turn dies away over
/// the start of the next, as on the console).
pub struct VgmStream {
    song: std::sync::Arc<Vgm>,
    synth: Synth,
    queue: VecDeque<(u64, Write)>,
    /// the first clock a new write can be given to the chip (a write takes a few clocks)
    free: u64,
    /// the turns of the loop queued so far after the first pass through the song; the clock the last queued pass ends at
    turns: u64,
    queued_until: u64,
}

/// The address and the data of a write are given to the chip a few clocks apart, and the writes one after the other.
const WRITE_GAP: u64 = 4;
/// A song that does not loop ends this long after its last command.
const TAIL_SECONDS: u64 = 2;

impl VgmStream {
    fn new(song: std::sync::Arc<Vgm>) -> VgmStream {
        let mut st = VgmStream { song, synth: Synth::default(), queue: VecDeque::new(), free: 1, turns: 0, queued_until: 0 };
        let first: Vec<(u32, Write)> = st.song.events.clone();
        for (t, w) in first {
            st.push(t as u64, w);
        }
        st.queued_until = st.clock_of(st.song.total_samples as u64);
        st
    }

    pub fn sample_rate(&self) -> u32 {
        Synth::sample_rate(self.song.ym_clock)
    }

    /// OPN clocks (6 master clocks each) at a time in samples of 44100 Hz.
    fn clock_of(&self, sample: u64) -> u64 {
        sample * self.song.ym_clock as u64 / (6 * 44100)
    }

    fn push(&mut self, sample: u64, w: Write) {
        let at = self.clock_of(sample).max(self.free).max(1);
        self.queue.push_back((at, w));
        self.free = at + WRITE_GAP;
    }

    /// Queues the next turn of the loop if the chips are getting close to the end of the writes queued.
    fn refill(&mut self, upto: u64) {
        let Some(l) = self.song.loop_sample else { return };
        let len = (self.song.total_samples - l) as u64;
        if len == 0 {
            return;
        }
        while self.queued_until < upto {
            self.turns += 1;
            let song = self.song.clone();
            for &(t, w) in &song.events[song.loop_event..] {
                self.push(t as u64 + self.turns * len, w);
            }
            self.queued_until = self.clock_of(l as u64 + (self.turns + 1) * len);
        }
    }

    /// True when a song that does not loop has played to its end.
    pub fn finished(&self) -> bool {
        self.song.loop_sample.is_none() && self.synth.clock_time > self.clock_of(self.song.total_samples as u64) + TAIL_SECONDS * (self.song.ym_clock as u64 / 6)
    }

    /// Appends `frames` stereo frames (interleaved).
    pub fn fill(&mut self, frames: usize, out: &mut Vec<i16>) {
        self.refill(self.synth.clock_time + frames as u64 * 24 + 1);
        self.synth.render(frames, &mut self.queue, out);
    }
}

/// Renders writes given with their exact OPN clock (the C++ engine's dumps, `lsctl evt-render`): `frames` stereo frames.
pub fn render_events(events: &[(u64, Write)], frames: usize) -> Vec<i16> {
    let mut q: VecDeque<(u64, Write)> = events.iter().copied().collect();
    let mut synth = Synth::default();
    let mut pcm = Vec::new();
    synth.render(frames, &mut q, &mut pcm);
    pcm
}

/// Writes a VGM file from writes with their time in 44100 Hz samples (`loop_sample`: the sample the loop starts at, which must
/// be the time of an event or the end).
pub fn write_vgm(events: &[(u32, Write)], total_samples: u32, loop_sample: Option<u32>) -> Vec<u8> {
    let mut body: Vec<u8> = Vec::new();
    let mut t: u32 = 0;
    let mut loop_offset: Option<usize> = None;
    let wait = |body: &mut Vec<u8>, mut n: u32| {
        while n > 0 {
            let w = n.min(0xffff);
            body.extend_from_slice(&[0x61, (w & 0xff) as u8, (w >> 8) as u8]);
            n -= w;
        }
    };
    let mut i = 0;
    while i < events.len() {
        let (et, w) = events[i];
        if loop_offset.is_none() && loop_sample.map_or(false, |l| et >= l) {
            let l = loop_sample.unwrap();
            if l > t {
                wait(&mut body, l - t);
                t = l;
            }
            loop_offset = Some(body.len());
        }
        if et > t {
            wait(&mut body, et - t);
            t = et;
        }
        match w {
            Write::Psg(v) => body.extend_from_slice(&[0x50, v]),
            Write::Ym(p, a) => {
                // an address and its data go in one command
                if p & 1 == 0 {
                    if let Some(&(_, Write::Ym(p2, d))) = events.get(i + 1) {
                        if p2 == p + 1 {
                            body.extend_from_slice(&[if p == 0 { 0x52 } else { 0x53 }, a, d]);
                            i += 2;
                            continue;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if loop_offset.is_none() && loop_sample.is_some() {
        wait(&mut body, loop_sample.unwrap().saturating_sub(t));
        t = t.max(loop_sample.unwrap());
        loop_offset = Some(body.len());
    }
    if total_samples > t {
        wait(&mut body, total_samples - t);
    }
    body.push(0x66);
    let mut head = vec![0u8; 0x80];
    head[0..4].copy_from_slice(b"Vgm ");
    let eof = (0x80 + body.len() - 4) as u32;
    head[4..8].copy_from_slice(&eof.to_le_bytes());
    head[8..12].copy_from_slice(&0x150u32.to_le_bytes());
    head[0x0C..0x10].copy_from_slice(&SN_CLOCK.to_le_bytes());
    head[0x18..0x1C].copy_from_slice(&total_samples.to_le_bytes());
    if let (Some(off), Some(l)) = (loop_offset, loop_sample) {
        head[0x1C..0x20].copy_from_slice(&((0x80 + off - 0x1C) as u32).to_le_bytes());
        head[0x20..0x24].copy_from_slice(&(total_samples - l).to_le_bytes());
    }
    head[0x24..0x28].copy_from_slice(&60u32.to_le_bytes());
    head[0x28..0x2A].copy_from_slice(&9u16.to_le_bytes()); // SN76489 feedback
    head[0x2A] = 16; // shift register width
    head[0x2C..0x30].copy_from_slice(&YM_CLOCK.to_le_bytes());
    head[0x34..0x38].copy_from_slice(&(0x80u32 - 0x34).to_le_bytes());
    head.extend_from_slice(&body);
    head
}

/// A file compressed with gzip (.vgz).
pub fn gzip(bytes: &[u8]) -> Vec<u8> {
    let mut enc = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
    std::io::Write::write_all(&mut enc, bytes).expect("in memory");
    enc.finish().expect("in memory")
}

/// The writes of the song without the samples sent to the DAC (register 0x2A and its data), which come at a rate of the driver's
/// own: in a song that plays samples their number and place change a little from one turn of the loop to the next.
pub fn without_dac(events: &[(u64, Write)]) -> Vec<(u64, Write)> {
    let mut out = Vec::with_capacity(events.len());
    let mut i = 0;
    while i < events.len() {
        if events[i].1 == Write::Ym(0, 0x2a) {
            if let Some((_, Write::Ym(1, _))) = events.get(i + 1) {
                i += 2;
                continue;
            }
        }
        out.push(events[i]);
        i += 1;
    }
    out
}

/// The writes of a dump of the C++ engine (SoundEngine::DumpEvents): "LSEV", u32 count, then per event u64 time in OPN clocks,
/// u8 kind (0..3 YM2612 port, 4 PSG), u8 value.
pub fn parse_events(d: &[u8]) -> Result<Vec<(u64, Write)>> {
    if d.len() < 8 || &d[0..4] != b"LSEV" {
        bail!("not an event dump (LSEV)");
    }
    let n = u32_at(d, 4) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let at = 8 + i * 10;
        let Some(e) = d.get(at..at + 10) else { bail!("event dump cut short") };
        let t = u64::from_le_bytes(e[0..8].try_into().unwrap());
        out.push((t, if e[8] == 4 { Write::Psg(e[9]) } else { Write::Ym(e[8], e[9]) }));
    }
    Ok(out)
}

/// Where a song that was recorded for longer than its loop starts to repeat itself: (index of the first event of the loop,
/// events in one turn of the loop). The driver is deterministic, so a loop plays the very same writes at the very same
/// distances: the loop is the (start, period) whose repetition reaches back the furthest and holds at least two turns.
/// Does this stretch of a song play a note: a key on of the FM chip, or a PSG channel that is not muted? (What a driver writes over
/// and over once its song is over does not: that is not a loop of the music.)
fn plays_notes(events: &[(u64, Write)]) -> bool {
    events.windows(2).any(|w| match (w[0].1, w[1].1) {
        (Write::Ym(0, 0x28), Write::Ym(1, v)) => v >> 4 != 0,
        (Write::Psg(v), _) => v & 0x90 == 0x90 && v & 0x0f != 0x0f,
        _ => false,
    })
}

/// Where a song that does not loop is over: seconds after its last note (a key on, a PSG channel not muted, or a sample sent to the
/// DAC that is not silence), so the notes can fade. (A driver goes on writing to the chips after its song, and for a long time.)
pub fn song_end(events: &[(u64, Write)]) -> u64 {
    const FADE_SECONDS: u64 = 6;
    let mut last = 0;
    for (i, &(t, w)) in events.iter().enumerate() {
        let sound = match (w, events.get(i + 1).map(|e| e.1)) {
            (Write::Ym(0, 0x28), Some(Write::Ym(1, v))) => v >> 4 != 0,
            (Write::Ym(0, 0x2a), Some(Write::Ym(1, v))) => v != 0x80,
            (Write::Psg(v), _) => v & 0x90 == 0x90 && v & 0x0f != 0x0f,
            _ => false,
        };
        if sound {
            last = t;
        }
    }
    last + FADE_SECONDS * (YM_CLOCK as u64 / 6)
}

/// Clocks of difference allowed between the distances of two writes of two turns of a loop (6 clocks of the Z80's 3.58 MHz
/// make about 2: the sound driver's timing varies a little from one turn to the next).
pub const JITTER: u64 = 128;

/// The same, for the songs that play samples (DAC): the driver's loop that feeds the DAC makes the other writes later or sooner.
pub const JITTER_WITH_SAMPLES: u64 = 2048;

pub fn detect_loop(events: &[(u64, Write)]) -> Option<(usize, usize)> {
    detect_loop_with(events, JITTER)
}

pub fn detect_loop_with(events: &[(u64, Write)], jitter: u64) -> Option<(usize, usize)> {
    detect_repeat(events, MIN_LOOP_CLOCKS, jitter, true)
}

/// A loop lasts at least 3 seconds (OPN clocks: the YM2612's master clock / 6).
const MIN_LOOP_CLOCKS: u64 = 3 * (YM_CLOCK as u64 / 6);

/// The same search, for any period of at least `min_clocks`: with 0, it also finds the idle pattern a driver writes over and over
/// once a jingle is over.
pub fn detect_repeat(events: &[(u64, Write)], min_clocks: u64, jitter: u64, music: bool) -> Option<(usize, usize)> {
    let n = events.len();
    if n < 64 {
        return None;
    }
    // each event as (clocks since the one before, what it writes)
    let tok: Vec<(u64, Write)> = (0..n).map(|i| (if i == 0 { 0 } else { events[i].0 - events[i - 1].0 }, events[i].1)).collect();
    let mut best: Option<(usize, usize)> = None;
    for p in 32..=n / 2 {
        // the run of events that equal the one a period later, counted from the end
        let mut k = n - p;
        // (the Z80's timing differs by a few clocks from one turn to the next: the writes must be the same, their distances
        // the same to within JITTER)
        while k > 0 && tok[k - 1].1 == tok[k - 1 + p].1 && tok[k - 1].0.abs_diff(tok[k - 1 + p].0) <= jitter {
            k -= 1;
        }
        // k is the first event of the run: two turns must fit (the first turn from k, the second ends at the end)
        if k + 2 * p < n && events[k + p].0 - events[k].0 >= min_clocks && best.map_or(true, |(bk, _)| k < bk) && (!music || plays_notes(&events[k..k + p])) {
            best = Some((k, p));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_written_file_reads_back() {
        let events = vec![(0, Write::Ym(0, 0x22)), (0, Write::Ym(1, 0x08)), (100, Write::Psg(0x9f)), (200, Write::Ym(2, 0xb4)), (200, Write::Ym(3, 0xc0))];
        let bytes = write_vgm(&events, 1000, Some(100));
        let v = Vgm::parse(&gzip(&bytes)).unwrap();
        assert_eq!(v.events.len(), 5);
        assert_eq!(v.events[2], (100, Write::Psg(0x9f)));
        assert_eq!(v.total_samples, 1000);
        assert_eq!(v.loop_sample, Some(100));
        assert_eq!(v.loop_event, 2);
        // the loop goes on for ever: three turns later there is still sound being asked for
        let mut st = std::sync::Arc::new(v).stream();
        let mut out = Vec::new();
        st.fill(3000, &mut out);
        assert_eq!(out.len(), 6000);
        assert!(!st.finished());
    }

    #[test]
    fn a_key_on_makes_sound_and_silence_stays_silent() {
        // channel 1: algorithm 7 (the four operators are carriers), full level, instant attack, key on
        let mut ev: Vec<(u64, Write)> = Vec::new();
        let mut t = 10;
        let mut regs: Vec<(u8, u8)> = Vec::new();
        for op in [0x00u8, 0x04, 0x08, 0x0c] {
            regs.extend([(0x30 + op, 0x01), (0x40 + op, 0x00), (0x50 + op, 0x1f), (0x60 + op, 0x00), (0x70 + op, 0x00), (0x80 + op, 0x0f)]);
        }
        regs.extend([(0xb0, 0x07), (0xb4, 0xc0), (0xa4, 0x22), (0xa0, 0x69), (0x28, 0xf0)]);
        for (a, d) in regs {
            ev.push((t, Write::Ym(0, a)));
            ev.push((t + 4, Write::Ym(1, d)));
            t += 8;
        }
        // (the chip was checked sample for sample against the C++ engine on a whole song, see docs/vgm.md; here: a key on is heard)
        let loud = render_events(&ev, 4000);
        let quiet = render_events(&[], 4000);
        let diff: i64 = loud.iter().zip(&quiet).map(|(a, b)| (*a as i64 - *b as i64).abs()).sum();
        assert!(diff > 100_000, "{}", diff);
        // the same writes give the same sound
        assert_eq!(loud, render_events(&ev, 4000));
    }
}
