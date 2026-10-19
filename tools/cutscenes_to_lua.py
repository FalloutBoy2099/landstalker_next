#!/usr/bin/env python3
"""Turns the story actions of data/cutscenes (YAML, one file per area) into Lua scripts, which are then written by hand (see data/scripts/common/cutscenes.lua
for the vocabulary): `story(n, { say(9), ... })`.

Usage: cutscenes_to_lua.py [data dir] [--force]

  data/scripts/rooms/room_NNN.lua      the actions only that room starts (with its behaviours)
  data/scripts/cutscenes/<area>.lua    the actions several rooms of one area start
  data/scripts/cutscenes/common.lua    the actions several areas start
  data/scripts/cutscenes/unassigned.lua  the ones no room starts (the story code starts them)
  data/scripts/cutscenes/routines.lua  the routines the actions call (`run(n)` / `say(n)` of a routine number)

An action belongs to the rooms whose entities start it (`cutscene(e, n)` in the behaviours) or whose characters' conversations run it; an action another
one runs goes with its caller. The area of a room is its folder in the game's tree. Existing script files are appended to only if they do not define the
action yet; the YAML files are removed once converted.
"""
import collections
import glob
import re
import shutil
import sys
from pathlib import Path

import yaml


def slug(text):
    return re.sub(r'[^a-z0-9]+', '_', text.lower()).strip('_')


def lua_bool(v):
    return 'true' if v else 'false'


def op_lua(o, ind):
    """One operation as a call of the vocabulary of common/cutscenes.lua."""
    pad = '  ' * ind
    if 'text' in o:
        return 'say(%d, %d)' % (o['text'], o['wait']) if o.get('wait') else 'say(%d)' % o['text']
    if 'flag' in o and 'if' not in o:
        return 'story_flag(%d, %s)' % (o['flag'], lua_bool(o.get('set', True)))
    if 'sound' in o:
        return 'play_sound(%d)' % o['sound']
    if 'swap' in o:
        return 'swap_tiles(%d)' % o['swap']
    if 'call' in o:
        return 'run(%d)' % o['call']
    if 'warp' in o:
        return 'go_to(%d, %d, %d, %d)' % (o['warp'], o.get('x', 0), o.get('y', 0), o.get('dir', -1))
    if 'lock' in o:
        return 'lock_controls(%s)' % lua_bool(o['lock'])
    if 'hostile' in o:
        return 'make_hostile(%d, %s)' % (o['hostile'], lua_bool(o.get('set', True)))
    if 'visible' in o:
        return 'show_entity(%d, %s)' % (o['visible'], lua_bool(o.get('set', True)))
    if 'credits' in o:
        return 'show_credits()'
    if 'intro_string' in o:
        return 'intro_caption(%d)' % o['intro_string']
    if 'fx' in o:
        return 'effect(%s)' % lua_str(o['fx'])
    if 'wait' in o:
        return 'pause_frames(%d)' % o['wait']
    if 'friday' in o:
        return 'friday_does(%s)' % lua_str(o['friday'])
    if 'hide' in o:
        return 'hide_entity(%d)' % o['hide']
    if 'block' in o:
        return 'block_cells({%s})' % ', '.join('{%d, %d}' % (c[0], c[1]) for c in o['block'])
    if 'if' in o:
        cond = o['if']
        then_ops = ops_lua(o.get('then', []), ind + 1)
        else_ops = ops_lua(o.get('else', []), ind + 1)
        head = 'if_flag(%d, ' % cond['flag'] if 'flag' in cond else 'if_yes('
        return head + then_ops + (', ' + else_ops if o.get('else') else '') + ')'
    raise ValueError('unknown operation %r' % (o,))


def lua_str(s):
    return '"%s"' % str(s).replace('\\', '\\\\').replace('"', '\\"')


def ops_lua(ops, ind):
    if not ops:
        return '{}'
    pad = '  ' * (ind + 1)
    lines = [pad + op_lua(o, ind + 1) + ',' for o in ops]
    return '{\n' + '\n'.join(lines) + '\n' + '  ' * ind + '}'


