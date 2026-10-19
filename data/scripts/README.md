# Scripts (Lua)

```
scripts/
  global.lua                 the standard library: one Lua function per command of the original behaviour bytecode,
                             the triggers, the runner (start_room / tick). Hand-written.
  generated/behaviours.lua   the 915 sprite behaviours of the original, translated by tools/behaviours_to_lua.py.
                             Never edit: regenerate with `python tools/behaviours_to_lua.py`.
  ai.lua                     the enemies' AI (patrol, sight, chase, lunge, float, hop...), the hurt / attack poses, what the
                             dead leave behind. Called by tick() for every awake hostile enemy (not by their script).
  objects/ critters/ rooms/  (to come) hand-written scripts replacing a generated behaviour: set `script_fn` of an
                             entity table to a function(e) that yields (`tick_wait(e)`) once per script tick.
```

A script is a coroutine run once per script tick (60 per second, one per frame of the console). The entity `e` is a table: `i, kind, x, y, z,
orientation (0 NE, 1 SE, 2 SW, 3 NW), speed, visible, hostile, gravity, solid, falls, hbase, hheight ...`.
Everything the game gives to Lua: `step(e)`, `floor_height(cx, cy)`, `overlaps_player(e)`, `trigger_program(n)`,
`flag(n)`, `set_flag(n)`, `clear_flag(n)`, `sfx(n)`, `log(text)` and the player `P` (x, y, z, facing, grounded, support).
`t(key)` gives the text of an i18n key (`data/i18n/<language>/`): scripts only handle keys, never literal texts. `camera_focus(i)` centres the camera on entity `i` (0-based, `e.i`) and `camera_focus(-1)` gives it back to Nigel: it glides from one to the other. A story action started by a dormant enemy (a boss waking up) looks at it by itself, until the action is over. A room change resets it.

Debug: `LS_LOG_SCRIPT=1` prints flags and sounds, `LS_ON=n` pretends the player stands on entity n.

Enemies: a hostile entity with the stats of a fighter is driven by `ai_tick(e)` (ai.lua) instead of its script, unless the
script waits for a story event (SCRIPTED in generated/behaviours.lua): it then sleeps (`dormant`) until the script wakes it
(MakeHostile / LoadSpecialAI). The damage is done by combat.rs.

Bosses: `boss_tick(e)` (ai.lua) replaces the generic AI of Ifrit, Gola, Nole, Zak, Mir and the spinners; their fireballs, axes and boomerangs are rows
of the `ents` table marked `slot` (the last PROJECTILE_SLOTS rows, hidden until `spawn_projectile` uses one). `LS_WAKE=1` wakes the dormant enemies (test aid).
