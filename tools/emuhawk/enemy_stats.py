"""Summarises what enemy_batch.py recorded: per enemy type, the speed byte and how far an enemy moves per frame (1/16 cell)."""
import csv, sys, os, collections
outdir = sys.argv[1]
summary = collections.defaultdict(lambda: collections.defaultdict(collections.Counter))
for room in sorted(os.listdir(outdir), key=lambda s: int(s) if s.isdigit() else 0):
    p = os.path.join(outdir, room, 'log.csv')
    if not os.path.exists(p):
        continue
    rows = list(csv.DictReader(open(p)))
    if len(rows) < 100:
        continue
    for i in range(1, 12):
        b = 0xFF5400 + 0x80 * i
        key = lambda off, sz='': '0x%X%s' % (b + off, sz)
        prev = None
        for r in rows[150:]:
            t = int(r[key(0x3B)])
            if t == 0:
                continue
            x, y = int(r[key(0)]) * 16 + int(r[key(2)]), int(r[key(1)]) * 16 + int(r[key(3)])
            if prev is not None and prev[2] == t:
                d = abs(x - prev[0]) + abs(y - prev[1])
                if d < 48:
                    summary[t][('speed', int(r[key(9)]))][d] += 1
            prev = (x, y, t)
for t, v in sorted(summary.items()):
    for (kind, sp), c in sorted(v.items()):
        tot = sum(c.values())
        moving = {d: n for d, n in c.items() if d}
        print('type', t, 'speed byte', sp, 'frames', tot, 'still %.0f%%' % (100 * c[0] / tot), 'steps(1/16 cell/frame):', dict(sorted(moving.items(), key=lambda kv: -kv[1])[:5]))
