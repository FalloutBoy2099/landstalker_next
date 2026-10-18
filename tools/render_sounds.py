#!/usr/bin/env python3
"""Renders the sounds of the game to audio files (data/audio), once, with the C++ engine's Z80 + YM2612 emulation
(LS_SOUND_DUMP): sound effects -> sfx_XX.flac, music -> music_XX.ogg + music_XX.json.

A song loops in the game: the file holds the intro and one loop, music_XX.json gives {"sample_rate", "loop_start",
"loop_end"} in samples (loop_end = end of the file) or {"loop": false} for a jingle that plays once.

Usage: render_sounds.py [--engine path/to/dds520.exe] [ids...]     (ids in hex, default: all)
Needs numpy, scipy and ffmpeg (libvorbis).
"""
import argparse
import json
import subprocess
import sys
import tempfile
import wave
from pathlib import Path

import numpy as np
from scipy.signal import fftconvolve

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / 'data' / 'audio'
ENGINE = ROOT.parent / 'landstalker_engine' / 'build' / 'RelWithDebInfo' / 'dds520.exe'

MUSIC = list(range(0x01, 0x17)) + [0x1B, 0x1D, 0x1F] + list(range(0x21, 0x30))
SFX = list(range(0x41, 0x7B))


def dump(engine, sid, seconds, path):
    import os
    env = dict(os.environ, LS_SOUND_DUMP='%#x:%d:%s' % (sid, seconds, path))
    subprocess.run([str(engine)], cwd=str(Path(engine).parent), env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=600)


def read_wav(path):
    with wave.open(str(path)) as w:
        sr = w.getframerate()
        raw = np.frombuffer(w.readframes(w.getnframes()), dtype='<i2').reshape(-1, w.getnchannels())
    return sr, raw


def write_flac(stereo, sr, path):
    """Lossless: the sound effects are short PCM samples (cahier des charges, section 10: WAV / FLAC)."""
    with tempfile.TemporaryDirectory() as t:
        wav = Path(t) / 'x.wav'
        with wave.open(str(wav), 'wb') as w:
            w.setnchannels(stereo.shape[1])
            w.setsampwidth(2)
            w.setframerate(sr)
            w.writeframes(stereo.astype('<i2').tobytes())
        subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-i', str(wav), '-c:a', 'flac', str(path)], check=True)


def write_ogg(stereo, sr, path, quality=4):
    with tempfile.TemporaryDirectory() as t:
        wav = Path(t) / 'x.wav'
        with wave.open(str(wav), 'wb') as w:
            w.setnchannels(stereo.shape[1])
            w.setsampwidth(2)
            w.setframerate(sr)
            w.writeframes(stereo.astype('<i2').tobytes())
        subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-i', str(wav), '-c:a', 'libvorbis', '-q:a', str(quality), str(path)], check=True)


def rms(x):
    return float(np.sqrt(np.mean(x.astype(np.float64) ** 2))) if len(x) else 0.0


def trim_end(stereo, sr, threshold=30.0):
    """Cuts the silence at the end (keeping a little)."""
    mono = np.abs(stereo.astype(np.int32)).max(axis=1)
    loud = np.nonzero(mono > threshold)[0]
    if not len(loud):
        return stereo[: int(0.1 * sr)]
    return stereo[: min(len(stereo), loud[-1] + int(0.15 * sr))]


def corr_search(d, x):
    """Normalised correlation of the segment x with every position of d."""
    num = fftconvolve(d, x[::-1], mode='valid')
    e = np.concatenate([[0.0], np.cumsum(d.astype(np.float64) ** 2)])
    win = e[len(x):] - e[:-len(x)]
    return num / np.sqrt(win * float(np.dot(x, x)) + 1e-9)


