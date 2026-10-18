//! YM2612 (the Mega Drive's FM chip), a Rust port of Nuked-OPN2 by Nuke.YKT (ym3438.c 1.0.12, LGPL 2.1 or later), in its YM2612
//! mode. It is the emulator the C++ engine renders the game's music with, so the VGM files play back the same sound.
//! The chip is clocked one "OPN clock" at a time (6 master clocks): 24 of them make a sample (53267 Hz at the console's clock).
//! The code follows the C source line by line (the register and variable names are the original's); C's integer widths are kept.

const LOGSINROM: [u16; 256] = [0x859, 0x6c3, 0x607, 0x58b, 0x52e, 0x4e4, 0x4a6, 0x471, 0x443, 0x41a, 0x3f5, 0x3d3, 0x3b5, 0x398, 0x37e, 0x365, 0x34e, 0x339, 0x324, 0x311, 0x2ff, 0x2ed, 0x2dc, 0x2cd, 0x2bd, 0x2af, 0x2a0, 0x293, 0x286, 0x279, 0x26d, 0x261, 0x256, 0x24b, 0x240, 0x236, 0x22c, 0x222, 0x218, 0x20f, 0x206, 0x1fd, 0x1f5, 0x1ec, 0x1e4, 0x1dc, 0x1d4, 0x1cd, 0x1c5, 0x1be, 0x1b7, 0x1b0, 0x1a9, 0x1a2, 0x19b, 0x195, 0x18f, 0x188, 0x182, 0x17c, 0x177, 0x171, 0x16b, 0x166, 0x160, 0x15b, 0x155, 0x150, 0x14b, 0x146, 0x141, 0x13c, 0x137, 0x133, 0x12e, 0x129, 0x125, 0x121, 0x11c, 0x118, 0x114, 0x10f, 0x10b, 0x107, 0x103, 0x0ff, 0x0fb, 0x0f8, 0x0f4, 0x0f0, 0x0ec, 0x0e9, 0x0e5, 0x0e2, 0x0de, 0x0db, 0x0d7, 0x0d4, 0x0d1, 0x0cd, 0x0ca, 0x0c7, 0x0c4, 0x0c1, 0x0be, 0x0bb, 0x0b8, 0x0b5, 0x0b2, 0x0af, 0x0ac, 0x0a9, 0x0a7, 0x0a4, 0x0a1, 0x09f, 0x09c, 0x099, 0x097, 0x094, 0x092, 0x08f, 0x08d, 0x08a, 0x088, 0x086, 0x083, 0x081, 0x07f, 0x07d, 0x07a, 0x078, 0x076, 0x074, 0x072, 0x070, 0x06e, 0x06c, 0x06a, 0x068, 0x066, 0x064, 0x062, 0x060, 0x05e, 0x05c, 0x05b, 0x059, 0x057, 0x055, 0x053, 0x052, 0x050, 0x04e, 0x04d, 0x04b, 0x04a, 0x048, 0x046, 0x045, 0x043, 0x042, 0x040, 0x03f, 0x03e, 0x03c, 0x03b, 0x039, 0x038, 0x037, 0x035, 0x034, 0x033, 0x031, 0x030, 0x02f, 0x02e, 0x02d, 0x02b, 0x02a, 0x029, 0x028, 0x027, 0x026, 0x025, 0x024, 0x023, 0x022, 0x021, 0x020, 0x01f, 0x01e, 0x01d, 0x01c, 0x01b, 0x01a, 0x019, 0x018, 0x017, 0x017, 0x016, 0x015, 0x014, 0x014, 0x013, 0x012, 0x011, 0x011, 0x010, 0x00f, 0x00f, 0x00e, 0x00d, 0x00d, 0x00c, 0x00c, 0x00b, 0x00a, 0x00a, 0x009, 0x009, 0x008, 0x008, 0x007, 0x007, 0x007, 0x006, 0x006, 0x005, 0x005, 0x005, 0x004, 0x004, 0x004, 0x003, 0x003, 0x003, 0x002, 0x002, 0x002, 0x002, 0x001, 0x001, 0x001, 0x001, 0x001, 0x001, 0x001, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000];
const EXPROM: [u16; 256] = [0x000, 0x003, 0x006, 0x008, 0x00b, 0x00e, 0x011, 0x014, 0x016, 0x019, 0x01c, 0x01f, 0x022, 0x025, 0x028, 0x02a, 0x02d, 0x030, 0x033, 0x036, 0x039, 0x03c, 0x03f, 0x042, 0x045, 0x048, 0x04b, 0x04e, 0x051, 0x054, 0x057, 0x05a, 0x05d, 0x060, 0x063, 0x066, 0x069, 0x06c, 0x06f, 0x072, 0x075, 0x078, 0x07b, 0x07e, 0x082, 0x085, 0x088, 0x08b, 0x08e, 0x091, 0x094, 0x098, 0x09b, 0x09e, 0x0a1, 0x0a4, 0x0a8, 0x0ab, 0x0ae, 0x0b1, 0x0b5, 0x0b8, 0x0bb, 0x0be, 0x0c2, 0x0c5, 0x0c8, 0x0cc, 0x0cf, 0x0d2, 0x0d6, 0x0d9, 0x0dc, 0x0e0, 0x0e3, 0x0e7, 0x0ea, 0x0ed, 0x0f1, 0x0f4, 0x0f8, 0x0fb, 0x0ff, 0x102, 0x106, 0x109, 0x10c, 0x110, 0x114, 0x117, 0x11b, 0x11e, 0x122, 0x125, 0x129, 0x12c, 0x130, 0x134, 0x137, 0x13b, 0x13e, 0x142, 0x146, 0x149, 0x14d, 0x151, 0x154, 0x158, 0x15c, 0x160, 0x163, 0x167, 0x16b, 0x16f, 0x172, 0x176, 0x17a, 0x17e, 0x181, 0x185, 0x189, 0x18d, 0x191, 0x195, 0x199, 0x19c, 0x1a0, 0x1a4, 0x1a8, 0x1ac, 0x1b0, 0x1b4, 0x1b8, 0x1bc, 0x1c0, 0x1c4, 0x1c8, 0x1cc, 0x1d0, 0x1d4, 0x1d8, 0x1dc, 0x1e0, 0x1e4, 0x1e8, 0x1ec, 0x1f0, 0x1f5, 0x1f9, 0x1fd, 0x201, 0x205, 0x209, 0x20e, 0x212, 0x216, 0x21a, 0x21e, 0x223, 0x227, 0x22b, 0x230, 0x234, 0x238, 0x23c, 0x241, 0x245, 0x249, 0x24e, 0x252, 0x257, 0x25b, 0x25f, 0x264, 0x268, 0x26d, 0x271, 0x276, 0x27a, 0x27f, 0x283, 0x288, 0x28c, 0x291, 0x295, 0x29a, 0x29e, 0x2a3, 0x2a8, 0x2ac, 0x2b1, 0x2b5, 0x2ba, 0x2bf, 0x2c4, 0x2c8, 0x2cd, 0x2d2, 0x2d6, 0x2db, 0x2e0, 0x2e5, 0x2e9, 0x2ee, 0x2f3, 0x2f8, 0x2fd, 0x302, 0x306, 0x30b, 0x310, 0x315, 0x31a, 0x31f, 0x324, 0x329, 0x32e, 0x333, 0x338, 0x33d, 0x342, 0x347, 0x34c, 0x351, 0x356, 0x35b, 0x360, 0x365, 0x36a, 0x370, 0x375, 0x37a, 0x37f, 0x384, 0x38a, 0x38f, 0x394, 0x399, 0x39f, 0x3a4, 0x3a9, 0x3ae, 0x3b4, 0x3b9, 0x3bf, 0x3c4, 0x3c9, 0x3cf, 0x3d4, 0x3da, 0x3df, 0x3e4, 0x3ea, 0x3ef, 0x3f5, 0x3fa];

