#!/usr/bin/env python3
"""Migrates the sprite data to the "poses and directions" format (docs/sprites_format.md). Idempotent: what is already in the new
format is left alone, so it can be run after every `convert_data.py` (which copies the exports of the engine, in the old format).

  entities:  sprites/entities.yaml (one line per type: default_file, back_file, front_file, walk_back "file|w|h|ax|ay;...", ...)
             -> sprites/entities/entity_NNN/entity.yaml + its pictures in idle/ walk/ attack/ hurt/ folders
  sprites:   sprites/sprite_NNN/sprite.yaml (frames + animations; Nigel's animation 2 / 3 is the walk seen from behind / from the
             front...) -> poses with a view (any / back / front) each; Nigel's and the chest's pictures go to a folder per pose
  rooms:     the `images:` of a room's entity (default_file, back_file, front_file...) -> `poses: {idle: {any, back, front}}`

Usage: migrate_sprites.py [data dir]
"""
import json
import re
import shutil
import sys
from pathlib import Path

import yaml


def pic_line(p):
    return '{file: %s, w: %d, h: %d, ax: %d, ay: %d}' % (json.dumps(p['file']), p['w'], p['h'], p['ax'], p['ay'])


def emit_poses(poses, indent):
    """YAML lines of {pose: {view: [pictures]}}"""
    pad = ' ' * indent
    out = [pad + 'poses:']
    for pose, views in poses.items():
        out.append(pad + '  %s:' % pose)
        for view, pics in views.items():
            out.append(pad + '    %s:' % view)
            for p in pics:
                out.append(pad + '      - ' + pic_line(p))
    return out


def parse_list(text):
    """'file|w|h|ax|ay;file|w|h|ax|ay' -> [(file, w, h, ax, ay)]"""
    out = []
    for item in (text or '').split(';'):
        f = item.split('|')
        if len(f) == 5:
            out.append((f[0], int(f[1]), int(f[2]), int(f[3]), int(f[4])))
    return out


# ---------------------------------------------------------------------------------------------------------------------
# entities
# ---------------------------------------------------------------------------------------------------------------------

def migrate_entities(sprites):
    legacy = sprites / 'entities.yaml'
    if not legacy.exists():
        return 0
    table = yaml.safe_load(legacy.read_text(encoding='utf8'))['entities']
    for eid, d in table.items():
        folder = sprites / 'entities' / ('entity_%03d' % eid)
        folder.mkdir(parents=True, exist_ok=True)
        two = bool(d.get('has_front_and_back'))
        poses = {}

        def put(pose, view, entries, names):
            """moves the pictures into the entity's folder and records them"""
            for (src, w, h, ax, ay), name in zip(entries, names):
                source = sprites / src
                target = folder / name
                target.parent.mkdir(parents=True, exist_ok=True)
                if source.exists():
                    shutil.move(str(source), str(target))
                poses.setdefault(pose, {}).setdefault(view, []).append({'file': name, 'w': w, 'h': h, 'ax': ax, 'ay': ay})

        def single(key, prefix):
            if d.get(key + '_file'):
                return [(d[key + '_file'], d.get(key + '_w', 32), d.get(key + '_h', 32), d.get(key + '_ax', 16), d.get(key + '_ay', 16))]
            return []

        put('idle', 'any', single('default', ''), ['idle/any.png'])
        if two:
            put('idle', 'back', single('back', ''), ['idle/back.png'])
            put('idle', 'front', single('front', ''), ['idle/front.png'])
            for pose in ('walk', 'attack', 'hurt'):
                for view in ('back', 'front'):
                    entries = parse_list(d.get('%s_%s' % (pose, view)))
                    put(pose, view, entries, ['%s/%s_%d.png' % (pose, view, i) for i in range(len(entries))])
        lines = ['# Entity type %d (see docs/sprites_format.md)' % eid, 'sprite: %d' % d['sprite']]
        for k in ('hitbox_base', 'hitbox_height', 'enemy', 'health', 'defence', 'attack', 'gold_drop'):
            if k in d:
                lines.append('%s: %s' % (k, str(d[k]).lower() if isinstance(d[k], bool) else d[k]))
        lines += emit_poses(poses, 0)
        (folder / 'entity.yaml').write_text('\n'.join(lines) + '\n', encoding='utf8')
    legacy.unlink()
    # pictures nobody refers to
    leftovers = [p for p in (sprites / 'entities').glob('*.png')]
    for p in leftovers:
        p.unlink()
    return len(table)


