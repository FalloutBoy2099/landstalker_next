#!/usr/bin/env python3
"""Records the music of the game as VGM files (data/audio/music_XX.vgz): a log of the writes of the sound driver to the YM2612
and the SN76489, with their times. The C++ engine runs the game's own Z80 sound driver (LS_EVENT_DUMP) and writes every register
write; `lsctl evt-to-vgm` finds where the song starts to repeat itself (the loop of the VGM) and writes the file.

The game plays them with the chips emulated in Rust (crates/ls_data/src/ym2612.rs: a port of the engine's Nuked-OPN2, which
gives the very same samples, and a PSG), so the music sounds as the engine's, without the 30 MB of ogg.

Usage: render_vgm.py [--engine path/to/dds520.exe] [ids...]     (ids in hex, default: all the songs)
The song's length comes from music_XX.json (made by render_sounds.py): a song is recorded for two loops and more.
"""
import argparse
import json
import os
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / 'data' / 'audio'
ENGINE = ROOT.parent / 'landstalker_engine' / 'build' / 'RelWithDebInfo' / 'dds520.exe'
LSCTL = ROOT / 'target' / 'release' / 'lsctl.exe'

MUSIC = list(range(0x01, 0x17)) + [0x1B, 0x1D, 0x1F] + list(range(0x21, 0x30))


# sounds that never end by themselves (the dog whistle is a trill the game stops): recorded for this long, once
SHORT = {0x0D: 10}


def song_info(sid):
    """(seconds to record, loops): the intro, two loops and some margin; a jingle (music_XX.json says "loop": false) plays once:
    40 s are enough to hear it end."""
    if sid in SHORT:
        return SHORT[sid], False
    meta = OUT / ('music_%02X.json' % sid)
    if not meta.exists():
        return 300, True
    j = json.loads(meta.read_text())
    if not j.get('loop', True):
        return 40, False
    length = j.get('loop_end', 0) / j.get('sample_rate', 53267)
    return min(450, int(2.2 * length + 40)), True


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--engine', default=str(ENGINE))
    ap.add_argument('--no-retry', action='store_true', help='do not record longer when no loop is found')
    ap.add_argument('ids', nargs='*')
    a = ap.parse_args()
    ids = [int(i, 16) for i in a.ids] or MUSIC
    for sid in ids:
        secs, loops = song_info(sid)
        out = OUT / ('music_%02X.vgz' % sid)
        while True:
            with tempfile.TemporaryDirectory() as t:
                evt = Path(t) / 'song.evt'
                env = dict(os.environ, LS_EVENT_DUMP='%#x:%d:%s' % (sid, secs, evt))
                subprocess.run([a.engine], cwd=str(Path(a.engine).parent), env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=7200)
                r = subprocess.run([str(LSCTL), 'evt-to-vgm', str(evt), str(out)] + ([] if loops else ['--once']), capture_output=True, text=True)
            text = (r.stdout or r.stderr).strip().replace(str(OUT) + os.sep, '')
            print('%02X (%d s recorded): %s' % (sid, secs, text), flush=True)
            # no loop found in a song that should have one: it may be longer than the recording (the length in music_XX.json
            # comes from the audio, which can be wrong): record longer, up to 10 minutes
            if loops and 'no loop' in text and secs < 600 and not a.no_retry:
                secs = min(600, int(secs * 2.5))
                continue
            break


if __name__ == '__main__':
    main()