const EG_ATTACK: u8 = 0;
const EG_DECAY: u8 = 1;
const EG_SUSTAIN: u8 = 2;
const EG_RELEASE: u8 = 3;

const FN_NOTE: [u32; 16] = [0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 3, 3, 3, 3, 3];
const EG_STEPHI: [[u32; 4]; 4] = [[0, 0, 0, 0], [1, 0, 0, 0], [1, 0, 1, 0], [1, 1, 1, 0]];
const EG_AM_SHIFT: [u32; 4] = [7, 3, 1, 0];
const PG_DETUNE: [u32; 8] = [16, 17, 19, 20, 22, 24, 27, 29];
const PG_LFO_SH1: [[u32; 8]; 8] = [
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 7, 7, 1, 1],
    [7, 7, 7, 7, 1, 1, 1, 1],
    [7, 7, 7, 1, 1, 1, 1, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
];
const PG_LFO_SH2: [[u32; 8]; 8] = [
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 2, 2, 2, 2],
    [7, 7, 7, 2, 2, 2, 7, 7],
    [7, 7, 2, 2, 7, 7, 2, 2],
    [7, 7, 2, 7, 7, 7, 2, 7],
    [7, 7, 7, 2, 7, 7, 2, 1],
    [7, 7, 7, 2, 7, 7, 2, 1],
    [7, 7, 7, 2, 7, 7, 2, 1],
];
const OP_OFFSET: [u32; 12] = [0x000, 0x001, 0x002, 0x100, 0x101, 0x102, 0x004, 0x005, 0x006, 0x104, 0x105, 0x106];
const CH_OFFSET: [u32; 6] = [0x000, 0x001, 0x002, 0x100, 0x101, 0x102];
const LFO_CYCLES: [u32; 8] = [108, 77, 71, 67, 62, 44, 8, 5];
const FM_ALGORITHM: [[[u8; 8]; 6]; 4] = [
    [[1, 1, 1, 1, 1, 1, 1, 1], [1, 1, 1, 1, 1, 1, 1, 1], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 1]],
    [[0, 1, 0, 0, 0, 1, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [1, 1, 1, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 1, 1, 1]],
    [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [1, 0, 0, 1, 1, 1, 1, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 1, 1, 1, 1]],
    [[0, 0, 1, 0, 0, 1, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0], [1, 1, 0, 1, 1, 0, 0, 0], [0, 0, 1, 0, 0, 0, 0, 0], [1, 1, 1, 1, 1, 1, 1, 1]],
];

/// The bits of `v` below `bit_index`, minus the bit `bit_index` (sign extension of a `bit_index + 1` bit number).
fn sign_extend(bit_index: u32, v: i32) -> i16 {
    let v = v as u32;
    ((v & ((1u32 << bit_index) - 1)).wrapping_sub(v & (1u32 << bit_index))) as i16
}

