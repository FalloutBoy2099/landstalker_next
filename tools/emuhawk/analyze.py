"""Reads a log.csv of hawk.lua made with entity columns (see entcols) and prints what each entity does, when it changes."""
import csv, sys
def load(path):
    return list(csv.DictReader(open(path)))
def ents(row, n=12):
    out = []
    for i in range(n):
        b = 0xFF5400 + 0x80 * i
        g = lambda off: int(row['0x%X' % (b + off)])
        out.append(dict(i=i, x=g(0), y=g(1), sx=g(2), sy=g(3), z=g(0x12), gfx=g(0xB), f1=g(8), f2=g(0xC), t=g(0x3B)))
    return out
if __name__ == '__main__':
    rows = load(sys.argv[1]); lo = int(sys.argv[2]); hi = int(sys.argv[3]); step = int(sys.argv[4]) if len(sys.argv) > 4 else 10
    last = {}
    for r in rows:
        f = int(r['frame'])
        if f < lo or f > hi: continue
        for e in ents(r):
            key = (e['x'], e['y'], e['z'], e['gfx'], e['f1'], e['t'])
            if e['i'] and e['t'] == 0 and e['gfx'] == 0 and e['x'] == 0: continue
            if last.get(e['i']) != key and f % step == 0:
                print(f, 'room', r['g_RmNum1'], 'e%d' % e['i'], 'pos %d.%02x,%d.%02x z%04x' % (e['x'], e['sx'], e['y'], e['sy'], e['z']), 'gfx', e['gfx'], 'f1 %02x f2 %02x type %d' % (e['f1'], e['f2'], e['t']), 'str', r['g_IntroStringToDisplay'])
                last[e['i']] = key


def entcols(n=12):
    """The --log argument that records n entity slots (x y subx suby z gfx flags1 flags2 type rotation anim frame) plus the camera and scene state."""
    cols = ['g_RmNum1:w', 'g_IntroStringToDisplay', 'g_ControllerPlayback', 'g_PlayerSpeed:w', 'vsramA', '0xFF0100:w', 'g_PaletteDarkenCurrentBrightness:w', 'g_IntroStringDisplayTime:w', 'g_LockPlayerActions']
    for i in range(n):
        b = 0xFF5400 + 0x80 * i
        for off, size in [(0, ''), (1, ''), (2, ''), (3, ''), (0x12, ':w'), (0xB, ''), (6, ':w'), (8, ''), (0xC, ''), (0x3B, ''), (4, ''), (0x24, ''), (0x26, '')]:
            cols.append('0x%X%s' % (b + off, size))
    return ','.join(cols)
