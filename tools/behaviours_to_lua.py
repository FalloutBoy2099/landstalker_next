#!/usr/bin/env python3
"""Translates the sprite behaviour bytecode of the original game (data/text/behaviours.txt, `N hexbytes` per line) into Lua:
data/scripts/generated/behaviours.lua defines B[n] = function(e) ... end for every behaviour n.

The command semantics (what yields, what carries on in the same tick) follow the C++ engine's RoomView::BehaviourTick;
the commands themselves are Lua functions of data/scripts/global.lua. Edit THAT file or write a hand-made script
(data/scripts/objects, critters, rooms), never the generated one.

Usage: behaviours_to_lua.py [data dir]
"""
import sys
from pathlib import Path

data = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / 'data')

# command -> (length in bytes, lua template with {p} = first parameter, {q} = second, {f} = flag number)
T = {
    0x00: (2, 'pause(e, {p})'),
    0x01: (2, 'move_timed(e, {p})'),
    0x11: (2, 'move_wait(e, {p})'),
    0x4F: (2, 'move_noclip(e, {p})'),
    0x30: (2, 'move_up_relative(e, {p})'),
    0x31: (2, 'move_down_relative(e, {p})'),
    0x3A: (2, 'move_up_timed(e, {p})'),
    0x3B: (2, 'move_down_timed(e, {p})'),
    0x3C: (2, 'move_up_to(e, {p})'),
    0x3D: (2, 'move_down_to(e, {p})'),
    0x13: (1, 'move_until_collision(e)'),
    0x2A: (3, 'move_random_time(e, {p}, {q})'),
    0x02: (1, 'turn_wait(e, e.orientation + 1)'), 0x08: (1, 'turn_wait(e, e.orientation + 1)'),
    0x03: (1, 'turn_wait(e, e.orientation + 3)'), 0x09: (1, 'turn_wait(e, e.orientation + 3)'),
    0x04: (1, 'turn_wait(e, 0)'), 0x0A: (1, 'turn_wait(e, 0)'),
    0x05: (1, 'turn_wait(e, 1)'), 0x0B: (1, 'turn_wait(e, 1)'),
    0x06: (1, 'turn_wait(e, 2)'), 0x0C: (1, 'turn_wait(e, 2)'),
    0x07: (1, 'turn_wait(e, 3)'), 0x0D: (1, 'turn_wait(e, 3)'),
    0x14: (1, 'turn_wait(e, math.random(0, 3))'), 0x15: (1, 'turn_wait(e, math.random(0, 3))'),
    0x23: (1, 'turn_wait(e, e.orientation + 2)'), 0x24: (1, 'turn_wait(e, e.orientation + 2)'),
    0x17: (1, 'turn(e, math.random(0, 3))'),
    0x18: (1, 'turn(e, e.orientation + 1)'),
    0x19: (1, 'turn(e, e.orientation + 3)'),
    0x1A: (1, 'turn(e, 0)'), 0x1B: (1, 'turn(e, 1)'), 0x1C: (1, 'turn(e, 2)'), 0x1D: (1, 'turn(e, 3)'),
    0x25: (1, 'turn(e, e.orientation + 2)'),
    0x1F: (1, 'e.speed = 1'), 0x20: (1, 'e.speed = 2'), 0x21: (1, 'e.speed = 4'), 0x22: (1, 'e.speed = 8'),
    0x0E: (1, 'e.visible = true; e.base_visible = true'),
    0x0F: (1, 'e.visible = false; e.base_visible = false'),
    0x44: (1, 'e.visible = false'),
    0x45: (1, 'show_when_clear(e)'),
    0x3E: (1, 'e.visible = false; do return end'),
    0x41: (3, 'set_flag({f})'),
    0x43: (3, 'clear_flag({f})'),
    0x42: (3, 'wait_flag(e, {f}, true)'),
    0x46: (3, 'wait_flag(e, {f}, false)'),
    0x62: (2, 'sfx({p})'),
    0x4B: (3, 'move_to(e, {p}, {q})'),
    0x50: (2, 'rotate_player({p})'),
    0x57: (2, 'tile_swap({p})'),
    0x4C: (2, ''), 0x47: (2, ''), 0x5A: (2, ''),
    0x40: (1, 'move_down_until_collision(e)'),
    0x3F: (1, ''), 0x56: (1, ''),
    0x5F: (2, ''), 0x60: (2, ''), 0x61: (2, ''), 0x63: (2, ''), 0x65: (2, ''), 0x66: (2, ''),
    0x34: (2, 'playback({p})'),
    0x64: (2, 'cutscene(e, 0x100 + {p})'),
    0x36: (2, 'wait_trigger(e, {p})'),
    0x38: (1, 'e.falls = true; e.fall_v = 0'),
    0x4E: (2, 'cutscene(e, {p})'),
    0x51: (1, 'e.hostile = true; e.dormant = false'),
    0x52: (1, 'e.hostile = false'),
    0x2B: (1, 'e.dormant = false; do return end'),
    0x49: (1, 'activate_switch(e)'),
    0x4A: (1, 'turn(e, 0)'),
    0x37: (1, 'pause_player()'),
    0x1E: (1, ''), 0x27: (1, ''), 0x28: (1, ''), 0x29: (1, ''), 0x35: (1, ''), 0x39: (1, ''),
    0x53: (1, ''), 0x54: (1, ''), 0x58: (1, ''),
}