def collect(o, key, out):
    if isinstance(o, dict):
        for k, v in o.items():
            if k == key and isinstance(v, int):
                out.add(v)
            else:
                collect(v, key, out)
    elif isinstance(o, list):
        for v in o:
            collect(v, key, out)


PAD = [(0x01, 'up'), (0x02, 'down'), (0x04, 'left'), (0x08, 'right'), (0x10, 'b'), (0x20, 'c'), (0x40, 'a'), (0x80, 'start')]


def playbacks_to_lua(data):
    """data/cutscenes/playbacks.yaml -> data/scripts/cutscenes/playbacks.lua (`playback_seq(n, { press("up+right", 18), ... })`)."""
    path = data / 'cutscenes' / 'playbacks.yaml'
    if not path.exists():
        return
    seqs = yaml.safe_load(path.read_text(encoding='utf-8'))
    lines = ['-- Button presses the story replays in place of the player (see common/cutscenes.lua).', '']
    for n, steps in sorted(seqs.items()):
        items = []
        for st in steps:
            bits = int(str(st['buttons']), 16)
            keys = '+'.join(name for bit, name in PAD if bits & bit)
            items.append('press("%s", %d)' % (keys, st['frames']))
        lines.append('playback_seq(%d, { %s })' % (n, ', '.join(items)))
    out = data / 'scripts' / 'cutscenes' / 'playbacks.lua'
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(chr(10).join(lines) + chr(10), encoding='utf-8')
    path.unlink()
    print('%d playback sequences -> %s' % (len(seqs), out))


