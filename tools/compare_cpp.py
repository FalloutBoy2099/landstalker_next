#!/usr/bin/env python3
"""Compares the C++ engine and the Bevy game on the same room / position (Nigel standing there).

Usage: compare_cpp.py ROOM X Y [-o out.png]
Both are started with their own screenshot hook, the Bevy picture is registered on the C++ one (template match on the
background around Nigel) and written side by side with a difference map (C++ | Bevy | difference).
"""
import argparse
import os
import subprocess
import sys
import tempfile

import cv2
import numpy as np

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CPP_DIR = os.path.join(os.path.dirname(ROOT), 'landstalker_engine', 'build', 'RelWithDebInfo')
BEVY = os.path.join(ROOT, 'target', 'release', 'landstalker.exe')


def shot_cpp(room, x, y, path):
    env = dict(os.environ, LS_NOINTRO='1', LS_NOSOUND='1', LS_SCREENSHOT=path, LS_SCREENSHOT_FRAME='10')
    subprocess.run([os.path.join(CPP_DIR, 'dds520.exe'), str(room), str(x), str(y)], cwd=CPP_DIR, env=env, timeout=60,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return cv2.imread(path)


def shot_bevy(room, x, y, path):
    env = dict(os.environ, LS_SHOT=path)
    subprocess.run([BEVY, str(room), str(x), str(y)], cwd=ROOT, env=env, timeout=60, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return cv2.imread(path)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('room', type=int)
    ap.add_argument('x', type=float)
    ap.add_argument('y', type=float)
    ap.add_argument('-o', '--out')
    ap.add_argument('--half', type=int, default=150, help='half size of the compared area in screen pixels')
    a = ap.parse_args()
    tmp = tempfile.mkdtemp()
    cpp = shot_cpp(a.room, a.x, a.y, os.path.join(tmp, 'cpp.bmp'))
    bevy = shot_bevy(a.room, a.x, a.y, os.path.join(tmp, 'bevy.png'))
    if cpp is None or bevy is None:
        print('a screenshot is missing')
        return 1
    # Bevy: Nigel at about (640, 480); C++: find where the same background is
    bx, by = 640, 480
    h = a.half
    tpl = bevy[by - h:by + h, bx - h:bx + h]
    res = cv2.matchTemplate(cpp, tpl, cv2.TM_CCOEFF_NORMED)
    _, score, _, loc = cv2.minMaxLoc(res)
    ox, oy = loc[0] - (bx - h), loc[1] - (by - h)   # C++ = Bevy shifted by (ox, oy)
    print('registration score %.3f offset (%d, %d)' % (score, ox, oy))
    c = cpp[by + oy - h:by + oy + h, bx + ox - h:bx + ox + h]
    b = bevy[by - h:by + h, bx - h:bx + h]
    diff = cv2.absdiff(c, b)
    mask = (diff.sum(axis=2) > 60).astype(np.uint8)
    print('pixels that differ: %d of %d (%.1f%%)' % (mask.sum(), mask.size, 100.0 * mask.sum() / mask.size))
    vis = np.zeros_like(c)
    vis[mask > 0] = (0, 0, 255)
    out = np.hstack([c, b, vis])
    path = a.out or os.path.join(ROOT, 'compare_%d_%s_%s.png' % (a.room, a.x, a.y))
    cv2.imwrite(path, out)
    print(path)
    return 0


if __name__ == '__main__':
    sys.exit(main())