#[derive(Clone, Default)]
pub struct Ym2612 {
    cycles: u32,
    channel: u32,
    mol: i16,
    mor: i16,
    // IO
    write_data: u16,
    write_a: u8,
    write_d: u8,
    write_a_en: u8,
    write_d_en: u8,
    write_busy: u8,
    write_busy_cnt: u8,
    write_fm_address: u8,
    write_fm_data: u8,
    write_fm_mode_a: u16,
    address: u16,
    data: u8,
    pin_test_in: u8,
    busy: u8,
    // LFO
    lfo_en: u8,
    lfo_freq: u8,
    lfo_pm: u8,
    lfo_am: u8,
    lfo_cnt: u8,
    lfo_inc: u8,
    lfo_quotient: u8,
    // phase generator
    pg_fnum: u16,
    pg_block: u8,
    pg_kcode: u8,
    pg_inc: [u32; 24],
    pg_phase: [u32; 24],
    pg_reset: [u8; 24],
    pg_read: u32,
    // envelope generator
    eg_cycle: u8,
    eg_cycle_stop: u8,
    eg_shift: u8,
    eg_shift_lock: u8,
    eg_timer_low_lock: u8,
    eg_timer: u16,
    eg_timer_inc: u8,
    eg_quotient: u16,
    eg_custom_timer: u8,
    eg_rate: u8,
    eg_ksv: u8,
    eg_inc: u8,
    eg_ratemax: u8,
    eg_sl: [u8; 2],
    eg_lfo_am: u8,
    eg_tl: [u8; 2],
    eg_state: [u8; 24],
    eg_level: [u16; 24],
    eg_out: [u16; 24],
    eg_kon: [u8; 24],
    eg_kon_csm: [u8; 24],
    eg_kon_latch: [u8; 24],
    eg_ssg_enable: [u8; 24],
    eg_ssg_pgrst_latch: [u8; 24],
    eg_ssg_repeat_latch: [u8; 24],
    eg_ssg_hold_up_latch: [u8; 24],
    eg_ssg_dir: [u8; 24],
    eg_ssg_inv: [u8; 24],
    eg_read: [u32; 2],
    eg_read_inc: u8,
    // FM
    fm_op1: [[i16; 2]; 6],
    fm_op2: [i16; 6],
    fm_out: [i16; 24],
    fm_mod: [u16; 24],
    // channel
    ch_acc: [i16; 6],
    ch_out: [i16; 6],
    ch_lock: i16,
    ch_lock_l: u8,
    ch_lock_r: u8,
    // timers
    timer_a_cnt: u16,
    timer_a_reg: u16,
    timer_a_load_lock: u8,
    timer_a_load: u8,
    timer_a_enable: u8,
    timer_a_reset: u8,
    timer_a_load_latch: u8,
    timer_a_overflow_flag: u8,
    timer_a_overflow: u8,
    timer_b_cnt: u16,
    timer_b_subcnt: u8,
    timer_b_reg: u16,
    timer_b_load_lock: u8,
    timer_b_load: u8,
    timer_b_enable: u8,
    timer_b_reset: u8,
    timer_b_load_latch: u8,
    timer_b_overflow_flag: u8,
    timer_b_overflow: u8,
    // registers
    mode_test_21: [u8; 8],
    mode_test_2c: [u8; 8],
    mode_ch3: u8,
    mode_kon_channel: u8,
    mode_kon_operator: [u8; 4],
    mode_kon: [u8; 24],
    mode_csm: u8,
    mode_kon_csm: u8,
    dacen: u8,
    dacdata: i16,
    ks: [u8; 24],
    ar: [u8; 24],
    sr: [u8; 24],
    dt: [u8; 24],
    multi: [u8; 24],
    sl: [u8; 24],
    rr: [u8; 24],
    dr: [u8; 24],
    am: [u8; 24],
    tl: [u8; 24],
    ssg_eg: [u8; 24],
    fnum: [u16; 6],
    block: [u8; 6],
    kcode: [u8; 6],
    fnum_3ch: [u16; 6],
    block_3ch: [u8; 6],
    kcode_3ch: [u8; 6],
    reg_a4: u8,
    reg_ac: u8,
    connect: [u8; 6],
    fb: [u8; 6],
    pan_l: [u8; 6],
    pan_r: [u8; 6],
    ams: [u8; 6],
    pms: [u8; 6],
}

impl Ym2612 {
    /// OPN2_Reset
    pub fn new() -> Ym2612 {
        // everything zero, like memset
        let mut c = Ym2612::default();
        for i in 0..24 {
            c.eg_out[i] = 0x3ff;
            c.eg_level[i] = 0x3ff;
            c.eg_state[i] = EG_RELEASE;
            c.multi[i] = 1;
        }
        for i in 0..6 {
            c.pan_l[i] = 1;
            c.pan_r[i] = 1;
        }
        c
    }

    /// OPN2_Write: port 0 / 2 address (parts 1 / 2), port 1 / 3 data.
    pub fn write(&mut self, port: u32, data: u8) {
        let port = port & 3;
        self.write_data = (((port << 7) & 0x100) as u16) | data as u16;
        if port & 1 != 0 {
            self.write_d |= 1;
        } else {
            self.write_a |= 1;
        }
    }

    fn do_io(&mut self) {
        self.write_a_en = ((self.write_a & 0x03) == 0x01) as u8;
        self.write_d_en = ((self.write_d & 0x03) == 0x01) as u8;
        self.write_a <<= 1;
        self.write_d <<= 1;
        self.busy = self.write_busy;
        self.write_busy_cnt = self.write_busy_cnt.wrapping_add(self.write_busy);
        self.write_busy = (((self.write_busy != 0) && (self.write_busy_cnt >> 5) == 0) || self.write_d_en != 0) as u8;
        self.write_busy_cnt &= 0x1f;
    }

