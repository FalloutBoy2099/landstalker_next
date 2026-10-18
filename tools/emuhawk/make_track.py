"""Turns a hawk.lua log of the original (analyze.entcols(), every 2 frames) into a cutscene track the port plays.

  python make_track.py LOG.csv OUT.track FIRST_FRAME LAST_FRAME

The track, all frames at 60 Hz counted from FIRST_FRAME:
  room <frame> <room>                          the screen is black: this room is loaded (the first line is the starting room)
  fade <frame> <0..1000>                       the black of the screen: 0 clear, 1000 black (the console's palette darkening)
  str <frame> <id> <frames>                    an intro caption (0: none) and the frames it still has to last
  handover <frame> <room>                      the recording ends: the game's own scripts play this room
  fr <frame> <x> <y>                           Friday's sprite on the screen, in pixels of the 320x224 picture (-1 -1: not shown)
  msg <frame> <text>                           the message box (| = new line; empty text closes it), added by hand
  e <frame> <slot> <x> <y> <z> <rot> <flags>   slot 0: the player, n: entity n-1 of the room; x and y in 1/16 cell, z in 1/16 unit,
                                               rot 0 NE 1 SE 2 SW 3 NW, flags 1 = not drawn; written when something changes
"""
import csv, sys

log, out, first, last = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
handover = set(int(r) for r in sys.argv[5].split(',')) if len(sys.argv) > 5 and sys.argv[5] else set()   # rooms the game's scripts play
extra = open(sys.argv[6]).read().splitlines() if len(sys.argv) > 6 else []                              # lines added by hand (msg ...)
rows = [r for r in csv.DictReader(open(log)) if first <= int(r['frame']) <= last]
SLOTS = 12


def slot(r, i):
    b = 0xFF5400 + 0x80 * i
    g = lambda off: int(r['0x%X' % (b + off)])
    x, y = g(0) * 16 + g(2), g(1) * 16 + g(3)
    hidden = (g(0) == 255 and g(1) == 255) or (g(8) & 1 != 0)
    if g(0xC) & 0x40:
        hidden = True          # Flags2 bit 6: not drawn (Nigel in the harbor scene is only the camera's anchor)
    return (x, y, g(0x12), (g(4) >> 6) & 3, 1 if hidden else 0)


lines = []
cur_room = None
pending = None      # (room, change frame): the new room shows when the screen is black
last_slot = {}
last_fade = None
last_str = None
last_fr = None
handing = False
for r in rows:
    f = int(r['frame']) - first
    room = int(r['g_RmNum1'])
    bright = int(r['g_PaletteDarkenCurrentBrightness'])
    level = 0 if bright >= 0xF000 else min(1000, bright * 1000 // 4096)
    if cur_room is None:
        cur_room = room
        lines.append('room %d %d' % (f, room))
    elif room != cur_room and room != 0:
        pending = (room, f)
        cur_room = room
    if pending and level >= 1000:
        lines.append('%s %d %d' % ('handover' if pending[0] in handover else 'room', f, pending[0]))
        if pending[0] in handover:
            handing = True
        last_slot = {}
        pending = None
        last_slot = {}          # the new room's entities: write them all again
    fade = level
    if fade != last_fade:
        lines.append('fade %d %d' % (f, fade))
        last_fade = fade
    if 'gVDPSprFriday_Y' in r:
        fy, fx = int(r['gVDPSprFriday_Y']), int(r['gVDPSprFriday_X'])
        fr = (fx - 128, fy - 128) if fy != 0 and fx != 0 else (-1, -1)
        if fr != last_fr and not pending:
            lines.append('fr %d %d %d' % (f, fr[0], fr[1]))
            last_fr = fr
    s = (int(r['g_IntroStringToDisplay']), int(r['g_IntroStringDisplayTime']))
    if s[0] != (last_str or (0, 0))[0]:
        lines.append('str %d %d %d' % (f, s[0], s[1]))
    last_str = s
    if pending:
        continue                # between the change of room and the black: the old room's entities are gone
    for i in range(SLOTS):
        st = slot(r, i)
        if last_slot.get(i) != st:
            lines.append('e %d %d %d %d %d %d %d' % ((f, i) + st))
            last_slot[i] = st
    if handing:
        break
open(out, 'w').write('# cutscene track recorded from the original ROM (tools/emuhawk/make_track.py)\n' + '\n'.join(lines + extra) + '\n')
print(len(lines), 'lines')
