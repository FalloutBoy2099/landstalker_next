"""Runs the original ROM in EmuHawk under hawk.lua.
  python run.py --out DIR [--end N] [--input "60:+Start;70:-Start"] [--shot 300,600] [--log g_RmNum1:w,Player_X] [--every 1]
                [--state FILE] [--save "300=start"] [--rom PATH]"""
import argparse, os, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
ap = argparse.ArgumentParser()
ap.add_argument('--out', required=True)
ap.add_argument('--end', default='600')
ap.add_argument('--input', default='')
ap.add_argument('--shot', default='')
ap.add_argument('--log', default='')
ap.add_argument('--every', default='1')
ap.add_argument('--state', default='')
ap.add_argument('--save', default='')
ap.add_argument('--rom', default=r'D:\Git\Landstalker\roms\landstalker_us.md')
ap.add_argument('--hawk', default=r'D:\Roms\Hawk\EmuHawk.exe')
ap.add_argument('--script', default='hawk.lua', help='the Lua driver (hawk.lua, warp.lua)')
ap.add_argument('--warp', default='', help='warp.lua: room to warp to')
ap.add_argument('--poke', default='', help='warp.lua: RAM bytes to write after the warp, Name=value,...')
ap.add_argument('--visible', action='store_true', help='do not minimize EmuHawk')
a = ap.parse_args()
os.makedirs(a.out, exist_ok=True)
env = dict(os.environ, LS_HAWK_OUT=os.path.abspath(a.out), LS_HAWK_END=a.end, LS_HAWK_INPUT=a.input, LS_HAWK_SHOT=a.shot,
           LS_HAWK_LOG=a.log, LS_HAWK_EVERY=a.every, LS_HAWK_STATE=a.state, LS_HAWK_SAVE=a.save, LS_HAWK_DIR=here + os.sep, LS_WARP_ROOM=a.warp, LS_POKE=a.poke)
with open(os.path.join(a.out, 'hawk.log'), 'w') as log:
    # minimized and without taking the focus (SW_SHOWMINNOACTIVE), so the runs do not get in the way
    si = subprocess.STARTUPINFO()
    si.dwFlags |= subprocess.STARTF_USESHOWWINDOW
    si.wShowWindow = 7
    r = subprocess.run([a.hawk, '--lua=' + os.path.join(here, a.script), a.rom], env=env, stdout=log, stderr=subprocess.STDOUT, timeout=900, startupinfo=None if a.visible else si)
sys.exit(r.returncode)