# ---------------------------------------------------------------------------------------------------------------------
# animated sprites
# ---------------------------------------------------------------------------------------------------------------------

# Nigel's animations (the engine's convention: an even number seen from behind, the next one from the front, except 16)
NIGEL = {0: ('idle', 'back'), 1: ('idle', 'front'), 2: ('walk', 'back'), 3: ('walk', 'front'), 4: ('lift', 'back'), 5: ('lift', 'front'),
         6: ('carry_walk', 'back'), 7: ('carry_walk', 'front'), 8: ('jump', 'back'), 9: ('jump', 'front'),
         10: ('carry_jump', 'back'), 11: ('carry_jump', 'front'), 12: ('hurt', 'back'), 13: ('hurt', 'front'),
         14: ('sword', 'back'), 15: ('sword', 'front'), 16: ('hold_item', 'any'), 17: ('death', 'back'), 18: ('death', 'front')}
NAMED = {0: NIGEL, 36: {0: ('open', 'any')}}   # sprite 36 is the chest: closed, opening, open


def migrate_sprite(folder):
    path = folder / 'sprite.yaml'
    if not path.exists():
        return False
    text = path.read_text(encoding='utf8')
    doc = yaml.safe_load(text)
    if 'poses' in doc or 'animations' not in doc:
        return False
    sid = doc['sprite']['id']
    names = NAMED.get(sid)
    frames = doc['frames']
    poses, renamed = {}, {}
    for anim, frame_names in doc['animations'].items():
        n = int(re.search(r'(\d+)$', anim).group(1))
        pose, view = names[n] if names and n in names else ('anim_%02d' % n, 'any')
        pics = []
        for i, fname in enumerate(frame_names):
            f = frames[fname]
            old = f['file']
            if names and n in names:
                if old not in renamed:
                    renamed[old] = '%s/%s.png' % (pose, ('%s_%d' % (view, i)) if view != 'any' else str(i))
                new = renamed[old]
            else:
                new = old
            pics.append({'file': new, 'w': f['width'], 'h': f['height'], 'ax': f['anchor_x'], 'ay': f['anchor_y']})
        poses.setdefault(pose, {})[view] = pics
    for old, new in renamed.items():
        target = folder / new
        target.parent.mkdir(parents=True, exist_ok=True)
        if (folder / old).exists():
            shutil.move(str(folder / old), str(target))
    head = [l for l in text.splitlines() if l.startswith('#')]
    body = {k: v for k, v in doc.items() if k not in ('frames', 'animations')}
    out = '\n'.join(head) + '\n' + yaml.safe_dump(body, sort_keys=False, allow_unicode=True, width=140, default_flow_style=None)
    out += '\n'.join(emit_poses(poses, 0)) + '\n'
    path.write_text(out, encoding='utf8')
    return True


# ---------------------------------------------------------------------------------------------------------------------
# rooms
# ---------------------------------------------------------------------------------------------------------------------

IMAGES = re.compile(r'^( *)images:\n((?:\1  \w+: .*\n)+)', re.M)


def migrate_room_text(text):
    def convert(m):
        pad = len(m.group(1))
        kv = dict(line.strip().split(': ', 1) for line in m.group(2).splitlines())
        views = {}
        for view, key in (('any', 'default'), ('back', 'back'), ('front', 'front')):
            if key + '_file' in kv:
                views[view] = [{'file': kv[key + '_file'], 'w': int(kv.get(key + '_w', 32)), 'h': int(kv.get(key + '_h', 32)),
                                'ax': int(kv.get(key + '_ax', 16)), 'ay': int(kv.get(key + '_ay', 16))}]
        return '\n'.join(emit_poses({'idle': views}, pad)) + '\n'
    return IMAGES.sub(convert, text)


def migrate_rooms(rooms):
    n = 0
    for f in sorted(rooms.glob('room_*/room.yaml')):
        text = f.read_text(encoding='utf8')
        new = migrate_room_text(text)
        if new != text:
            f.write_text(new, encoding='utf8')
            n += 1
    return n


def migrate(data):
    data = Path(data)
    e = migrate_entities(data / 'sprites')
    s = sum(1 for f in sorted((data / 'sprites').glob('sprite_*')) if f.is_dir() and migrate_sprite(f))
    r = migrate_rooms(data / 'rooms') if (data / 'rooms').exists() else 0
    print('migrated: %d entity types, %d sprites, %d rooms' % (e, s, r))


if __name__ == '__main__':
    migrate(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / 'data')
