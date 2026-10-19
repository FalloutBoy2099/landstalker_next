#!/usr/bin/env python3
"""Turns the text/ tables the C++ engine exports (JSON and txt files) into the YAML layout of landstalker_next: one file per thing, shared tables in one file.

Usage: migrate_text.py [--data DIR] [--legacy DIR]      (default: landstalker_next/data, and DIR/text for the old files)

  cutscenes/story/action_<id>.yaml      one file per story action (the list of operations: text, fade, flag, warp...)
  cutscenes/routines/routine_<id>.yaml  one file per routine the actions call (`call: id`)
  scripts/intro/intro_strings.yaml      the captions of the intro (tiles, frames)
  cutscenes/playbacks.yaml              the recorded button presses of the scenes (sequence -> [buttons, frames]); cutscenes_to_lua.py turns it into Lua
  dialogues/character_<id>.yaml         one file per character (what he says by story progress)
  dialogues/progress.yaml               the flags that give the story progress
  shops.yaml                            the shops, inns and churches by room
  items.yaml                            item index -> name key, base price, stack
  enemies.yaml                          what the enemies drop
  room_versions.yaml                    rooms replaced by another version once a flag is set
  triggers.yaml                         the trigger programs (WaitForCondition)
  behaviours.yaml                       the original's behaviour bytecode (input of behaviours_to_lua.py)

The old files are removed once converted. Nothing but YAML is left in the data folder (except the Tiled maps and the recorded intro timeline).
"""
import argparse
import json
import re
import shutil
from pathlib import Path

import yaml


def dump(path, data, header=''):
    path.parent.mkdir(parents=True, exist_ok=True)
    text = yaml.safe_dump(data, sort_keys=False, allow_unicode=True, width=160, default_flow_style=None)
    path.write_text((header + '\n' if header else '') + text, encoding='utf-8')


def lines_of(path):
    return [l.rstrip() for l in path.read_text(encoding='utf-8').splitlines() if l.strip() and not l.startswith('#')]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--data', default=str(Path(__file__).resolve().parent.parent / 'data'))
    ap.add_argument('--legacy', default='')
    a = ap.parse_args()
    data = Path(a.data)
    old = Path(a.legacy) if a.legacy else data / 'text'
    if not old.exists():
        print('nothing to convert:', old)
        return

    # --- cutscenes: one file per action, one per routine
    j = json.loads((old / 'cutscenes.json').read_text(encoding='utf-8'))
    for k, ops in j.get('actions', {}).items():
        dump(data / 'cutscenes' / 'story' / ('action_%s.yaml' % k), ops, '# story action %s: operations run in order (text, fade, flag, warp, sound, friday ...)' % k)
    for k, ops in j.get('scripts', {}).items():
        dump(data / 'cutscenes' / 'routines' / ('routine_%s.yaml' % k), ops, '# routine %s: operations that actions call with `call: %s`' % (k, k))
    for key in j:
        if key not in ('actions', 'scripts'):
            print('cutscenes.json: unused key', key)
    dump(data / 'scripts' / 'intro' / 'intro_strings.yaml', json.loads((old / 'intro_strings.json').read_text(encoding='utf-8')),
         '# the captions of the intro by number: the rectangle of the picture (x1 y1 x2 y2), how long it stays (frames) and the tiles of its two lines')

    # --- playbacks: "sequence hex:frames hex:frames ..."
    playbacks = {}
    for l in lines_of(old / 'playback.txt'):
        f = l.split()
        playbacks[int(f[0])] = [{'buttons': p.split(':')[0], 'frames': int(p.split(':')[1])} for p in f[1:]]
    dump(data / 'cutscenes' / 'playbacks.yaml', playbacks, '# the button presses the story replays (buttons: hex bits of the pad, frames: how long)')

    # --- dialogues: one file per character
    d = json.loads((old / 'dialogue.json').read_text(encoding='utf-8'))
    for k, ch in d.get('characters', {}).items():
        dump(data / 'dialogues' / ('character_%s.yaml' % k), ch, '# what character %s says, by story progress' % k)
    dump(data / 'dialogues' / 'progress.yaml', [{'flag': f, 'progress': p} for f, p in d.get('progress_flags', [])],
         '# the story progress: the first flag of this list that is set gives the progress value')
    for key in d:
        if key not in ('characters', 'progress_flags'):
            print('dialogue.json: unused key', key)

    # --- shops
    s = json.loads((old / 'shops.json').read_text(encoding='utf-8'))
    dump(data / 'shops.yaml', s, '# shops, inns and churches by room number (markup / 16 of the base price of items.yaml)')

    # --- items: names (one per line, the index is the line) + base prices (index price stack)
    names = lines_of(old / 'items.txt')
    items = {i: {'name': n} for i, n in enumerate(names)}
    for l in lines_of(old / 'item_prices.txt'):
        f = [int(x) for x in l.split()]
        row = items.setdefault(f[0], {})
        row['price'] = f[1]
        if len(f) > 2:
            row['stack'] = f[2]
    dump(data / 'items.yaml', items, '# items by index: name key (data/i18n items.yaml), base price in gold (above 1000: not sold), stack')

    # --- enemies: drops (enemy type, item index, one chance in N)
    drops = {}
    for l in lines_of(old / 'enemy_drops.txt'):
        f = [int(x) for x in l.split()]
        drops[f[0]] = {'item': f[1], 'one_in': f[2]}
    dump(data / 'enemies.yaml', {'drops': drops}, '# what the enemies leave behind by enemy type: item index, one chance in N (1 = always)')

    # --- room versions
    versions = []
    for l in lines_of(old / 'room_transitions.txt'):
        f = [int(x) for x in l.split()]
        versions.append({'room': f[0], 'replaced_by': f[1], 'flag': f[2]})
    dump(data / 'room_versions.yaml', versions, '# rooms replaced by another version once a story flag is set')

    # --- triggers: "TA n" then the program's lines
    triggers = {}
    cur = None
    for l in lines_of(old / 'triggers.txt'):
        m = re.match(r'TA (\d+)$', l.strip())
        if m:
            cur = int(m.group(1))
            triggers[cur] = []
        elif cur is not None:
            triggers[cur].append(l.strip())
    dump(data / 'triggers.yaml', triggers, '# trigger programs (WaitForCondition) by number: one instruction per line (cmp, tst, btst, load, branches, labels)')

    # --- behaviours: "index hexbytes"
    behaviours = {}
    for l in lines_of(old / 'behaviours.txt'):
        i, b = l.split()
        behaviours[int(i)] = b
    dump(data / 'behaviours.yaml', behaviours, '# the original\'s behaviour bytecode by number (hex); behaviours_to_lua.py turns it into Lua')

    shutil.rmtree(old)
    print('converted ->', data)


if __name__ == '__main__':
    main()
