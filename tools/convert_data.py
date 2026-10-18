#!/usr/bin/env python3
"""Converts the game files of the C++ engine (landstalker_engine/assets) into the data formats of landstalker_next.

Usage: convert_data.py [--engine-assets DIR] [--labels FILE] [--out DIR] [--rooms 18,175 | all]

Output (default landstalker_next/data):
  rooms/room_NNN/room.yaml      identity, tileset, music, entities, warps, flags, animations ... (see docs/data_formats.md)
  rooms/room_NNN/map.tmx        Tiled isometric map: layers background, foreground (blocks) and the heightmap split in
                                four editable layers (height, floor_type, cell_flags, cell_extra)
  rooms/room_NNN/blocks.png     16x16 blocks of the room (palette applied), blocks_priority.png, anim_N.png, entity_N_*.png,
                                preview.png (the whole room rendered), palette.yaml
  tilesets/*.png|tsx            the helper tilesets of the heightmap layers (shared by every room)
  sprites/, graphics/, text/    copied from the engine (entity images, fonts, dialogues, cutscenes, behaviours ...)
  index.yaml                    list of rooms with their names

The conversion is lossless for the maps (every heightmap cell value and block index is kept) and keeps the legacy
sections of room.yaml (entity_flags, tile_swaps, swap_flags, locked_doors, switches, chests ...) unchanged.
"""
import argparse
import csv
import shutil
import sys
from pathlib import Path

import yaml
from PIL import Image, ImageDraw

CELL_OFFSET = 12   # world cell = heightmap index + 12 (RoomView::CELL_OFFSET)
TW, TH = 32, 16    # isometric cell of the TMX (the engine steps 16 px / 8 px per cell)


def load_labels(path):
    try:
        d = yaml.safe_load(Path(path).read_text(encoding='utf-8'))
        return {int(k): v for k, v in d.get('rooms', {}).items()}, {int(k): v for k, v in d.get('entities', {}).items()}
    except Exception:
        return {}, {}


def read_csv(path, base):
    rows = []
    for line in Path(path).read_text().splitlines():
        line = line.strip()
        if line:
            rows.append([int(v, base) for v in line.split(',')])
    return rows


