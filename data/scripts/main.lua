-- landstalker_next: scripting entry point (loaded by the game, see scripting.rs). Lua holds the behaviours of the entities; the engine holds the rest.
--
--   common/      what every script shares: movement and timing commands, the other commands, triggers, the runner (start_room / tick), the enemies' AI,
--                projectiles and the boss dispatcher
--   behaviors/   one file per kind of object, creature or character that several rooms share (raft.lua, gate_s.lua, ifrit.lua ...), common.lua for what
--                several kinds share
--   cutscenes/   the story actions several rooms start, and the routines they share
--   rooms/       room_NNN.lua: the behaviours and the story actions of one room: its scenes, its own platforms and gates
--
-- A behaviour is `B[n] = function(e) ... end`, a coroutine run once per script tick (60 per second, one per frame of the console): `yield`
-- (tick_wait) ends the tick, the commands that "carry on" do not. n is the number the room file gives to the entity (`behaviour: n`).
--
-- Entities (`ents`, filled by the game when a room is loaded) are plain tables:
--   i (0-based index in the room), kind, x, y, z (cells / height units), orientation (0 NE, 1 SE, 2 SW, 3 NW), speed (1/16
--   cell per tick), visible, base_visible, hostile, gravity, solid, falls, fall_v, behaviour, hbase, hheight, killable.
-- The game provides: floor_height(cx, cy), step(e, noclip), overlaps_player(e), trigger_program(n), flag(n), set_flag(n),
-- clear_flag(n), sfx(n), t(key), and the table P (the player: x, y, z, facing, grounded, support).

B = {}          -- the behaviours by number
SCRIPTED = {}   -- behaviours that wait for a story event or turn their entity hostile: an enemy with such a script sleeps until the script wakes it
WAITS_FIRST = {} -- behaviours that start with a wait for a flag: a cutscene they start plays once (common/commands.lua, cutscene)

dofile_relative("common/movement.lua")
dofile_relative("common/commands.lua")
dofile_relative("common/triggers.lua")
dofile_relative("common/enemy_ai.lua")
dofile_relative("common/projectiles.lua")
dofile_relative("common/boss.lua")
dofile_relative("common/runner.lua")
dofile_relative("common/cutscenes.lua")
dofile_dir("behaviors")
dofile_dir("cutscenes")
dofile_dir("rooms")