def translate(n, code):
    """Lua source of one behaviour."""
    # first pass: instruction boundaries and jump targets
    pcs, targets = [], set()
    pc = 0
    while pc < len(code):
        cmd = code[pc]
        par = code[pc + 1] if pc + 1 < len(code) else 0
        length = 3 if cmd in (0x41, 0x42, 0x43, 0x46, 0x4B, 0x2A) else 2 if cmd in (0x12, 0x5D) else T.get(cmd, (1, None))[0]
        pcs.append((pc, cmd, length))
        if cmd == 0x12:
            targets.add(pc + (par - 256 if par >= 128 else par))
        if cmd == 0x5D:
            targets.add(pc + 2)
        pc += length
    lines = ['B[%d] = function(e)' % n]
    last_repeat = None
    for pc, cmd, length in pcs:
        b = code[pc:pc + length] + bytes(3)
        p, q = b[1], b[2]
        if pc in targets:
            lines.append('  ::L%d::' % pc)
        if cmd == 0x00 and p == 0 and pc == 0 and len(code) == 2:
            lines.append('  do return end  -- idle')
            break
        if cmd == 0x12:
            tgt = pc + (p - 256 if p >= 128 else p)
            lines.append('  jump_guard(e) goto L%d' % tgt if tgt in {x[0] for x in pcs} else '  do return end  -- jump outside')
        elif cmd == 0x5D:
            lines.append('  repeat_begin(e, %d)' % p)
            last_repeat = pc + 2
        elif cmd == 0x5E:
            lines.append('  if repeat_next(e) then goto L%d end' % last_repeat if last_repeat is not None else '  -- repeat end without begin')
        elif cmd in T:
            tpl = T[cmd][1]
            if tpl:
                lines.append('  ' + tpl.format(p=p, q=q, f=p * 8 + q))
        else:
            lines.append('  do return end  -- unknown command %02X' % cmd)
            break
    lines.append('end')
    return '\n'.join(lines)


SIZES = {0x00: 2, 0x01: 2, 0x11: 2, 0x12: 2, 0x2A: 3, 0x41: 3, 0x42: 3, 0x43: 3, 0x46: 3, 0x4B: 3, 0x4E: 2, 0x62: 2, 0x34: 2,
         0x64: 2, 0x36: 2, 0x57: 2, 0x50: 2, 0x5D: 2, 0x4F: 2, 0x30: 2, 0x31: 2, 0x3A: 2, 0x3B: 2, 0x3C: 2, 0x3D: 2, 0x4C: 2,
         0x47: 2, 0x5A: 2, 0x5F: 2, 0x60: 2, 0x61: 2, 0x63: 2, 0x65: 2, 0x66: 2, 0x48: 3, 0x59: 3, 0x5B: 5}


def is_scripted(code):
    """RoomView's Behaviours::IsScripted: the script waits for a flag, starts a cutscene or turns the entity hostile. An enemy
    with such a script sleeps (dormant) until the script wakes it; the others are driven by the enemy AI (data/scripts/ai.lua)."""
    i = 0
    while i < len(code):
        if code[i] in (0x42, 0x46, 0x4E, 0x64, 0x51, 0x52):
            return True
        i += SIZES.get(code[i], 1)
    return False


def main():
    out = ['-- Generated by tools/behaviours_to_lua.py from data/text/behaviours.txt. Do not edit: see data/scripts/global.lua.', 'B = {}', '']
    n_scripts = 0
    scripted = []
    waits_first = []
    for line in (data / 'text' / 'behaviours.txt').read_text().splitlines():
        if not line or line.startswith('#'):
            continue
        num, hexa = line.split()
        out.append(translate(int(num), bytes.fromhex(hexa)))
        out.append('')
        if is_scripted(bytes.fromhex(hexa)):
            scripted.append(int(num))
        if bytes.fromhex(hexa)[:1] in (b'\x42', b'\x46'):
            waits_first.append(int(num))
        n_scripts += 1
    out.append('-- behaviours that wait for a story event or turn their entity hostile (see is_scripted in the generator)')
    out.append('SCRIPTED = {}')
    out.extend('SCRIPTED[%d] = true' % n for n in scripted)
    out.append('-- behaviours that start with a wait for a flag: a cutscene they start plays once (global.lua, cutscene)')
    out.append('WAITS_FIRST = {}')
    out.extend('WAITS_FIRST[%d] = true' % n for n in waits_first)
    dst = data / 'scripts' / 'generated'
    dst.mkdir(parents=True, exist_ok=True)
    (dst / 'behaviours.lua').write_text('\n'.join(out), encoding='utf8')
    print('%d behaviours -> %s' % (n_scripts, dst / 'behaviours.lua'))


if __name__ == '__main__':
    main()
