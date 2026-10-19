#!/usr/bin/env python3
"""Finds the pictures of the graphic effects in the game data and writes data/effects.yaml and the lights of the rooms.

For now one effect, `flame`: the tiles (the 16x16 blocks of a room's blocks.png and of its animations anim_N.png) and the entity types whose pictures hold
flames, found by their colours: bright, saturated reds / oranges / yellows spread over a range of hues (the console's palette has few, and the flames of
the original are red at the edge and yellow at the heart).

The lights themselves are written in each room's map.tmx, in an object layer named `lights` that Tiled shows and lets you edit: one point object per
flame (`type="light"`) with the custom properties `light_type` (flame), `color`, `intensity` (1 = an ordinary flame) and `flicker` (0..1). The game builds
a point light with a flicker from each of them. The x and y of an object are the position of the flame in the block grid of the room, in the map units
of Tiled (16 a block, fractions allowed); moving the object in Tiled moves the light.

Usage: detect_effects.py [--room N] [data dir]

  --room N   only that room: its lights are written in its map.tmx, nothing else is touched (to try a room out)

Without --room the parameters of each effect (the light it casts...) are kept when effects.yaml exists: only the lists of pictures are rewritten, and the
object layer `lights` of the maps (what you changed in it is lost: edit the maps after the last run of the tool).
"""
import re
import sys
from pathlib import Path

import numpy as np
import yaml
from PIL import Image

MIN_PIXELS = 9     # bright flame pixels in a picture
MIN_SPREAD = 15    # degrees of hue between the reddest and the yellowest of them


def flame_mask(rgba):
    """Pixels that look like fire: hue up to 56 degrees (red to yellow), saturated, bright (the console's colours go up to 216 of 255)."""
    a = rgba[..., 3] >= 128
    rgb = rgba[..., :3].astype(np.float32) / 255.0
    mx, mn = rgb.max(-1), rgb.min(-1)
    d = mx - mn
    with np.errstate(divide='ignore', invalid='ignore'):
        h = np.where(mx == rgb[..., 0], ((rgb[..., 1] - rgb[..., 2]) / d) % 6, np.where(mx == rgb[..., 1], (rgb[..., 2] - rgb[..., 0]) / d + 2, (rgb[..., 0] - rgb[..., 1]) / d + 4)) * 60.0
        s = np.where(mx > 0, d / mx, 0)
    return a & (d > 0) & (h <= 56) & (s > 0.8) & (mx >= 0.8), h


def is_flame(rgba, share=0.2, grass=True):
    """Flames in a picture: enough fire pixels over a range of hues, which are at least `share` of the picture's opaque pixels, and (a tile) not
    a picture that is mostly green (the flowers of a meadow have the same colours)."""
    mask, h = flame_mask(rgba)
    n = int(mask.sum())
    opaque = max(int((rgba[..., 3] >= 128).sum()), 1)
    if n < MIN_PIXELS or n / opaque < share:
        return False
    if grass:
        rgb = rgba[..., :3].astype(np.int32)
        green = (rgba[..., 3] >= 128) & (rgb[..., 1] > rgb[..., 0] + 20) & (rgb[..., 1] > rgb[..., 2] + 20)
        if green.sum() / opaque >= 0.2:
            return False
    hs = h[mask]
    return float(hs.max() - hs.min()) >= MIN_SPREAD


def flame_heart(tile):
    """Where the flame is in a 16x16 tile: the middle of its bright pixels (weighted by how bright they are, so the yellow heart counts more than the red
    edge) and how many of them there are; None if the tile holds no flame."""
    if not is_flame(tile):
        return None
    mask, _ = flame_mask(tile)
    ys, xs = np.nonzero(mask)
    weight = tile[ys, xs, :3].astype(np.float32).sum(axis=1) + 1.0
    # the yellow heart: the pixels with the most green
    weight = weight * (1.0 + tile[ys, xs, 1].astype(np.float32) / 128.0)
    return (float((xs * weight).sum() / weight.sum()) + 0.5, float((ys * weight).sum() / weight.sum()) + 0.5, int(mask.sum()))


