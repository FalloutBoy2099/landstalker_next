"""Tiles screenshots (shot_<frame>.png) of one or two runs into a contact sheet with the frame numbers.
  python montage.py OUT.png DIR_A[,DIR_B] first last step [cols] [scale]"""
import sys, os
from PIL import Image, ImageDraw
out, dirs = sys.argv[1], sys.argv[2].split(',')
lo, hi, step = int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
cols = int(sys.argv[6]) if len(sys.argv) > 6 else 4
scale = float(sys.argv[7]) if len(sys.argv) > 7 else 1.0
tiles = []
for f in range(lo, hi + 1, step):
    for d in dirs:
        p = os.path.join(d, 'shot_%d.png' % f)
        if os.path.exists(p):
            im = Image.open(p).convert('RGB')
            im = im.resize((int(im.width * scale), int(im.height * scale)), Image.NEAREST)
            ImageDraw.Draw(im).text((2, 2), '%s %d' % (os.path.basename(os.path.normpath(d)), f), fill=(255, 255, 0))
            tiles.append(im)
if not tiles: sys.exit('no shots')
w, h = tiles[0].size
rows = (len(tiles) + cols - 1) // cols
sheet = Image.new('RGB', (w * cols, h * rows))
for i, t in enumerate(tiles):
    sheet.paste(t, ((i % cols) * w, (i // cols) * h))
sheet.save(out)
