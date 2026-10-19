"""Adds who speaks to the texts of the story actions (data/i18n/<lang>/cutscenes.yaml): the original prints "Owner: Heh heh heh!" in its boxes, the name
comes from the `LOAD CHARACTER SCRIPT <name>` / `LOAD SPECIAL CHARACTER <name>` that runs before the `PRINT MSG`, and stays until another one.

    python tools/export_speakers.py [lang] [path of the disassembly's script/en/cutscenes folder]

The texts become maps `{speaker: names.<slug>, text: [pages]}` and the names go to data/i18n/<lang>/names.yaml. The texts are matched with the disassembly's messages by their content.
"""
import re
import sys
from pathlib import Path

import yaml

lang = sys.argv[1] if len(sys.argv) > 1 else 'en'
src = Path(sys.argv[2]) if len(sys.argv) > 2 else Path(r'D:\Git\Landstalker\landstalker_disasm\code\script\en\cutscenes')
root = Path(__file__).resolve().parent.parent
i18n = root / 'data' / 'i18n' / lang
MSG_BASE = 1762

speaker_of = {}
item_in_text = {}     # text -> the item whose name {5F} stands for (LOAD n INTO 0xFF1198)
names = {}
for name in ('script_cutscenetable.asm', 'script_cutscenes.asm'):
    path = src / name
    if not path.exists():
        continue
    speaker = None
    item_var = None
    for line in path.read_text(encoding='latin-1').splitlines():
        m = re.search(r'LOAD (?:CHARACTER SCRIPT|SPECIAL CHARACTER) (.+?)\s*\(0x', line)
        if m:
            who = m.group(1).strip()
            speaker = None if who == '*' else who
        m = re.search(r'INTO 0xFF1198 \((.+?)\)', line)
        if m:
            item_var = m.group(1)
        m = re.search(r'PRINT MSG 0x[0-9A-Fa-f]+[^:]*: "(.*)"\s*$', line)
        if m and '{5F}' in m.group(1) and item_var:
            item_in_text[m.group(1).replace('	', ' ')] = item_var
        if m and speaker:
            speaker_of[m.group(1).replace('	', ' ')] = speaker
for who in set(speaker_of.values()):
    names[re.sub(r'[^a-z0-9]+', '_', who.lower()).strip('_')] = who

path = i18n / 'cutscenes.yaml'
data = yaml.safe_load(path.read_text(encoding='utf8')) or {}
lines = ['# texts of the story actions: t<id>, a list of pages, or {speaker: names.<who>, text: pages} (data/cutscenes/story holds the actions)']
done = 0
for key, value in data.items():
    pages = value['text'] if isinstance(value, dict) else value
    # the text is found in the disassembly by its content (the ids of the messages do not follow ours)
    who = speaker_of.get(pages[0].replace('	', ' ')) if pages else None
    item = item_in_text.get(pages[0].replace('	', ' ')) if pages else None
    if item:
        pages = [p.replace('{5F}', item) for p in pages]
    if who:
        slug = re.sub(r'[^a-z0-9]+', '_', who.lower()).strip('_')
        lines.append('%s:\n  speaker: %s\n  text:' % (key, 'names.' + slug))
        lines += ['    - ' + __import__('json').dumps(p, ensure_ascii=False) for p in pages]
        done += 1
    else:
        lines.append('%s:' % key)
        lines += ['  - ' + __import__('json').dumps(p, ensure_ascii=False) for p in pages]
path.write_text('\n'.join(lines) + '\n', encoding='utf8')
(i18n / 'names.yaml').write_text('# names of the characters that speak in the boxes of the story\n' + ''.join('%s: %s\n' % (k, v) for k, v in sorted(names.items())), encoding='utf8')
print(done, 'texts with a speaker,', len(names), 'names:', ', '.join(sorted(names.values())))