def find_loop(stereo, sr):
    """(loop_start, loop_end) in samples, or None when the song does not repeat."""
    mono = stereo.astype(np.float32).mean(axis=1)
    k = 8
    d = mono[::k]
    sr2 = sr / k
    n = len(d)
    seg = int(12 * sr2)
    x = d[n - seg:]
    c = corr_search(d[:n - 1], x)           # position t matches the end when c[t] is high
    lags = (n - seg) - np.arange(len(c))     # samples of d between the match and the segment
    ok = lags > 3 * sr2
    if not ok.any():
        return None
    cm = np.where(ok, c, -1.0)
    best = float(cm.max())
    if best < 0.5:
        return None
    # the smallest lag whose correlation is close to the best one is the period
    good = np.nonzero(cm >= max(0.7 * best, 0.45))[0]
    t = int(good[-1])                         # the latest of those = the smallest lag
    lag = (n - seg) - t
    # refine at the full rate around lag * k
    approx = lag * k
    xs = mono[len(mono) - seg * k:]
    best_l, best_c = approx, -2
    for l in range(approx - 4 * k, approx + 4 * k + 1):
        s = mono[len(mono) - seg * k - l: len(mono) - l]
        if len(s) != len(xs):
            continue
        cc = float(np.dot(s, xs) / np.sqrt(np.dot(s, s) * np.dot(xs, xs) + 1e-9))
        if cc > best_c:
            best_c, best_l = cc, l
    period = best_l
    # where does the repetition begin? first window that equals the one one period later
    w = int(4 * sr)
    start = 0
    step = int(0.25 * sr)
    while start + period + w <= len(mono):
        a = mono[start:start + w]
        b = mono[start + period:start + period + w]
        cc = float(np.dot(a, b) / np.sqrt(np.dot(a, a) * np.dot(b, b) + 1e-9))
        if cc > 0.6:
            break
        start += step
    # a loop point on a quiet moment makes a gentler seam: look around `start` for the smallest amplitude
    env = np.abs(mono[start: start + 2 * step])
    if len(env) > 64:
        smooth = np.convolve(env, np.ones(64) / 64, mode='same')
        start += int(np.argmin(smooth[32:-32])) + 32
    return start, start + period


def render_sfx(engine, sid, tmp):
    wav = Path(tmp) / ('s%02X.wav' % sid)
    dump(engine, sid, 4, wav)
    if not wav.exists():
        return False
    sr, st = read_wav(wav)
    st = trim_end(st, sr)
    write_flac(st, sr, OUT / ('sfx_%02X.flac' % sid))
    return True


def render_music(engine, sid, tmp):
    wav = Path(tmp) / ('m%02X.wav' % sid)
    dump(engine, sid, 45, wav)
    if not wav.exists():
        return False
    sr, st = read_wav(wav)
    mono_tail = st[-int(5 * sr):]
    if rms(mono_tail) < 20.0:                  # a jingle: it ended
        st = trim_end(st, sr)
        write_ogg(st, sr, OUT / ('music_%02X.ogg' % sid))
        (OUT / ('music_%02X.json' % sid)).write_text(json.dumps({'sample_rate': sr, 'loop': False}))
        return True
    dump(engine, sid, 240, wav)
    sr, st = read_wav(wav)
    lp = find_loop(st, sr)
    if lp is None:
        print('  no loop found for %#x: the first 120 s are used, looping the end' % sid)
        st = st[: int(120 * sr)]
        lp = (int(60 * sr), len(st))
    s, e = lp
    st = st[:e]
    write_ogg(st, sr, OUT / ('music_%02X.ogg' % sid), quality=3)
    (OUT / ('music_%02X.json' % sid)).write_text(json.dumps({'sample_rate': sr, 'loop': True, 'loop_start': int(s), 'loop_end': int(e)}))
    print('  loop %.2f s -> %.2f s' % (s / sr, e / sr))
    return True


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--engine', default=str(ENGINE))
    ap.add_argument('ids', nargs='*')
    a = ap.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    ids = [int(i, 16) for i in a.ids] if a.ids else MUSIC + SFX
    with tempfile.TemporaryDirectory() as tmp:
        for sid in ids:
            music = sid in MUSIC
            print('%s %#04x' % ('music' if music else 'sfx  ', sid), flush=True)
            ok = render_music(a.engine, sid, tmp) if music else render_sfx(a.engine, sid, tmp)
            if not ok:
                print('  failed')


if __name__ == '__main__':
    sys.exit(main())
