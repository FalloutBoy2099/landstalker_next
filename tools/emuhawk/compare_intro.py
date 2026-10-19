"""Compares the port's pictures of the intro (p_<track frame>.png, LS_TRACK_SHOTS) with the original's (shot_<abs frame>.png, 320x224):
the port's game area is 1280x960 at scale 4 (the picture is its top-left 1280x896). Prints the difference per frame and writes a sheet of the worst.
  python compare_intro.py PORT_DIR ORIGINAL_ROOT POINTS.json OUT.png [N]"""
import json, os, sys
from PIL import Image, ImageChops, ImageDraw, ImageFilter
port, root, points, out = sys.argv[1:5]
n = int(sys.argv[5]) if len(sys.argv) > 5 else 8
pts = json.load(open(points))
rows = []
for a in pts['abs']:
    p = os.path.join(port, 'p_%d.png' % (a - 218))
    o = os.path.join(root, pts['src'][str(a)], 'shot_%d.png' % a)
    if not (os.path.exists(p) and os.path.exists(o)):
        continue
    pi = Image.open(p).convert('RGB').crop((0, 0, 1280, 896)).resize((320, 224), Image.NEAREST)
    oi = Image.open(o).convert('RGB')
    # blurred: a pixel of dither or a pixel of shift is not a difference
    d = ImageChops.difference(pi.filter(ImageFilter.GaussianBlur(2)), oi.filter(ImageFilter.GaussianBlur(2))).convert('L')
    score = sum(1 for v in d.getdata() if v > 30) / (320 * 224)
    rows.append((score, a, pi, oi))
rows.sort(key=lambda r: -r[0])
for s, a, _, _ in sorted(rows, key=lambda r: r[1]):
    print('abs %5d (track %5d): %5.1f%% different' % (a, a - 218, s * 100))
sheet = Image.new('RGB', (640 * 2, 224 * 2 * min(n, len(rows))))
for i, (s, a, pi, oi) in enumerate(rows[:n]):
    sheet.paste(oi.resize((640, 448), Image.NEAREST), (0, i * 448))
    sheet.paste(pi.resize((640, 448), Image.NEAREST), (640, i * 448))
    ImageDraw.Draw(sheet).text((4, i * 448 + 4), 'original abs %d (%.0f%% diff)   | port' % (a, s * 100), fill=(255, 255, 0))
sheet.save(out)