    fn do_reg_write(&mut self) {
        let mut slot = self.cycles % 12;
        let channel = self.channel as usize;
        // update registers
        if self.write_fm_data != 0 {
            if OP_OFFSET[slot as usize] == (self.address as u32 & 0x107) {
                if self.address & 0x08 != 0 {
                    slot += 12;
                }
                let s = slot as usize;
                match self.address & 0xf0 {
                    0x30 => {
                        self.multi[s] = self.data & 0x0f;
                        if self.multi[s] == 0 {
                            self.multi[s] = 1;
                        } else {
                            self.multi[s] <<= 1;
                        }
                        self.dt[s] = (self.data >> 4) & 0x07;
                    }
                    0x40 => self.tl[s] = self.data & 0x7f,
                    0x50 => {
                        self.ar[s] = self.data & 0x1f;
                        self.ks[s] = (self.data >> 6) & 0x03;
                    }
                    0x60 => {
                        self.dr[s] = self.data & 0x1f;
                        self.am[s] = (self.data >> 7) & 0x01;
                    }
                    0x70 => self.sr[s] = self.data & 0x1f,
                    0x80 => {
                        self.rr[s] = self.data & 0x0f;
                        self.sl[s] = (self.data >> 4) & 0x0f;
                        self.sl[s] |= (self.sl[s] + 1) & 0x10;
                    }
                    0x90 => self.ssg_eg[s] = self.data & 0x0f,
                    _ => {}
                }
            }
            // channel
            if CH_OFFSET[channel] == (self.address as u32 & 0x103) {
                match self.address & 0xfc {
                    0xa0 => {
                        self.fnum[channel] = (self.data as u16 & 0xff) | ((self.reg_a4 as u16 & 0x07) << 8);
                        self.block[channel] = (self.reg_a4 >> 3) & 0x07;
                        self.kcode[channel] = (self.block[channel] << 2) | FN_NOTE[(self.fnum[channel] >> 7) as usize] as u8;
                    }
                    0xa4 => self.reg_a4 = self.data,
                    0xa8 => {
                        self.fnum_3ch[channel] = (self.data as u16 & 0xff) | ((self.reg_ac as u16 & 0x07) << 8);
                        self.block_3ch[channel] = (self.reg_ac >> 3) & 0x07;
                        self.kcode_3ch[channel] = (self.block_3ch[channel] << 2) | FN_NOTE[(self.fnum_3ch[channel] >> 7) as usize] as u8;
                    }
                    0xac => self.reg_ac = self.data,
                    0xb0 => {
                        self.connect[channel] = self.data & 0x07;
                        self.fb[channel] = (self.data >> 3) & 0x07;
                    }
                    0xb4 => {
                        self.pms[channel] = self.data & 0x07;
                        self.ams[channel] = (self.data >> 4) & 0x03;
                        self.pan_l[channel] = (self.data >> 7) & 0x01;
                        self.pan_r[channel] = (self.data >> 6) & 0x01;
                    }
                    _ => {}
                }
            }
        }

        if self.write_a_en != 0 || self.write_d_en != 0 {
            // data
            if self.write_a_en != 0 {
                self.write_fm_data = 0;
            }
            if self.write_fm_address != 0 && self.write_d_en != 0 {
                self.write_fm_data = 1;
            }
            // address
            if self.write_a_en != 0 {
                if (self.write_data & 0xf0) != 0x00 {
                    // FM write
                    self.address = self.write_data;
                    self.write_fm_address = 1;
                } else {
                    // SSG write
                    self.write_fm_address = 0;
                }
            }
            // FM mode, data
            if self.write_d_en != 0 && (self.write_data & 0x100) == 0 {
                match self.write_fm_mode_a {
                    0x21 => {
                        for i in 0..8 {
                            self.mode_test_21[i] = ((self.write_data >> i) & 0x01) as u8;
                        }
                    }
                    0x22 => {
                        self.lfo_en = if (self.write_data >> 3) & 0x01 != 0 { 0x7f } else { 0 };
                        self.lfo_freq = (self.write_data & 0x07) as u8;
                    }
                    0x24 => {
                        self.timer_a_reg &= 0x03;
                        self.timer_a_reg |= (self.write_data & 0xff) << 2;
                    }
                    0x25 => {
                        self.timer_a_reg &= 0x3fc;
                        self.timer_a_reg |= self.write_data & 0x03;
                    }
                    0x26 => self.timer_b_reg = self.write_data & 0xff,
                    0x27 => {
                        self.mode_ch3 = ((self.write_data & 0xc0) >> 6) as u8;
                        self.mode_csm = (self.mode_ch3 == 2) as u8;
                        self.timer_a_load = (self.write_data & 0x01) as u8;
                        self.timer_a_enable = ((self.write_data >> 2) & 0x01) as u8;
                        self.timer_a_reset = ((self.write_data >> 4) & 0x01) as u8;
                        self.timer_b_load = ((self.write_data >> 1) & 0x01) as u8;
                        self.timer_b_enable = ((self.write_data >> 3) & 0x01) as u8;
                        self.timer_b_reset = ((self.write_data >> 5) & 0x01) as u8;
                    }
                    0x28 => {
                        for i in 0..4 {
                            self.mode_kon_operator[i] = ((self.write_data >> (4 + i)) & 0x01) as u8;
                        }
                        if (self.write_data & 0x03) == 0x03 {
                            self.mode_kon_channel = 0xff;
                        } else {
                            self.mode_kon_channel = ((self.write_data & 0x03) + ((self.write_data >> 2) & 1) * 3) as u8;
                        }
                    }
                    0x2a => {
                        self.dacdata &= 0x01;
                        self.dacdata |= ((self.write_data ^ 0x80) << 1) as i16;
                    }
                    0x2b => self.dacen = (self.write_data >> 7) as u8,
                    0x2c => {
                        for i in 0..8 {
                            self.mode_test_2c[i] = ((self.write_data >> i) & 0x01) as u8;
                        }
                        self.dacdata &= 0x1fe;
                        self.dacdata |= self.mode_test_2c[3] as i16;
                        self.eg_custom_timer = (self.mode_test_2c[7] == 0 && self.mode_test_2c[6] != 0) as u8;
                    }
                    _ => {}
                }
            }
            // address
            if self.write_a_en != 0 {
                self.write_fm_mode_a = self.write_data & 0x1ff;
            }
        }

        if self.write_fm_data != 0 {
            self.data = (self.write_data & 0xff) as u8;
        }
    }

