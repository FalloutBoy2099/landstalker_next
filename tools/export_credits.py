#!/usr/bin/env python3
"""Writes the credits of the ending as data/i18n/<lang>/credit.txt from the ROMs' export (roms/export/<lang>/End Credit Strings.txt).

Usage: export_credits.py LANG [EXPORT_DIR]

The file is a centred list (see `parse_credits` in cutscene.rs): `[GROUP]` is a group title (underlined; three empty lines come before it), `*Role*` a role,
an empty line an empty line, anything else a name; `;` starts a comment. The original's layout (heights, two columns for the cast) and its letter-spacing
codes are dropped, the underscores of the original (spaces) become spaces.
"""
import os
import re
import sys
from pathlib import Path

# the control codes of the original's credit font: kerning tags (K1..K4; renumbered in some ROMs), accents and the roman figures of the French "équipe"
FR = {'3E': 'é', '4B': '1', '4C': '2'}
KERN = re.compile(r'\{(K\d|5[0-3]|4[78]|FF)\}')


def clean(text, lang):
    text = re.sub(r'\{([0-9A-F]{2})\}', lambda m: FR.get(m.group(1), m.group(0)) if lang == 'fr' else m.group(0), text)
    text = KERN.sub('', text)
    text = re.sub(r'\{[A-Z0-9_]*LOGO\}', '', text)
    text = re.sub(r'\{UL[12]\}', '', text).replace('_', ' ').strip()
    words = text.split()
    if len(words) > 1 and all(len(w) == 1 for w in words):
        text = ''.join(words)          # "C A S T": the letter-spacing of the original's headings
    return text


def title_case(text):
    small = {'of', 'the', 'de', 'du', 'des', 'la', 'le', 'les', 'et', 'and', 'für', 'und'}
    words = text.lower().split(' ')
    return ' '.join(w if (i and w in small) else w[:1].upper() + w[1:] for i, w in enumerate(words))


def main():
    lang = sys.argv[1]
    root = Path(__file__).resolve().parent.parent
    export = Path(sys.argv[2] if len(sys.argv) > 2 else os.environ.get('LS_ROMS_EXPORT', r'D:\Git\Landstalker\roms\export'))
    rows = (export / lang / 'End Credit Strings.txt').read_text(encoding='utf-8').splitlines()[1:]
    out = ['; The credits of the ending (%s): a centred list that scrolls up.' % lang,
           ';   [GROUP]   a group title, underlined (three empty lines come before it)',
           ';   *Role*    a role, under a title',
           ';   a line    a name; an empty line is an empty line; ; starts a comment', '']
    first = True
    for row in rows:
        f = row.split('\t')
        if len(f) < 3 or not f[2].strip():
            continue
        raw = f[2]
        text = clean(raw, lang)
        if not text:
            continue
        if raw.startswith('{UL2}'):
            out.append('[%s]' % text)
        elif raw.startswith('{UL1}'):
            out.append('*%s*' % title_case(text))
        elif first:
            out.append('[%s]' % text)
        elif text == text.lower():
            out.append(title_case(text))      # the lower case of the original's font is small capitals
        else:
            out.append(text)
        first = False
    dest = root / 'data' / 'i18n' / lang / 'credit.txt'
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_text('\n'.join(out) + '\n', encoding='utf-8')
    print('%s: %d lines -> %s' % (lang, len(out), dest))


if __name__ == '__main__':
    main()