def tiles_of(image, columns):
    """Flame tiles of a strip / atlas of 16x16 blocks, `columns` of them a row: block number -> [(x, y, pixels)]."""
    rgba = np.array(image.convert('RGBA'))
    rows = rgba.shape[0] // 16
    out = {}
    for i in range(columns * rows):
        heart = flame_heart(rgba[(i // columns) * 16:(i // columns) * 16 + 16, (i % columns) * 16:(i % columns) * 16 + 16])
        if heart:
            out[i] = [heart]
    return out


# What each kind of place looks like (the parameters are kept when the file exists). ambient: light left in the room (1: all), tint: colour of the
# ambient light, light_strength / light_radius: what the flames do (radius in cells), vignette and bloom: strengths, shadows: how far the dark parts are pushed to
# blue / cyan, warmth: how warm the highlights are, contrast and saturation.
ENVIRONMENTS = {
    'outdoor': {'ambient': 1.0, 'tint': [1.0, 1.0, 1.0], 'light_strength': 0.1, 'light_radius': 2.4, 'vignette': 0.05, 'bloom': 0.12, 'shadows': 0.0, 'warmth': 0.0, 'contrast': 1.0, 'saturation': 1.0},
    'interior': {'ambient': 0.84, 'tint': [1.0, 0.88, 0.72], 'light_strength': 0.2, 'light_radius': 4.0, 'vignette': 0.45, 'bloom': 0.5, 'shadows': 0.12, 'warmth': 0.08, 'contrast': 1.04, 'saturation': 0.96, 'night': [16, 20, 32], 'night_amount': 0.22},
    'cave': {'ambient': 0.92, 'tint': [1.0, 0.85, 0.68], 'light_strength': 0.22, 'light_radius': 5.0, 'vignette': 0.45, 'bloom': 0.55, 'shadows': 0.12, 'warmth': 0.05, 'contrast': 1.08, 'saturation': 0.8},
}

# Which kind of place a room is: the first rule with a word in the room's path (lower case, the folders of the game's tree and its name) wins; a room that
# no rule picks is `interior`. `rooms` in the file overrides the rules room by room (room number -> kind).
RULES = [
    {'env': 'outdoor', 'path': ['outside', "kado's house/"]},
    {'env': 'cave', 'path': ["helga's hut/"]},
    {'env': 'interior', 'path': ['/interior/', 'shop', 'house', 'hut', 'church', 'inn', 'bedroom', 'lighthouse', 'kitchen', 'bunk']},
    {'env': 'cave', 'path': ['massan secret cave', 'destel well/', 'waterfall shrine/', 'swamp shrine/', 'lake shrine/', 'labyrinth/', "king nole's cave/", "king nole's palace/", "thieves' hideout/", 'verla mine/',
                             'mercator crypt/', 'kazalt/', 'underground', 'passageway', 'secret path', 'tunnel', 'mines']},
    {'env': 'outdoor', 'path': ['overworld/', 'mountainous', 'greenmaze', 'tree warps', 'mercator/', 'ryuma/', 'verla/', 'gumi/', 'massan/', 'destel/', 'tibor/']},
]


def block_grid(room, tmx_text):
    """Where the tiles are placed in the map: [(block number, bx, by, layer shift)], by layer (the same as `Room::block` and `spawn_room` of the game)."""
    info = room['map']
    origin, grid = info['blocks']['origin'], info['grid']['origin']
    width = info['grid']['size'][0]
    first = int(re.search(r'<tileset firstgid="(\d+)" name="blocks"', tmx_text).group(1))
    out = []
    for m in re.finditer(r'<layer id="\d+" name="(\w+)"[^>]*>\s*<data encoding="csv">\s*([\d,\s]+?)\s*</data>', tmx_text):
        name = m.group(1)
        if name not in ('background', 'foreground'):
            continue
        shift = 16.0 if name == 'background' else 0.0
        values = [int(v) for v in m.group(2).replace('\n', '').split(',') if v.strip()]
        for i, gid in enumerate(values):
            if gid:
                gx, gy = i % width, i // width
                out.append((gid - first, gx - origin[0] + grid[0], gy - origin[1] + grid[1], shift))
    return out


def merge_lights(points):
    """The tiles of one flame (a tall flame on two blocks) are made one light: the points within 11 pixels, weighted by the size of their flames."""
    points = sorted(points, key=lambda p: -p[2])
    merged = []
    for x, y, n in points:
        for m in merged:
            if ((m[0] - x) ** 2 + (m[1] - y) ** 2) ** 0.5 < 11.0:
                total = m[2] + n
                m[0], m[1], m[2] = (m[0] * m[2] + x * n) / total, (m[1] * m[2] + y * n) / total, total
                break
        else:
            merged.append([x, y, n])
    return merged


def write_lights(room_dir, room, hearts, light):
    """Writes the object layer `lights` of the room's map.tmx (replaces the one that was there). Returns how many lights."""
    tmx_path = room_dir / 'map.tmx'
    text = tmx_path.read_text(encoding='utf-8')
    text = re.sub(r'\n <objectgroup [^>]*name="lights".*?</objectgroup>', '', text, flags=re.S)
    block_h = room['map']['blocks']['size'][1]
    left, top = room['map']['legacy']['heightmap_left'], room['map']['legacy']['heightmap_top']
    points = []
    for block, bx, by, shift in block_grid(room, text):
        for h in hearts.get(block, []):
            # the screen pixel of the heart of the flame (spawn_room puts the top left of a block there)
            px = ((bx - by + block_h - 1) * 2 + left) * 8.0 + shift
            py = (bx + by + top) * 8.0
            points.append((px + h[0], py + h[1], h[2]))
    lights = merge_lights(points)
    layer_match = re.search(r'nextlayerid="(\d+)"', text)
    object_match = re.search(r'nextobjectid="(\d+)"', text)
    if not lights or not layer_match or not object_match:
        tmx_path.write_text(text, encoding='utf-8')
        return 0
    layer_id, object_id = int(layer_match.group(1)), int(object_match.group(1))
    color = '#%02x%02x%02x' % tuple(light['color'])
    rows = []
    for k, (x, y, n) in enumerate(lights):
        # the screen point as a position in the block grid: the inverse of the formula above (16 map units a block)
        u, v = x / 8.0 - left, y / 8.0 - top
        bx = (v + u / 2.0 - block_h + 1) / 2.0
        by = (v - u / 2.0 + block_h - 1) / 2.0
        intensity = min(max((n / 40.0) ** 0.5, 0.6), 1.5)
        rows.append(
            '  <object id="%d" name="flame" type="light" x="%.2f" y="%.2f">\n   <point/>\n   <properties>\n'
            '    <property name="light_type" value="flame"/>\n    <property name="color" type="color" value="%s"/>\n'
            '    <property name="intensity" type="float" value="%.2f"/>\n    <property name="flicker" type="float" value="%.2f"/>\n   </properties>\n  </object>'
            % (object_id + k, bx * 16.0, by * 16.0, color, intensity, light['flicker']))
    group = '\n <objectgroup id="%d" name="lights">\n%s\n </objectgroup>' % (layer_id, '\n'.join(rows))
    text = text.replace('nextlayerid="%d"' % layer_id, 'nextlayerid="%d"' % (layer_id + 1)).replace('nextobjectid="%d"' % object_id, 'nextobjectid="%d"' % (object_id + len(lights)))
    text = text.replace('\n</map>', group + '\n</map>')
    tmx_path.write_text(text, encoding='utf-8')
    return len(lights)


def hearts_of_room(room_dir):
    """block -> [x, y, size] of the flame tiles of a room (the mean position over the pictures of a tile, the most pixels of one picture)."""
    found = {}     # block -> the hearts of its flame in each picture
    blocks = room_dir / 'blocks.png'
    if blocks.exists():
        for b, hearts in tiles_of(Image.open(blocks), 32).items():
            found.setdefault(b, []).extend(hearts)
    info = yaml.safe_load((room_dir / 'room.yaml').read_text(encoding='utf-8')) if (room_dir / 'room.yaml').exists() else {}
    # the animated blocks: every picture of their animation (a flame is only a flame in some of its frames)
    for an in (info or {}).get('animations') or []:
        strip = room_dir / an['file']
        if not strip.exists():
            continue
        img = np.array(Image.open(strip).convert('RGBA'))
        for u, b in enumerate(an['blocks']):
            for f in range(an['frames']):
                heart = flame_heart(img[f * 16:f * 16 + 16, u * 16:u * 16 + 16])
                if heart:
                    found.setdefault(b, []).append(heart)
    hearts = {b: [round(sum(h[0] for h in hs) / len(hs), 1), round(sum(h[1] for h in hs) / len(hs), 1), max(h[2] for h in hs)] for b, hs in sorted(found.items())}
    return hearts, info


def main():
    args = sys.argv[1:]
    only = None
    if '--room' in args:
        only = int(args[args.index('--room') + 1])
        del args[args.index('--room'):args.index('--room') + 2]
    data = Path(args[0]) if args else Path(__file__).resolve().parent.parent / 'data'
    out = data / 'effects.yaml'
    old = yaml.safe_load(out.read_text(encoding='utf-8')) if out.exists() else {}
    flame = dict((old or {}).get('flame') or {})
    flame.setdefault('light', {'color': [255, 119, 0], 'strength': 0.6, 'flicker': 0.12, 'speed': 7.0})
    if only is not None:
        room_dir = data / 'rooms' / ('room_%03d' % only)
        hearts, info = hearts_of_room(room_dir)
        n = write_lights(room_dir, info, {b: [tuple(v)] for b, v in hearts.items()}, flame['light']) if hearts else 0
        print('room %d: %d lights written in %s' % (only, n, room_dir / 'map.tmx'))
        return
    rooms = {}
    lights = 0
    for room_dir in sorted((data / 'rooms').glob('room_*')):
        hearts, info = hearts_of_room(room_dir)
        if hearts:
            rooms[int(room_dir.name[5:])] = hearts
            if (room_dir / 'map.tmx').exists() and info:
                lights += write_lights(room_dir, info, {b: [tuple(v)] for b, v in hearts.items()}, flame['light'])
    # the entity types
    entities = {}
    for ent_dir in sorted((data / 'sprites' / 'entities').glob('entity_*')):
        # (the types from 192 on are the objects lying about: their pictures are icons, a red sword is not a flame)
        if not ent_dir.is_dir() or int(ent_dir.name[7:]) >= 192:
            continue
        pictures = [str(p.relative_to(data / 'sprites' / 'entities')).replace('\\', '/') for p in ent_dir.rglob('*.png') if is_flame(np.array(Image.open(p).convert('RGBA')), share=0.3, grass=False)]
        if pictures:
            entities[int(ent_dir.name[7:])] = sorted(pictures)
    flame['tiles'] = rooms
    flame['entities'] = entities
    header = (
        '# The graphic effects (landstalker.ini, [effects]): the pictures each one works on, found by tools/detect_effects.py, and its parameters.\n'
        '# flame:  tiles: room -> block (16x16, numbered as in the room\'s blocks.png) -> [x, y, size]: where the flame is in the tile and how many pixels it has;\n'
        '#         entities: entity type -> its flame pictures. The lights of a room are the objects of the layer `lights` of its map.tmx (written by the tool).\n'
        '#         light: what a flame lights (colour, strength, flicker in 0..1, flicker speed in Hz).\n'
        '# environments, rules, rooms: how each kind of place (outdoor, interior, cave) is lit and graded, and which kind each room is (see the tool).\n'
    )
    flame['light'].pop('radius', None)    # (the reach is per kind of place)
    tree = {'environments': (old or {}).get('environments') or ENVIRONMENTS, 'rules': (old or {}).get('rules') or RULES, 'rooms': (old or {}).get('rooms') or {}, 'flame': flame}
    text = yaml.safe_dump(tree, sort_keys=False, default_flow_style=None, width=160)
    out.write_text(header + text, encoding='utf-8')
    print('%d rooms with flame tiles (%d tiles), %d lights in the maps, %d entity types -> %s' % (len(rooms), sum(len(v) for v in rooms.values()), lights, len(entities), out))


if __name__ == '__main__':
    main()