    fn phase_calc_increment(&mut self) {
        let chan = self.channel as usize;
        let slot = self.cycles as usize;
        let mut fnum: u32 = self.pg_fnum as u32;
        let fnum_h = fnum >> 4;
        let lfo = self.lfo_pm;
        let mut lfo_l = lfo & 0x0f;
        let pms = self.pms[chan] as usize;
        let dt = self.dt[slot];
        let dt_l = dt & 0x03;
        let mut detune: u32 = 0;
        let mut kcode = self.pg_kcode;

        fnum <<= 1;
        // apply LFO
        if lfo_l & 0x08 != 0 {
            lfo_l ^= 0x0f;
        }
        let mut fm = (fnum_h >> PG_LFO_SH1[pms][lfo_l as usize]) + (fnum_h >> PG_LFO_SH2[pms][lfo_l as usize]);
        if pms > 5 {
            fm <<= pms - 5;
        }
        fm >>= 2;
        if lfo & 0x10 != 0 {
            fnum = fnum.wrapping_sub(fm);
        } else {
            fnum = fnum.wrapping_add(fm);
        }
        fnum &= 0xfff;

        let mut basefreq: u32 = (fnum << self.pg_block) >> 2;

        // apply detune
        if dt_l != 0 {
            if kcode > 0x1c {
                kcode = 0x1c;
            }
            let block = kcode >> 2;
            let note = kcode & 0x03;
            let sum = block + 9 + (((dt_l == 3) as u8) | (dt_l & 0x02));
            let sum_h = sum >> 1;
            let sum_l = sum & 0x01;
            detune = PG_DETUNE[((sum_l << 2) | note) as usize] >> (9 - sum_h);
        }
        if dt & 0x04 != 0 {
            basefreq = basefreq.wrapping_sub(detune);
        } else {
            basefreq = basefreq.wrapping_add(detune);
        }
        basefreq &= 0x1ffff;
        self.pg_inc[slot] = (basefreq * self.multi[slot] as u32) >> 1;
        self.pg_inc[slot] &= 0xfffff;
    }

    fn phase_generate(&mut self) {
        // mask increment
        let mut slot = ((self.cycles + 20) % 24) as usize;
        if self.pg_reset[slot] != 0 {
            self.pg_inc[slot] = 0;
        }
        // phase step
        slot = ((self.cycles + 19) % 24) as usize;
        if self.pg_reset[slot] != 0 || self.mode_test_21[3] != 0 {
            self.pg_phase[slot] = 0;
        }
        self.pg_phase[slot] = self.pg_phase[slot].wrapping_add(self.pg_inc[slot]);
        self.pg_phase[slot] &= 0xfffff;
    }

    fn envelope_ssg_eg(&mut self) {
        let slot = self.cycles as usize;
        let mut direction: u8 = 0;
        self.eg_ssg_pgrst_latch[slot] = 0;
        self.eg_ssg_repeat_latch[slot] = 0;
        self.eg_ssg_hold_up_latch[slot] = 0;
        if self.ssg_eg[slot] & 0x08 != 0 {
            direction = self.eg_ssg_dir[slot];
            if self.eg_level[slot] & 0x200 != 0 {
                // reset
                if (self.ssg_eg[slot] & 0x03) == 0x00 {
                    self.eg_ssg_pgrst_latch[slot] = 1;
                }
                // repeat
                if (self.ssg_eg[slot] & 0x01) == 0x00 {
                    self.eg_ssg_repeat_latch[slot] = 1;
                }
                // inverse
                if (self.ssg_eg[slot] & 0x03) == 0x02 {
                    direction ^= 1;
                }
                if (self.ssg_eg[slot] & 0x03) == 0x03 {
                    direction = 1;
                }
            }
            // hold up
            if self.eg_kon_latch[slot] != 0 && ((self.ssg_eg[slot] & 0x07) == 0x05 || (self.ssg_eg[slot] & 0x07) == 0x03) {
                self.eg_ssg_hold_up_latch[slot] = 1;
            }
            direction &= self.eg_kon[slot];
        }
        self.eg_ssg_dir[slot] = direction;
        self.eg_ssg_enable[slot] = (self.ssg_eg[slot] >> 3) & 0x01;
        self.eg_ssg_inv[slot] = (self.eg_ssg_dir[slot] ^ (((self.ssg_eg[slot] >> 2) & 0x01) & ((self.ssg_eg[slot] >> 3) & 0x01))) & self.eg_kon[slot];
    }

    fn envelope_adsr(&mut self) {
        let slot = ((self.cycles + 22) % 24) as usize;
        let nkon = self.eg_kon_latch[slot];
        let okon = self.eg_kon[slot];
        let eg_off: bool;
        let mut nextstate = self.eg_state[slot];
        let mut inc: i16 = 0;
        self.eg_read[0] = self.eg_read_inc as u32;
        self.eg_read_inc = (self.eg_inc > 0) as u8;

        // reset phase generator
        self.pg_reset[slot] = ((nkon != 0 && okon == 0) || self.eg_ssg_pgrst_latch[slot] != 0) as u8;

        // key on/off
        let kon_event = (nkon != 0 && okon == 0) || (okon != 0 && self.eg_ssg_repeat_latch[slot] != 0);
        let koff_event = okon != 0 && nkon == 0;

        let mut level: i16 = self.eg_level[slot] as i16;
        let mut ssg_level: i16 = level;

        if self.eg_ssg_inv[slot] != 0 {
            // inverse
            ssg_level = 512 - level;
            ssg_level &= 0x3ff;
        }
        if koff_event {
            level = ssg_level;
        }
        if self.eg_ssg_enable[slot] != 0 {
            eg_off = (level >> 9) != 0;
        } else {
            eg_off = (level & 0x3f0) == 0x3f0;
        }
        let mut nextlevel: i16 = level;
        if kon_event {
            nextstate = EG_ATTACK;
            // instant attack
            if self.eg_ratemax != 0 {
                nextlevel = 0;
            } else if self.eg_state[slot] == EG_ATTACK && level != 0 && self.eg_inc != 0 && nkon != 0 {
                inc = ((((!(level as i32)) << self.eg_inc) >> 5) as i16) as i16;
            }
        } else {
            match self.eg_state[slot] {
                EG_ATTACK => {
                    if level == 0 {
                        nextstate = EG_DECAY;
                    } else if self.eg_inc != 0 && self.eg_ratemax == 0 && nkon != 0 {
                        inc = (((!(level as i32)) << self.eg_inc) >> 5) as i16;
                    }
                }
                EG_DECAY => {
                    if (level >> 4) == ((self.eg_sl[1] as i16) << 1) {
                        nextstate = EG_SUSTAIN;
                    } else if !eg_off && self.eg_inc != 0 {
                        inc = 1 << (self.eg_inc - 1);
                        if self.eg_ssg_enable[slot] != 0 {
                            inc <<= 2;
                        }
                    }
                }
                EG_SUSTAIN | EG_RELEASE => {
                    if !eg_off && self.eg_inc != 0 {
                        inc = 1 << (self.eg_inc - 1);
                        if self.eg_ssg_enable[slot] != 0 {
                            inc <<= 2;
                        }
                    }
                }
                _ => {}
            }
            if nkon == 0 {
                nextstate = EG_RELEASE;
            }
        }
        if self.eg_kon_csm[slot] != 0 {
            nextlevel |= (self.eg_tl[1] as i16) << 3;
        }

        // envelope off
        if !kon_event && self.eg_ssg_hold_up_latch[slot] == 0 && self.eg_state[slot] != EG_ATTACK && eg_off {
            nextstate = EG_RELEASE;
            nextlevel = 0x3ff;
        }

        nextlevel = nextlevel.wrapping_add(inc);

        self.eg_kon[slot] = self.eg_kon_latch[slot];
        self.eg_level[slot] = (nextlevel as u16) & 0x3ff;
        self.eg_state[slot] = nextstate;
    }

