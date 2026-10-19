#!/usr/bin/env python3
"""Makes the dictionaries of another language (data/i18n/<lang>/*.yaml) from the paired exports of the ROMs (roms/export/<lang>.json: one entry per English string,
with its translation, see its `meta`), keeping the keys of the English dictionaries (data/i18n/en) so that the code and the scripts do not change.

Usage: export_i18n_lang.py LANG [EXPORT_DIR]          (default EXPORT_DIR = D:/Git/Landstalker/roms/export, or LS_ROMS_EXPORT)

The English texts of data/i18n/en (conversations, story texts, item names, character names) are found among the English strings of the export by their text
(the original's control codes written as {5B}, {57}...), and replaced by the translation, written with the same codes. A text the translation did not pair
(`match: missing`) is left out (the game shows the English one); a text that the translation spreads over several boxes becomes a list of pages.
Writes data/i18n/<lang>/_unmatched.txt with what could not be paired.
"""
import json
import os
import re
import sys
from pathlib import Path

import yaml

TAGS = {'start': '5B', 'nl': '57', 'end': '5E', 'end_alt': '62', 'choice': '58', 'pause': '59', 'number': '5A', 'item': '5F', 'opt': '56'}


def codes(text):
    """The export's tags back to the original's control codes (English numbering)."""
    def tag(m):
        name = m.group(1)
        if name in TAGS:
            return '{%s}' % TAGS[name]
        if name.startswith('ctrl_'):
            return '{%s}' % name[5:]
        return m.group(0)
    return re.sub(r'\{([a-z_0-9A-Z]+)\}', tag, text)


def plain_item(text):
    """An item name: the breaks of the menu become a space (or nothing, or a hyphen)."""
    return text.replace('{br_space}', ' ').replace('{br_hyphen}', '-').replace('{br}', '')


def norm(text):
    return re.sub(r'\s+', ' ', text).strip().lower()


def squash(text):
    """Only the letters and digits, without the control codes: texts that differ by a space, a tab or a code still pair."""
    return re.sub(r'[^a-z0-9]', '', re.sub(r'\{[^}]*\}', '', text.lower()))


def main():
    lang = sys.argv[1]
    root = Path(__file__).resolve().parent.parent
    export = Path(sys.argv[2] if len(sys.argv) > 2 else os.environ.get('LS_ROMS_EXPORT', r'D:\Git\Landstalker\roms\export'))
    data = json.loads((export / (lang + '.json')).read_text(encoding='utf-8'))
    en_dir, out_dir = root / 'data' / 'i18n' / 'en', root / 'data' / 'i18n' / lang
    out_dir.mkdir(parents=True, exist_ok=True)

    # the English strings of the export by their text: [(entry, parts)] in the order of the ROM
    pool, loose = {}, {}
    for s in data['strings']:
        pool.setdefault(norm(codes(s['en'])), []).append(s)
        loose.setdefault(squash(s['en']), []).append(s)
    used = set()
    unmatched = []

    def translate(text, label):
        """The pages of the translation of one English text, or None."""
        entries = pool.get(norm(text)) or loose.get(squash(text))
        if not entries:
            unmatched.append('%s: no such English string: %s' % (label, text[:80]))
            return None
        entry = next((e for e in entries if e['id'] not in used), entries[0])
        used.add(entry['id'])
        if entry.get(lang) is None:
            unmatched.append('%s: no translation (id %d): %s' % (label, entry['id'], text[:80]))
            return None
        if entry.get('match') == 'split' and entry.get(lang + '_parts'):
            return [codes(p) for p in entry[lang + '_parts']]
        return [codes(entry[lang])]

    def pages_value(pages):
        return pages[0] if len(pages) == 1 else pages

    todo = set()    # (file, key) whose text is still the English one: marked "TO TRANSLATE" so that it is easy to find and replace

    def write(name, header, mapping):
        lines = ['# ' + header]
        for k, v in mapping.items():
            if (name, k) in todo:
                lines.append('# TO TRANSLATE (English text)')
            lines.append(yaml.safe_dump({k: v}, allow_unicode=True, width=10000, default_flow_style=False).rstrip())
        (out_dir / (name + '.yaml')).write_text('\n'.join(lines) + '\n', encoding='utf-8')

    # conversations
    dialogs = yaml.safe_load((en_dir / 'dialogs.yaml').read_text(encoding='utf-8'))
    out = {}
    for key, text in dialogs.items():
        t = translate(text, 'dialogs.' + key)
        if t:
            out[key] = pages_value(t)
        else:
            out[key] = text
            todo.add(('dialogs', key))
    write('dialogs', 'conversations of the characters (%s); {5B} {57} {5E}... are the original text codes' % lang, out)
    n_dialogs = len(out)

    # story texts: t<id>: pages, or {speaker, text: pages}
    cutscenes = yaml.safe_load((en_dir / 'cutscenes.yaml').read_text(encoding='utf-8'))
    out = {}
    for key, value in cutscenes.items():
        pages = value['text'] if isinstance(value, dict) else value
        res = []
        missing = False
        for i, page in enumerate(pages):
            t = translate(page, 'cutscenes.%s#%d' % (key, i))
            missing = missing or not t
            res.extend(t if t else [page])
        out[key] = {'speaker': value['speaker'], 'text': res} if isinstance(value, dict) and 'speaker' in value else res
        if missing:
            todo.add(('cutscenes', key))
    write('cutscenes', 'texts of the story actions (%s): t<id> a list of pages, or {speaker, text}' % lang, out)

    # items, by name
    items = yaml.safe_load((en_dir / 'items.yaml').read_text(encoding='utf-8'))
    by_name = {norm(plain_item(codes(x['en']))): x for x in data['item_names']}
    out = {}
    for key, name in items.items():
        x = by_name.get(norm(name))
        if x:
            out[key] = plain_item(codes(x[lang]))
        else:
            out[key] = name
            todo.add(('items', key))
            unmatched.append('items.%s: %s' % (key, name))
    write('items', 'names of the items (%s)' % lang, out)

    # characters (who speaks)
    names = yaml.safe_load((en_dir / 'names.yaml').read_text(encoding='utf-8'))
    chars = {norm(x['en']): x[lang] for x in data['character_names'] + data['special_character_names']}
    out = {}
    for key, name in names.items():
        if norm(name) in chars:
            out[key] = chars[norm(name)]
        else:
            out[key] = name
            todo.add(('names', key))
            unmatched.append('names.%s: %s' % (key, name))
    write('names', 'names of the characters that speak in the boxes of the story (%s)' % lang, out)

    (out_dir / '_unmatched.txt').write_text('\n'.join(unmatched) + '\n', encoding='utf-8')
    print('%s: %d conversation texts, %d story texts, %d items, %d names; %d not paired (data/i18n/%s/_unmatched.txt)' % (lang, n_dialogs, len(cutscenes), len(items), len(names), len(unmatched), lang))


if __name__ == '__main__':
    main()
