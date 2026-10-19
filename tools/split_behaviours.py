#!/usr/bin/env python3
"""Splits the original's behaviours (data/behaviours.yaml, bytecode translated by behaviours_to_lua.py) into Lua files that are then written by hand:

  data/scripts/rooms/room_NNN.lua       the behaviours only one room uses (the room's own scenes: the bear that leaves in room 424, the rafts of 163 ...)
  data/scripts/behaviors/<type>.lua     the behaviours several rooms use, when one family of entities (an object, a creature, a character) is the only user
  data/scripts/behaviors/common.lua     the behaviours several rooms and several kinds of entities use

Each file defines `B[n] = function(e) ... end` (n is the number the room file gives to the entity: `behaviour: n`), and `SCRIPTED[n]` / `WAITS_FIRST[n]` when
the script waits for a story event. The behaviours no entity of the data uses are not written. Existing files are never overwritten (they are the
source now): use --force to start again.

Usage: split_behaviours.py [data dir] [--force]
"""
import collections
import glob
import re
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
import behaviours_to_lua as b2l


def slug(text):
    return re.sub(r'[^a-z0-9]+', '_', text.lower()).strip('_')


def family(type_name):
    """'Orc 1', 'Orc 3' -> 'Orc': the variants of a creature share a file."""
    return re.sub(r'\s+\d+$', '', type_name)


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    force = '--force' in sys.argv
    data = Path(args[0]) if args else Path(__file__).resolve().parent.parent / 'data'
    code = {int(k): bytes.fromhex(v) for k, v in yaml.safe_load((data / 'behaviours.yaml').read_text(encoding='utf-8')).items()}
    users = collections.defaultdict(lambda: {'rooms': set(), 'types': set()})
    for f in glob.glob(str(data / 'rooms' / 'room_*' / 'room.yaml')):
        d = yaml.safe_load(open(f, encoding='utf-8'))
        for e in d.get('entities', []):
            if e['behaviour']:
                users[e['behaviour']]['rooms'].add(d['id'])
                users[e['behaviour']]['types'].add(family(e['type_name']))
    files = collections.defaultdict(list)     # path -> [(n, lua)]
    skipped = 0
    for n, bytecode in sorted(code.items()):
        if n == 0:
            continue
        u = users.get(n)
        if not u:
            skipped += 1
            continue
        if len(u['rooms']) == 1:
            path = data / 'scripts' / 'rooms' / ('room_%03d.lua' % next(iter(u['rooms'])))
        elif len(u['types']) == 1:
            path = data / 'scripts' / 'behaviors' / (slug(next(iter(u['types']))) + '.lua')
        else:
            path = data / 'scripts' / 'behaviors' / 'common.lua'
        lua = b2l.translate(n, bytecode)
        if b2l.is_scripted(bytecode):
            lua += '\nSCRIPTED[%d] = true' % n
        if bytecode[:1] in (b'\x42', b'\x46'):
            lua += '\nWAITS_FIRST[%d] = true' % n
        files[path].append((n, lua))
    written = kept = 0
    for path, items in sorted(files.items()):
        if path.exists() and not force:
            kept += 1
            continue
        path.parent.mkdir(parents=True, exist_ok=True)
        if path.parent.name == 'rooms':
            head = '-- Behaviours of room %d (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].' % int(path.stem[5:])
        elif path.stem == 'common':
            head = '-- Behaviours that several rooms and several kinds of entities use.'
        else:
            head = '-- Behaviours that several rooms use for the entities of this kind (%s).' % path.stem.replace('_', ' ')
        path.write_text(head + '\n\n' + '\n\n'.join(lua for _, lua in items) + '\n', encoding='utf-8')
        written += 1
    print('%d files written, %d kept, %d behaviours used by no entity not written' % (written, kept, skipped))


if __name__ == '__main__':
    main()