    fn envelope_prepare(&mut self) {
        let mut inc: u32 = 0;
        let slot = self.cycles as usize;

        // prepare increment
        let mut rate: u32 = ((self.eg_rate as u32) << 1) + self.eg_ksv as u32;
        if rate > 0x3f {
            rate = 0x3f;
        }
        let sum = ((rate >> 2) + self.eg_shift_lock as u32) & 0x0f;
        if self.eg_rate != 0 && self.eg_quotient == 2 {
            if rate < 48 {
                match sum {
                    12 => inc = 1,
                    13 => inc = (rate >> 1) & 0x01,
                    14 => inc = rate & 0x01,
                    _ => {}
                }
            } else {
                inc = EG_STEPHI[(rate & 0x03) as usize][self.eg_timer_low_lock as usize] + (rate >> 2) - 11;
                if inc > 4 {
                    inc = 4;
                }
            }
        }
        self.eg_inc = inc as u8;
        self.eg_ratemax = ((rate >> 1) == 0x1f) as u8;

        // prepare rate & ksv
        let mut rate_sel = self.eg_state[slot];
        if (self.eg_kon[slot] != 0 && self.eg_ssg_repeat_latch[slot] != 0) || (self.eg_kon[slot] == 0 && self.eg_kon_latch[slot] != 0) {
            rate_sel = EG_ATTACK;
        }
        match rate_sel {
            EG_ATTACK => self.eg_rate = self.ar[slot],
            EG_DECAY => self.eg_rate = self.dr[slot],
            EG_SUSTAIN => self.eg_rate = self.sr[slot],
            EG_RELEASE => self.eg_rate = (self.rr[slot] << 1) | 0x01,
            _ => {}
        }
        self.eg_ksv = self.pg_kcode >> (self.ks[slot] ^ 0x03);
        if self.am[slot] != 0 {
            self.eg_lfo_am = self.lfo_am >> EG_AM_SHIFT[self.ams[self.channel as usize] as usize];
        } else {
            self.eg_lfo_am = 0;
        }
        // delay TL & SL value
        self.eg_tl[1] = self.eg_tl[0];
        self.eg_tl[0] = self.tl[slot];
        self.eg_sl[1] = self.eg_sl[0];
        self.eg_sl[0] = self.sl[slot];
    }

    fn envelope_generate(&mut self) {
        let slot = ((self.cycles + 23) % 24) as usize;
        let mut level: u16 = self.eg_level[slot];
        if self.eg_ssg_inv[slot] != 0 {
            // inverse
            level = 512u16.wrapping_sub(level);
        }
        if self.mode_test_21[5] != 0 {
            level = 0;
        }
        level &= 0x3ff;
        // apply AM LFO
        level = level.wrapping_add(self.eg_lfo_am as u16);
        // apply TL
        if !(self.mode_csm != 0 && self.channel == 2 + 1) {
            level = level.wrapping_add((self.eg_tl[0] as u16) << 3);
        }
        if level > 0x3ff {
            level = 0x3ff;
        }
        self.eg_out[slot] = level;
    }

    fn update_lfo(&mut self) {
        let c = LFO_CYCLES[self.lfo_freq as usize];
        if (self.lfo_quotient as u32 & c) == c {
            self.lfo_quotient = 0;
            self.lfo_cnt = self.lfo_cnt.wrapping_add(1);
        } else {
            self.lfo_quotient = self.lfo_quotient.wrapping_add(self.lfo_inc);
        }
        self.lfo_cnt &= self.lfo_en;
    }

