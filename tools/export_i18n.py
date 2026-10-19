"""Moves the texts of data/text (exported from the ROM) into the i18n dictionaries: data/i18n/<lang>/*.yaml, and leaves keys in their place.

    python tools/export_i18n.py [lang]        (default en)

* dialogue.json   every string of a "messages" list becomes a key `dialogs.c<character>_<path>`
* cutscenes.json  the "texts" (id -> pages) become `cutscenes.t<id>` (the JSON keeps the actions)
* shops.json      the name of a room becomes `shops.<slug>`
* items.txt       one key per line (`items.<slug>`), the names go to items.yaml
The script can be run again: what is a key already is left alone. The captions of the intro (intro_strings.json) are pictures of the original's font
(tiles), not text: they are not in the dictionaries.
"""
import json
import re
import sys
from pathlib import Path

lang = sys.argv[1] if len(sys.argv) > 1 else 'en'
root = Path(__file__).resolve().parent.parent
text_dir = root / 'data' / 'text'
out_dir = root / 'data' / 'i18n' / lang
out_dir.mkdir(parents=True, exist_ok=True)


def is_key(s, domain):
    return isinstance(s, str) and s.startswith(domain + '.') and ' ' not in s


def yaml_value(v):
    """A YAML scalar or list written without a library (stable, readable output)."""
    if isinstance(v, list):
        return '\n' + '\n'.join('  - ' + json.dumps(x, ensure_ascii=False) for x in v)
    return ' ' + json.dumps(v, ensure_ascii=False)


def write_yaml(name, entries, header):
    lines = ['# ' + header]
    for k, v in entries.items():
        lines.append('%s:%s' % (k, yaml_value(v)))
    (out_dir / (name + '.yaml')).write_text('\n'.join(lines) + '\n', encoding='utf8')
    print(name + '.yaml:', len(entries), 'texts')


def load_existing(name):
    """The texts already exported (so that a second run keeps them)."""
    path = out_dir / (name + '.yaml')
    entries = {}
    if path.exists():
        import yaml
        entries = yaml.safe_load(path.read_text(encoding='utf8')) or {}
    return entries


# ---- dialogue.json
dialogs = load_existing('dialogs')
path = text_dir / 'dialogue.json'
data = json.loads(path.read_text(encoding='utf8'))


def walk(obj, p):
    if isinstance(obj, dict):
        for k, v in list(obj.items()):
            if k == 'messages' and isinstance(v, list):
                for i, m in enumerate(v):
                    if isinstance(m, str) and not is_key(m, 'dialogs'):
                        key = '%s_m%d' % (p, i)
                        dialogs[key] = m
                        v[i] = 'dialogs.' + key
            else:
                walk(v, '%s_%s' % (p, k))
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            walk(v, '%s_%d' % (p, i))


for cid, ch in data.get('characters', {}).items():
    walk(ch, 'c' + cid)
path.write_text(json.dumps(data, ensure_ascii=False), encoding='utf8')
write_yaml('dialogs', dialogs, 'conversations of the characters (data/text/dialogue.json holds the keys); {5B} {57} {5E}... are the original text codes')

# ---- cutscenes.json
cut = load_existing('cutscenes')
path = text_dir / 'cutscenes.json'
data = json.loads(path.read_text(encoding='utf8'))
for tid, pages in data.get('texts', {}).items():
    cut['t' + tid] = pages
data.pop('texts', None)
path.write_text(json.dumps(data, ensure_ascii=False), encoding='utf8')
write_yaml('cutscenes', cut, 'texts of the story actions: t<id> (data/text/cutscenes.json holds the actions); a list is the pages of a conversation')

# ---- shops.json
shops = load_existing('shops')
path = text_dir / 'shops.json'
data = json.loads(path.read_text(encoding='utf8'))
for rid, room in data.get('rooms', {}).items():
    name = room.get('name', '')
    if name and not is_key(name, 'shops'):
        slug = re.sub(r'[^a-z0-9]+', '_', name.lower()).strip('_') or 'room_' + rid
        shops[slug] = name
        room['name'] = 'shops.' + slug
path.write_text(json.dumps(data, ensure_ascii=False), encoding='utf8')
write_yaml('shops', shops, 'names of the shops, inns and churches (data/text/shops.json holds the keys)')


# ---- items.txt
def clean(name):
    """{6A} is a line break of the original's menu (a space here), the other codes are glyph variants."""
    out = re.sub(r'\{6A\}', ' ', name)
    return re.sub(r'\{[0-9A-Fa-f]{2}\}', '', out)


items = load_existing('items')
path = text_dir / 'items.txt'
keys = []
used = set()
for line in path.read_text(encoding='utf8').splitlines():
    if is_key(line, 'items'):
        keys.append(line)
        continue
    name = clean(line)
    slug = re.sub(r'[^a-z0-9]+', '_', name.lower()).strip('_') or 'item_%d' % len(keys)
    base, n = slug, 2
    while slug in used:
        slug = '%s_%d' % (base, n)
        n += 1
    used.add(slug)
    items[slug] = name
    keys.append('items.' + slug)
path.write_text('\n'.join(keys) + '\n', encoding='utf8')
write_yaml('items', items, 'names of the items (data/text/items.txt lists the keys, one per item number)')

