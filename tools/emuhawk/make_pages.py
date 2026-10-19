"""Times the boxes of the story in the intro from the original's sound: every letter typed in a box makes a blip (sound 70, 71 or 105 depending on who
speaks), so a burst of blips is a page being typed; the page stays on until the next one comes (or its `wait` after the typing).

    python make_pages.py INTRO.track [lang] > pages.txt          (lines `msg <frame> <key>#<page>` and `msg <frame>` to close)

The order of the pages is the order of the story actions 273, 276, 277, 278, 274 (data/cutscenes/story) of room 142 and 291 of room 143; the pages whose speaker
has no voice (Wally, Ink, a text without a speaker) make no blips: they follow the page before.
"""
import json
import sys
from pathlib import Path

import yaml

track = Path(sys.argv[1])
lang = sys.argv[2] if len(sys.argv) > 2 else 'en'
root = Path(__file__).resolve().parent.parent.parent
texts = yaml.safe_load((root / 'data' / 'i18n' / lang / 'cutscenes.yaml').read_text(encoding='utf8'))
actions = {}
for f in sorted((root / 'data' / 'cutscenes' / 'story').glob('*.yaml')):
    actions.update({str(k): v for k, v in (yaml.safe_load(f.read_text(encoding='utf8')).get('actions') or {}).items()})

# (action, first frame at which it may start): the blips found after that frame belong to it
CLOSES = [5592, 5802, 7532, 7902, 8322, 10972, 11564]
sequence = [(273, 4400), (276, 5900), (277, 7500), (278, 8100), (274, 8400), (291, 11200)]
NOVOICE = {'names.wally', 'names.ink', None}

events = [(int(l.split()[1]), int(l.split()[2])) for l in track.read_text().splitlines() if l.startswith('sfx ')]
blips = [(f, i) for f, i in events if i in (70, 71, 105) and f > 4000]
bursts = []
for f, i in blips:
    if bursts and f - bursts[-1][1] <= 45:
        bursts[-1][1] = f
    else:
        bursts.append([f, f])

pages = []   # (key, page index, wait, voiced, chars)
for action, _ in sequence:
    for op in actions[str(action)]:
        if 'text' not in op:
            continue
        key = 'cutscenes.t%d' % op['text']
        name = 't%d' % op['text']
        value = texts.get(name)
        if value is None:
            # texts above 1000 (the message "Nigel got 2000 golds!") are in the original's list of messages: a short page without voice
            value = {'text': ['Nigel got 2000 golds!'] if op['text'] in (254, 1141) else ['?'], 'speaker': None}
            key = None
        speaker = value.get('speaker') if isinstance(value, dict) else None
        plist = value['text'] if isinstance(value, dict) else value
        for n, p in enumerate(plist):
            pages.append((key, n, op.get('wait', 0), speaker not in NOVOICE, len(p)))

out = []
bi = 0
t_end = None
voiced_seen = 0
last_end = 0
for idx, (key, n, wait, voiced, chars) in enumerate(pages):
    if voiced:
        if bi >= len(bursts):
            break
        start, typed_end = bursts[bi]
        bi += 1
    else:
        start = last_end + 5
        typed_end = start + int(chars * 1.0)
    # the box of 1141 is a message that is not in the dictionaries: written by hand below
    last_end = typed_end + max(wait, 1)
    pages[idx] = (key, n, wait, voiced, chars, start, typed_end, last_end)

for idx, p in enumerate(pages):
    if len(p) < 8:
        continue
    key, n, wait, voiced, chars, start, typed_end, end = p
    nxt = pages[idx + 1] if idx + 1 < len(pages) and len(pages[idx + 1]) >= 8 else None
    if key:
        out.append('msg %d %s#%d' % (start, key, n))
    else:
        out.append('msg %d messages.intro.got_2000' % start)
# the box stays open from page to page; it closes where the original's picture loses its frame (found on screenshots of the original, in frames
# of the recording): after the pages 257, 259, 266, 268, 269 and 280, and after the one of room 143
out += ['msg %d' % f for f in CLOSES]
print(chr(10).join(sorted(out, key=lambda l: int(l.split()[1]))))