    fn fm_prepare(&mut self) {
        let mut slot = ((self.cycles + 6) % 24) as usize;
        let channel = self.channel as usize;
        let op = slot / 6;
        let connect = self.connect[channel] as usize;
        let prevslot = ((self.cycles + 18) % 24) as usize;

        // calculate modulation
        let mut mod1: i16 = 0;
        let mut mod2: i16 = 0;
        if FM_ALGORITHM[op][0][connect] != 0 {
            mod2 |= self.fm_op1[channel][0];
        }
        if FM_ALGORITHM[op][1][connect] != 0 {
            mod1 |= self.fm_op1[channel][1];
        }
        if FM_ALGORITHM[op][2][connect] != 0 {
            mod1 |= self.fm_op2[channel];
        }
        if FM_ALGORITHM[op][3][connect] != 0 {
            mod2 |= self.fm_out[prevslot];
        }
        if FM_ALGORITHM[op][4][connect] != 0 {
            mod1 |= self.fm_out[prevslot];
        }
        let mut m: i16 = mod1.wrapping_add(mod2);
        if op == 0 {
            // feedback
            m >>= 10 - self.fb[channel] as i32;
            if self.fb[channel] == 0 {
                m = 0;
            }
        } else {
            m >>= 1;
        }
        self.fm_mod[slot] = m as u16;

        slot = ((self.cycles + 18) % 24) as usize;
        // OP1
        if slot / 6 == 0 {
            self.fm_op1[channel][1] = self.fm_op1[channel][0];
            self.fm_op1[channel][0] = self.fm_out[slot];
        }
        // OP2
        if slot / 6 == 2 {
            self.fm_op2[channel] = self.fm_out[slot];
        }
    }

    fn ch_generate(&mut self) {
        let slot = ((self.cycles + 18) % 24) as usize;
        let channel = self.channel as usize;
        let op = slot / 6;
        let test_dac = self.mode_test_2c[5];
        let mut acc: i16 = self.ch_acc[channel];
        let mut add: i16 = test_dac as i16;
        if op == 0 && test_dac == 0 {
            acc = 0;
        }
        if FM_ALGORITHM[op][5][self.connect[channel] as usize] != 0 && test_dac == 0 {
            add += self.fm_out[slot] >> 5;
        }
        let mut sum: i16 = acc.wrapping_add(add);
        // clamp
        if sum > 255 {
            sum = 255;
        } else if sum < -256 {
            sum = -256;
        }
        if op == 0 || test_dac != 0 {
            self.ch_out[channel] = self.ch_acc[channel];
        }
        self.ch_acc[channel] = sum;
    }

    fn ch_output(&mut self) {
        let cycles = self.cycles;
        let slot = self.cycles;
        let mut channel = self.channel as usize;
        let test_dac = self.mode_test_2c[5];
        if slot < 12 {
            // ch 4, 5, 6
            channel += 1;
        }
        if (cycles & 3) == 0 {
            if test_dac == 0 {
                // lock value
                self.ch_lock = self.ch_out[channel];
            }
            self.ch_lock_l = self.pan_l[channel];
            self.ch_lock_r = self.pan_r[channel];
        }
        // ch 6
        let out: i16 = if ((cycles >> 2) == 1 && self.dacen != 0) || test_dac != 0 { sign_extend(8, self.dacdata as i32) } else { self.ch_lock };
        self.mol = 0;
        self.mor = 0;

        // YM2612 DAC emulation (not verified, said the original's author)
        let out_en = ((cycles & 3) == 3) || test_dac != 0;
        let mut out = out;
        let mut sign: i16 = out >> 8;
        if out >= 0 {
            out += 1;
            sign += 1;
        }
        self.mol = if self.ch_lock_l != 0 && out_en { out } else { sign };
        self.mor = if self.ch_lock_r != 0 && out_en { out } else { sign };
        // amplify signal
        self.mol = self.mol.wrapping_mul(3);
        self.mor = self.mor.wrapping_mul(3);
    }

    fn fm_generate(&mut self) {
        let slot = ((self.cycles + 19) % 24) as usize;
        // calculate phase
        let phase: u16 = (self.fm_mod[slot].wrapping_add((self.pg_phase[slot] >> 10) as u16)) & 0x3ff;
        let quarter: u16 = if phase & 0x100 != 0 { (phase ^ 0xff) & 0xff } else { phase & 0xff };
        let mut level: u16 = LOGSINROM[quarter as usize];
        // apply envelope
        level = level.wrapping_add(self.eg_out[slot] << 2);
        // transform
        if level > 0x1fff {
            level = 0x1fff;
        }
        let mut output: i32 = (((EXPROM[((level & 0xff) ^ 0xff) as usize] | 0x400) as i32) << 2) >> (level >> 8);
        let t4 = (self.mode_test_21[4] as i32) << 13;
        if phase & 0x200 != 0 {
            output = ((!output) ^ t4) + 1;
        } else {
            output ^= t4;
        }
        self.fm_out[slot] = sign_extend(13, output as i16 as i32);
    }

    fn do_timer_a(&mut self) {
        let mut load: u8 = self.timer_a_overflow;
        if self.cycles == 2 {
            // lock load value
            load |= (self.timer_a_load_lock == 0 && self.timer_a_load != 0) as u8;
            self.timer_a_load_lock = self.timer_a_load;
            if self.mode_csm != 0 {
                // CSM key on
                self.mode_kon_csm = load;
            } else {
                self.mode_kon_csm = 0;
            }
        }
        // load counter
        let mut time: u16 = if self.timer_a_load_latch != 0 { self.timer_a_reg } else { self.timer_a_cnt };
        self.timer_a_load_latch = load;
        // increase counter
        if (self.cycles == 1 && self.timer_a_load_lock != 0) || self.mode_test_21[2] != 0 {
            time = time.wrapping_add(1);
        }
        // set overflow flag
        if self.timer_a_reset != 0 {
            self.timer_a_reset = 0;
            self.timer_a_overflow_flag = 0;
        } else {
            self.timer_a_overflow_flag |= self.timer_a_overflow & self.timer_a_enable;
        }
        self.timer_a_overflow = (time >> 10) as u8;
        self.timer_a_cnt = time & 0x3ff;
    }