def main():
    args = [a for a in sys.argv[1:] if not a.startswith('--')]
    data = Path(args[0]) if args else Path(__file__).resolve().parent.parent / 'data'
    story_dir = data / 'cutscenes' / 'story'
    playbacks_to_lua(data)
    scripts = data / 'scripts'
    actions = {}
    for f in sorted(glob.glob(str(story_dir / '*.yaml'))):
        loaded = yaml.safe_load(open(f, encoding='utf-8'))
        m = re.search(r'action_(\d+)\.yaml$', f)
        if m:      # the flat files of migrate_text.py: one action each
            actions[int(m.group(1))] = loaded
        else:      # one file per area: `actions: {id: operations}`
            for k, ops in (loaded.get('actions') or {}).items():
                actions[int(k)] = ops
    routines = {}
    if (data / 'cutscenes' / 'routines.yaml').exists():
        routines = {int(k): v for k, v in (yaml.safe_load((data / 'cutscenes' / 'routines.yaml').read_text(encoding='utf-8')).get('routines') or {}).items()}
    for f in glob.glob(str(data / 'cutscenes' / 'routines' / 'routine_*.yaml')):
        routines[int(re.search(r'routine_(\d+)', f).group(1))] = yaml.safe_load(open(f, encoding='utf-8'))
    if not actions:
        print('no YAML actions: nothing to convert')
        return

    # which behaviours start which actions: cutscene(e, n) / cutscene(e, 0x100 + p) in the behaviours' Lua
    lua_text = ''.join(Path(f).read_text(encoding='utf-8') for f in glob.glob(str(scripts / '**' / '*.lua'), recursive=True))
    beh_actions = {}
    for m in re.finditer(r'^B\[(\d+)\] = function\(e\)(.*?)^end', lua_text, re.S | re.M):
        ids = {int(c.group(2)) + (256 if c.group(1) else 0) for c in re.finditer(r'cutscene\(e, (0x100 \+ )?(\d+)\)', m.group(2))}
        if ids:
            beh_actions[int(m.group(1))] = ids
    rooms = {}
    for f in glob.glob(str(data / 'rooms' / 'room_*' / 'room.yaml')):
        d = yaml.safe_load(open(f, encoding='utf-8'))
        rooms[d['id']] = d
    chars = {}
    for f in glob.glob(str(data / 'dialogues' / 'character_*.yaml')):
        chars[int(re.search(r'character_(\d+)', f).group(1))] = yaml.safe_load(open(f, encoding='utf-8'))
    act_rooms = collections.defaultdict(set)
    for rid, d in rooms.items():
        for e in d.get('entities', []):
            for a in beh_actions.get(e['behaviour'], ()):
                act_rooms[a].add(rid)
        for c in d.get('characters', []):
            found = set()
            collect(chars.get(c, {}), 'action', found)
            for a in found:
                act_rooms[a].add(rid)
    callers = collections.defaultdict(set)
    for a, ops in actions.items():
        called = set()
        collect(ops, 'call', called)
        for c in called:
            callers[c].add(a)
    changed = True
    while changed:
        changed = False
        for c, who in callers.items():
            for a in who:
                new = act_rooms.get(a, set()) - act_rooms.get(c, set())
                if new:
                    act_rooms[c] |= new
                    changed = True

    def area(rid):
        p = rooms[rid]['path'].split('/')
        return p[:2] if p[0] == 'Overworld' else p[:1]

    per_area = collections.Counter()
    for a in actions:
        for g in {tuple(area(r)) for r in act_rooms.get(a, ())}:
            per_area[g] += 1

    def group_of(rid):
        g = tuple(area(rid))
        return tuple(rooms[rid]['path'].split('/')[:2]) if per_area[g] > 30 else g

    files = collections.defaultdict(list)   # path -> [(n, lua)]
    for a in sorted(actions):
        rs = act_rooms.get(a, set())
        gs = {group_of(r) for r in rs}
        if len(rs) == 1:
            path = scripts / 'rooms' / ('room_%03d.lua' % next(iter(rs)))
        elif not gs:
            path = scripts / 'cutscenes' / 'unassigned.lua'
        elif len(gs) > 1:
            path = scripts / 'cutscenes' / 'common.lua'
        else:
            path = scripts / 'cutscenes' / (slug('_'.join(next(iter(gs)))) + '.lua')
        files[path].append((a, 'story(%d, %s)' % (a, ops_lua(actions[a], 0))))
    rpath = scripts / 'cutscenes' / 'routines.lua'
    files[rpath] = [(n, 'routine(%d, %s)' % (n, ops_lua(ops, 0))) for n, ops in sorted(routines.items())]

    for path, items in sorted(files.items()):
        path.parent.mkdir(parents=True, exist_ok=True)
        existing = path.read_text(encoding='utf-8') if path.exists() else ''
        body = '\n\n'.join(lua for n, lua in items if not re.search(r'^(story|routine)\(%d,' % n, existing, re.M))
        if not body:
            continue
        if not existing:
            if path.parent.name == 'rooms':
                existing = '-- Behaviours and story actions of room %d.\n' % int(path.stem[5:])
            elif path.stem == 'routines':
                existing = '-- Routines the story actions run (`run(n)`).\n'
            elif path.stem == 'common':
                existing = '-- Story actions that several areas start.\n'
            elif path.stem == 'unassigned':
                existing = '-- Story actions no room starts (the story code starts them).\n'
            else:
                existing = '-- Story actions of %s (started by several of its rooms).\n' % path.stem.replace('_', ' ')
        path.write_text(existing.rstrip('\n') + '\n\n-- story actions\n' + body + '\n' if 'story(' in body else existing.rstrip('\n') + '\n\n' + body + '\n', encoding='utf-8')
    shutil.rmtree(story_dir)
    for leftover in (data / 'cutscenes' / 'routines.yaml',):
        if leftover.exists():
            leftover.unlink()
    if (data / 'cutscenes' / 'routines').exists():
        shutil.rmtree(data / 'cutscenes' / 'routines')
    print('%d actions and %d routines -> %d Lua files' % (len(actions), len(routines), len(files)))


if __name__ == '__main__':
    main()
