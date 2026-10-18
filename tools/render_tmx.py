#!/usr/bin/env python3
"""Renders the background + foreground layers of a converted room the way Tiled draws an isometric map
(right-down order, tile bottom-left on the cell's bottom-left, tileset offset applied): a check of map.tmx.
Usage: render_tmx.py <room dir> <out.png>"""
import sys
import xml.etree.ElementTree as ET
from pathlib import Path
from PIL import Image

room = Path(sys.argv[1])
root = ET.parse(room / 'map.tmx').getroot()
W, H = int(root.get('width')), int(root.get('height'))
TW, TH = int(root.get('tilewidth')), int(root.get('tileheight'))
blocks = Image.open(room / 'blocks.png').convert('RGBA')
cols = blocks.width // 16
canvas = Image.new('RGBA', ((W + H) * TW // 2 + 64, (W + H) * TH // 2 + 64), (20, 20, 30, 255))
for layer in root.findall('layer'):
    if layer.get('name') not in ('background', 'foreground'):
        continue
    data = [int(v) for v in layer.find('data').text.replace('\n', '').split(',')]
    shift = int(layer.get('offsetx', '0'))   # the background is 16 px to the right of the foreground
    for y in range(H):
        for x in range(W):
            gid = data[y * W + x]
            if gid == 0:
                continue
            i = gid - 1
            tile = blocks.crop(((i % cols) * 16, (i // cols) * 16, (i % cols) * 16 + 16, (i // cols) * 16 + 16))
            px = (x - y + H) * TW // 2 - TW // 2 + 32          # bottom-left of the cell + tileoffset x
            py = (x + y) * TH // 2 + TH - 16                  # bottom of the cell - tile height
            canvas.alpha_composite(tile, (px + shift, py))
canvas.save(sys.argv[2])