def tileset_images(out):
    """The helper tilesets: numbers on isometric diamonds. Returns {name: (columns, count)}."""
    tdir = out / 'tilesets'
    tdir.mkdir(parents=True, exist_ok=True)
    specs = {
        'height': (16, 8, (60, 120, 200)),        # 0..15
        'floor_type': (64, 8, (200, 120, 60)),    # 0..63
        'cell_flags': (16, 8, (190, 60, 60)),     # bits 12..15 of the cell: 1 ?, 2 npc blocked, 4 blocked, 8 ?
        'cell_extra': (4, 4, (120, 120, 120)),    # bits 6..7 of the cell
    }
    info = {}
    for name, (count, cols, colour) in specs.items():
        rows = (count + cols - 1) // cols
        img = Image.new('RGBA', (cols * TW, rows * TH), (0, 0, 0, 0))
        d = ImageDraw.Draw(img)
        for i in range(count):
            x, y = (i % cols) * TW, (i // cols) * TH
            d.polygon([(x + TW // 2, y), (x + TW - 1, y + TH // 2), (x + TW // 2, y + TH - 1), (x, y + TH // 2)],
                      fill=colour + (150,), outline=(255, 255, 255, 255))
            label = str(i)
            d.text((x + TW // 2 - 3 * len(label), y + TH // 2 - 5), label, fill=(255, 255, 255, 255))
        img.save(tdir / (name + '.png'))
        (tdir / (name + '.tsx')).write_text(
            '<?xml version="1.0" encoding="UTF-8"?>\n'
            '<tileset version="1.10" name="%s" tilewidth="%d" tileheight="%d" tilecount="%d" columns="%d">\n'
            ' <image source="%s.png" width="%d" height="%d"/>\n</tileset>\n' % (name, TW, TH, count, cols, name, cols * TW, rows * TH),
            encoding='utf-8')
        info[name] = (cols, count)
    return info


def csv_data(grid):
    return ',\n'.join(','.join(str(v) for v in row) for row in grid)


def convert_room(src, dst, number, labels, entity_labels, helper):
    meta = yaml.safe_load((src / 'room.yaml').read_text(encoding='utf-8'))
    room, mp = meta['room'], meta['map']
    bg = read_csv(src / 'map_bg.csv', 10)
    fg = read_csv(src / 'map_fg.csv', 10)
    hm = read_csv(src / 'heightmap.csv', 16)
    bw, bh = mp['width'], mp['height']
    hw, hh = mp['heightmap_width'], mp['heightmap_height']
    # Where block (0, 0) sits in heightmap index coordinates (see RoomView::DrawLayer / ToScreen: the art of a cell is
    # drawn half a cell to the right and up of the cell itself).
    box, boy = mp['heightmap_left'] - 13, mp['heightmap_top'] - 11
    gx0, gy0 = min(0, box), min(0, boy)
    gw, gh = max(hw, box + bw) - gx0, max(hh, boy + bh) - gy0

    def grid_of(values, ox, oy, default=0):
        g = [[default] * gw for _ in range(gh)]
        for y, row in enumerate(values):
            for x, v in enumerate(row):
                g[y + oy - gy0][x + ox - gx0] = v
        return g

    blocks_img = Image.open(src / 'blocks.png')
    bcols, brows = blocks_img.width // 16, blocks_img.height // 16
    layer_bg = grid_of([[v + 1 for v in r] for r in bg], box, boy)
    layer_fg = grid_of([[v + 1 for v in r] for r in fg], box, boy)
    first = {'blocks': 1}
    nxt = 1 + bcols * brows
    for name in ('height', 'floor_type', 'cell_flags', 'cell_extra'):
        first[name] = nxt
        nxt += helper[name][1]
    # heightmap cell: bits 12..15 flags (0x4000 closed to everybody, 0x2000 closed to NPCs), 8..11 height, 6..7 extra, 0..5 floor type
    layer_h = grid_of([[first['height'] + ((v >> 8) & 0xF) for v in r] for r in hm], 0, 0)
    layer_t = grid_of([[first['floor_type'] + (v & 0x3F) for v in r] for r in hm], 0, 0)
    layer_f = grid_of([[first['cell_flags'] + ((v >> 12) & 0xF) for v in r] for r in hm], 0, 0)
    layer_x = grid_of([[first['cell_extra'] + ((v >> 6) & 0x3) for v in r] for r in hm], 0, 0)

    dst.mkdir(parents=True, exist_ok=True)
    for name in ('blocks.png', 'blocks_priority.png', 'palette.yaml'):
        if (src / name).exists():
            shutil.copyfile(src / name, dst / name)
    if (src / 'room.png').exists():
        shutil.copyfile(src / 'room.png', dst / 'preview.png')
    for f in list(src.glob('anim_*.png')) + list(src.glob('entity_*.png')):
        shutil.copyfile(f, dst / f.name)

    def tsx_ref(name, fg_id):
        return ' <tileset firstgid="%d" source="../../tilesets/%s.tsx"/>\n' % (fg_id, name)

    name = labels.get(number, 'Room%03d' % number)
    tmx = ['<?xml version="1.0" encoding="UTF-8"?>\n',
           '<map version="1.10" tiledversion="1.10.2" orientation="isometric" renderorder="right-down" width="%d" height="%d" '
           'tilewidth="%d" tileheight="%d" infinite="0" nextlayerid="7" nextobjectid="1">\n' % (gw, gh, TW, TH),
           ' <properties>\n'
           '  <property name="room" type="int" value="%d"/>\n'
           '  <property name="grid_origin_x" type="int" value="%d"/>\n'
           '  <property name="grid_origin_y" type="int" value="%d"/>\n'
           '  <property name="cell_offset" type="int" value="%d"/>\n'
           ' </properties>\n' % (number, gx0, gy0, CELL_OFFSET),
           ' <tileset firstgid="1" name="blocks" tilewidth="16" tileheight="16" tilecount="%d" columns="%d">\n'
           '  <tileoffset x="32" y="0"/>\n'
           '  <image source="blocks.png" width="%d" height="%d"/>\n </tileset>\n' % (bcols * brows, bcols, blocks_img.width, blocks_img.height)]
    for n in ('height', 'floor_type', 'cell_flags', 'cell_extra'):
        tmx.append(tsx_ref(n, first[n]))
    for lid, (lname, grid, visible) in enumerate([('background', layer_bg, 1), ('foreground', layer_fg, 1), ('height', layer_h, 0),
                                                  ('floor_type', layer_t, 0), ('cell_flags', layer_f, 0), ('cell_extra', layer_x, 0)], 1):
        # The engine draws the background layer 2 grid units (16 px) to the right of the foreground (RoomView::DrawWorld);
        # the foreground and the heightmap cells are the reference.
        extra = ' offsetx="16"' if lname == 'background' else ''
        tmx.append(' <layer id="%d" name="%s" width="%d" height="%d"%s%s>\n  <data encoding="csv">\n%s\n  </data>\n </layer>\n'
                   % (lid, lname, gw, gh, extra, '' if visible else ' visible="0"', csv_data(grid)))
    tmx.append('</map>\n')
    (dst / 'map.tmx').write_text(''.join(tmx), encoding='utf-8')

    # --- room.yaml ---
    entities = []
    for i, e in enumerate(meta.get('entities') or []):
        ent = {'id': i, 'type': e['type'], 'type_name': entity_labels.get(e['type'], 'Entity%d' % e['type']),
               'position': e['position'], 'orientation': e['orientation'], 'speed': e['speed'], 'palette': e['palette'],
               'behaviour': e['behaviour'], 'script': None, 'dialogue': e['dialogue'] if e['talks'] else None,
               'flags': {'hostile': e['hostile'], 'solid': e['solid'], 'visible': e['visible'], 'gravity': e['gravity'],
                         'pickable': e.get('pickable', False), 'talks': e['talks']}}
        if 'images' in e:
            ent['images'] = e['images']
        entities.append(ent)
    warps = []
    for w in meta.get('warps') or []:
        if w['room1'] == number:
            warps.append({'kind': w['type'], 'area': [w['x1'], w['y1'], w['x_size'], w['y_size']], 'target_room': w['room2'],
                          'target_cell': [w['x2'], w['y2']]})
        if w['room2'] == number:
            warps.append({'kind': w['type'], 'area': [w['x2'], w['y2'], w['x_size'], w['y_size']], 'target_room': w['room1'],
                          'target_cell': [w['x1'], w['y1']]})
    out = {
        'id': number,
        'name': name.split('/')[-1],
        'path': name,
        'tileset': {'id': room['tileset'], 'palette': room['palette'], 'primary_blockset': room['primary_blockset'],
                    'secondary_blockset': room['secondary_blockset']},
        'music': room['bgm'],
        'heights': {'z_begin': room['z_begin'], 'z_end': room['z_end']},
        'fall_destination': room['fall_destination'] if room['fall_destination'] >= 0 else None,
        'climb_destination': room['climb_destination'] if room['climb_destination'] >= 0 else None,
        'map': {
            'file': 'map.tmx', 'preview': 'preview.png', 'palette': 'palette.yaml', 'block_size_px': 16,
            'cell_offset': CELL_OFFSET,
            'grid': {'origin': [gx0, gy0], 'size': [gw, gh]},
            'heightmap': {'origin': [0, 0], 'size': [hw, hh]},
            'blocks': {'origin': [box, boy], 'size': [bw, bh], 'tile_count': bcols * brows},
            'legacy': {'heightmap_left': mp['heightmap_left'], 'heightmap_top': mp['heightmap_top'],
                       'pixel_size': mp['pixel_size']},
        },
        'entities': entities,
        'warps': warps,
        'animations': meta.get('animations') or [],
        'chests': {'enabled': bool(meta.get('chests_enabled')), 'contents': meta.get('chests') or []},
        'characters': meta.get('characters') or [],
        'entity_flags': meta.get('entity_flags') or [],
        'tile_swaps': meta.get('tile_swaps') or [],
        'swap_flags': meta.get('swap_flags') or [],
        'locked_doors': meta.get('locked_doors') or [],
        'switches': meta.get('switches') or [],
    }
    (dst / 'room.yaml').write_text(
        '# Converted from landstalker_engine by tools/convert_data.py (room %d). Cell coordinates are world cells\n'
        '# (heightmap index + map.cell_offset); behaviours are legacy script numbers (data/text/behaviours.txt) until\n'
        '# the Lua scripts (scripts/rooms/room_%03d.lua) replace them.\n' % (number, number)
        + yaml.safe_dump(out, sort_keys=False, allow_unicode=True, width=140), encoding='utf-8')
    return {'id': number, 'name': name.split('/')[-1], 'path': name, 'entities': len(entities), 'warps': len(warps),
            'grid': [gw, gh]}


def main():
    here = Path(__file__).resolve().parent.parent
    ap = argparse.ArgumentParser()
    ap.add_argument('--engine-assets', default=str(here.parent / 'landstalker_engine' / 'assets'))
    ap.add_argument('--labels', default=str(here.parent / 'landstalker_editor' / 'landstalker_labels.yaml'))
    ap.add_argument('--out', default=str(here / 'data'))
    ap.add_argument('--rooms', default='all')
    a = ap.parse_args()
    assets, out = Path(a.engine_assets), Path(a.out)
    labels, entity_labels = load_labels(a.labels)
    helper = tileset_images(out)
    numbers = sorted(int(p.name[5:]) for p in (assets / 'rooms').glob('room_*')) if a.rooms == 'all' else \
        [int(x) for x in a.rooms.split(',')]
    index = []
    for n in numbers:
        index.append(convert_room(assets / 'rooms' / ('room_%03d' % n), out / 'rooms' / ('room_%03d' % n), n, labels, entity_labels, helper))
        if len(index) % 100 == 0:
            print(len(index), 'rooms', file=sys.stderr)
    (out / 'index.yaml').write_text(yaml.safe_dump({'rooms': index}, sort_keys=False, allow_unicode=True, width=140), encoding='utf-8')
    for sub in ('sprites', 'graphics', 'text'):
        if (assets / sub).exists():
            shutil.copytree(assets / sub, out / sub, dirs_exist_ok=True)
    # the engine's exports are in the old sprite format: poses and directions (docs/sprites_format.md)
    sys.path.insert(0, str(Path(__file__).resolve().parent))
    import migrate_sprites
    migrate_sprites.migrate(out)
    print('%d rooms -> %s' % (len(index), out))


if __name__ == '__main__':
    main()
