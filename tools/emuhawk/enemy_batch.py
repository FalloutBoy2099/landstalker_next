"""Records the enemies of the original (debug-enabled copy of the ROM, warp.lua) in a list of rooms while Nigel walks about.
  python enemy_batch.py STATE OUTDIR ROOM[,ROOM...]   (ROM copy with DebugModeEnable patched: see README)"""
import subprocess, sys, os
here = os.path.dirname(os.path.abspath(__file__))
state, outdir, rooms = sys.argv[1], sys.argv[2], sys.argv[3].split(',')
rom = os.path.join(os.path.dirname(state), '..', 'ls_debug.md')
cols = ['g_RmNum1:w']
for i in range(12):
    b = 0xFF5400 + 0x80 * i
    for off, size in [(0, ''), (1, ''), (2, ''), (3, ''), (0x12, ':w'), (0x3B, ''), (9, ''), (8, ''), (0x3E, ''), (0xC, '')]:
        cols.append('0x%X%s' % (b + off, size))
steps = []
dirs = ['Up', 'Left', 'Down', 'Right']
f, d = 0, 0
while f < 1500:
    steps.append('%d:+%s' % (f, dirs[d % 4]))
    steps.append('%d:-%s' % (f + 70, dirs[d % 4]))
    f += 80
    d += 1
for r in rooms:
    out = os.path.join(outdir, r)
    subprocess.run([sys.executable, os.path.join(here, 'run.py'), '--script', 'warp.lua', '--warp', r, '--rom', os.path.abspath(rom), '--out', out, '--state', state,
                    '--end', '1500', '--input', ';'.join(steps), '--log', ','.join(cols), '--shot', '200,600'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    print('room', r, 'done', flush=True)
