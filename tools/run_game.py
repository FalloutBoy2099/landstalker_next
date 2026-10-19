"""Runs the game for a test without taking the screen and the focus (minimized, SW_SHOWMINNOACTIVE), like tools/emuhawk/run.py does for the emulator.

    python tools/run_game.py [--timeout SECONDS] [--visible] [--out FILE] [game arguments]      (the LS_* variables of the environment are passed on)

The output of the game (its console) goes to --out (default: the output of this script, once the game has ended).
"""
import argparse
import os
import subprocess
import sys

ap = argparse.ArgumentParser()
ap.add_argument('--timeout', type=float, default=60)
ap.add_argument('--visible', action='store_true', help='normal window')
ap.add_argument('--out', default='')
ap.add_argument('args', nargs='*')
a = ap.parse_args()
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
exe = os.path.join(root, 'target', 'release', 'landstalker.exe')
si = None
if not a.visible:
    si = subprocess.STARTUPINFO()
    si.dwFlags |= subprocess.STARTF_USESHOWWINDOW
    si.wShowWindow = 7  # SW_SHOWMINNOACTIVE
env = dict(os.environ)
env.setdefault('LS_KEYBOARD', 'physical')   # the tests press keys by place (the autoplay and the bot do not type letters)
if not a.visible:
    env['LS_MINIMIZED'] = '1'   # the game minimizes its window itself (the start-up flags of Windows are not enough: the window is shown normally)
out = open(a.out, 'w', encoding='utf8', errors='replace') if a.out else subprocess.PIPE
try:
    r = subprocess.run([exe] + a.args, cwd=root, stdout=out, stderr=subprocess.STDOUT, startupinfo=si, timeout=a.timeout, env=env)
    if not a.out and r.stdout:
        sys.stdout.write(r.stdout.decode('utf8', 'replace'))
except subprocess.TimeoutExpired as e:
    if not a.out and e.stdout:
        sys.stdout.write(e.stdout.decode('utf8', 'replace'))