    fn do_timer_b(&mut self) {
        let mut load: u8 = self.timer_b_overflow;
        if self.cycles == 2 {
            // lock load value
            load |= (self.timer_b_load_lock == 0 && self.timer_b_load != 0) as u8;
            self.timer_b_load_lock = self.timer_b_load;
        }
        // load counter
        let mut time: u16 = if self.timer_b_load_latch != 0 { self.timer_b_reg } else { self.timer_b_cnt };
        self.timer_b_load_latch = load;
        // increase counter
        if self.cycles == 1 {
            self.timer_b_subcnt = self.timer_b_subcnt.wrapping_add(1);
        }
        if (self.timer_b_subcnt == 0x10 && self.timer_b_load_lock != 0) || self.mode_test_21[2] != 0 {
            time = time.wrapping_add(1);
        }
        self.timer_b_subcnt &= 0x0f;
        // set overflow flag
        if self.timer_b_reset != 0 {
            self.timer_b_reset = 0;
            self.timer_b_overflow_flag = 0;
        } else {
            self.timer_b_overflow_flag |= self.timer_b_overflow & self.timer_b_enable;
        }
        self.timer_b_overflow = (time >> 8) as u8;
        self.timer_b_cnt = time & 0xff;
    }

    fn key_on(&mut self) {
        let slot = self.cycles as usize;
        let chan = self.channel as usize;
        // key on
        self.eg_kon_latch[slot] = self.mode_kon[slot];
        self.eg_kon_csm[slot] = 0;
        if self.channel == 2 && self.mode_kon_csm != 0 {
            // CSM key on
            self.eg_kon_latch[slot] = 1;
            self.eg_kon_csm[slot] = 1;
        }
        if self.cycles == self.mode_kon_channel as u32 {
            self.mode_kon[chan] = self.mode_kon_operator[0];
            self.mode_kon[chan + 12] = self.mode_kon_operator[1];
            self.mode_kon[chan + 6] = self.mode_kon_operator[2];
            self.mode_kon[chan + 18] = self.mode_kon_operator[3];
        }
    }

    /// OPN2_Clock: one step of the chip (a sixth of a master clock cycle count of 6); returns the left and right outputs.
    pub fn clock(&mut self) -> (i16, i16) {
        let slot = self.cycles;
        self.lfo_inc = self.mode_test_21[1];
        self.pg_read >>= 1;
        self.eg_read[1] >>= 1;
        self.eg_cycle = self.eg_cycle.wrapping_add(1);
        // lock envelope generator timer value
        if self.cycles == 1 && self.eg_quotient == 2 {
            if self.eg_cycle_stop != 0 {
                self.eg_shift_lock = 0;
            } else {
                self.eg_shift_lock = self.eg_shift.wrapping_add(1);
            }
            self.eg_timer_low_lock = (self.eg_timer & 0x03) as u8;
        }
        // cycle specific functions
        match self.cycles {
            0 => {
                self.lfo_pm = self.lfo_cnt >> 2;
                if self.lfo_cnt & 0x40 != 0 {
                    self.lfo_am = self.lfo_cnt & 0x3f;
                } else {
                    self.lfo_am = self.lfo_cnt ^ 0x3f;
                }
                self.lfo_am <<= 1;
            }
            1 => {
                self.eg_quotient += 1;
                self.eg_quotient %= 3;
                self.eg_cycle = 0;
                self.eg_cycle_stop = 1;
                self.eg_shift = 0;
                self.eg_timer_inc |= (self.eg_quotient >> 1) as u8;
                self.eg_timer = self.eg_timer.wrapping_add(self.eg_timer_inc as u16);
                self.eg_timer_inc = (self.eg_timer >> 12) as u8;
                self.eg_timer &= 0xfff;
            }
            2 => {
                self.pg_read = self.pg_phase[21] & 0x3ff;
                self.eg_read[1] = self.eg_out[0] as u32;
            }
            13 => {
                self.eg_cycle = 0;
                self.eg_cycle_stop = 1;
                self.eg_shift = 0;
                self.eg_timer = self.eg_timer.wrapping_add(self.eg_timer_inc as u16);
                self.eg_timer_inc = (self.eg_timer >> 12) as u8;
                self.eg_timer &= 0xfff;
            }
            23 => self.lfo_inc |= 1,
            _ => {}
        }
        self.eg_timer &= !(((self.mode_test_21[5] as u32) << self.eg_cycle) as u16);
        if ((((self.eg_timer as u32) >> self.eg_cycle) | (self.pin_test_in & self.eg_custom_timer) as u32) & self.eg_cycle_stop as u32) != 0 {
            self.eg_shift = self.eg_cycle;
            self.eg_cycle_stop = 0;
        }

        self.do_io();

        self.do_timer_a();
        self.do_timer_b();
        self.key_on();

        self.ch_output();
        self.ch_generate();

        self.fm_prepare();
        self.fm_generate();

        self.phase_generate();
        self.phase_calc_increment();

        self.envelope_adsr();
        self.envelope_generate();
        self.envelope_ssg_eg();
        self.envelope_prepare();

        // prepare fnum & block
        let ch1 = ((self.channel + 1) % 6) as usize;
        if self.mode_ch3 != 0 {
            // channel 3 special mode
            match slot {
                1 => {
                    self.pg_fnum = self.fnum_3ch[1];
                    self.pg_block = self.block_3ch[1];
                    self.pg_kcode = self.kcode_3ch[1];
                }
                7 => {
                    self.pg_fnum = self.fnum_3ch[0];
                    self.pg_block = self.block_3ch[0];
                    self.pg_kcode = self.kcode_3ch[0];
                }
                13 => {
                    self.pg_fnum = self.fnum_3ch[2];
                    self.pg_block = self.block_3ch[2];
                    self.pg_kcode = self.kcode_3ch[2];
                }
                _ => {
                    self.pg_fnum = self.fnum[ch1];
                    self.pg_block = self.block[ch1];
                    self.pg_kcode = self.kcode[ch1];
                }
            }
        } else {
            self.pg_fnum = self.fnum[ch1];
            self.pg_block = self.block[ch1];
            self.pg_kcode = self.kcode[ch1];
        }

        self.update_lfo();
        self.do_reg_write();
        self.cycles = (self.cycles + 1) % 24;
        self.channel = self.cycles % 6;

        (self.mol, self.mor)
    }
}
