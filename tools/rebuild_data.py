#!/usr/bin/env python3
"""Rebuilds the data folder from the C++ engine's assets, in the order the steps need:

  1. convert_data.py       rooms, sprites, graphics, tilesets; copies the engine's text/ tables (JSON and txt) into data/text
  2. export_i18n.py        moves the texts of data/text into data/i18n/<lang>/*.yaml and leaves keys in the tables
  3. export_speakers.py    adds who speaks to the texts of the story actions
  4. migrate_text.py       data/text -> YAML (cutscenes, dialogues, shops, items, enemies, triggers, behaviours ...), data/text removed
  5. split_behaviours.py   the original's behaviour bytecode -> Lua files: data/scripts/rooms/room_NNN.lua, behaviors/<type>.lua (never overwrites a script)
  6. cutscenes_to_lua.py   the story actions -> Lua (in the room files, or data/scripts/cutscenes/<area>.lua)
  7. export_credits.py     the credits of the ending -> data/i18n/<lang>/credit.txt (en, fr)
  8. export_i18n_lang.py   the other languages' dictionaries from roms/export/<lang>.json (fr)

Usage: rebuild_data.py [convert_data.py arguments]
"""
import subprocess
import sys
from pathlib import Path

here = Path(__file__).resolve().parent
steps = [['convert_data.py'] + sys.argv[1:], ['export_i18n.py'], ['export_speakers.py'], ['migrate_text.py'], ['split_behaviours.py'], ['cutscenes_to_lua.py'], ['export_credits.py', 'en'], ['export_credits.py', 'fr'], ['export_i18n_lang.py', 'fr']]
for step in steps:
    print('==', ' '.join(step), flush=True)
    subprocess.run([sys.executable, str(here / step[0])] + step[1:], check=True)
