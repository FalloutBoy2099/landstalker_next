-- The vocabulary of the story actions (cutscenes). An action is a list of operations the game runs in order, with the player held (boxes of text, fades, flags...);
-- the engine (cutscene.rs) runs them, Lua only says what happens:
--
--   story(7, { say(12), story_flag(30), if_yes({ say(13) }, { say(14) }) })     defines the action 7; a behaviour starts it with cutscene(e, 7)
--   routine(3, { ... })                                                          a routine other actions run with run(3)
--
-- The actions of a room are in rooms/room_NNN.lua with its behaviours, the ones several rooms start in cutscenes/<area>.lua, the shared routines in
-- cutscenes/routines.lua. Texts are i18n keys of data/i18n (`cutscenes.t<id>`): the ids are the numbers of the texts.

STORY = {}      -- the actions by number
ROUTINES = {}   -- the routines by number

function story(n, ops) STORY[n] = ops end
function routine(n, ops) ROUTINES[n] = ops end

-- a box of text (its pages are the text's), `wait` frames it stays up once typed
function say(id, wait) return { text = id, wait = wait } end
-- sets (or clears) a story flag
function story_flag(n, on) return { flag = n, set = on ~= false } end
function play_sound(n) return { sound = n } end
-- the tile swap n of the room happens
function swap_tiles(n) return { swap = n } end
-- runs another action
function run(n) return { call = n } end
-- changes room (dir -1: the way he already faces)
function go_to(room, x, y, dir) return { warp = room, x = x, y = y, dir = dir or -1 } end
-- the player cannot (true) / can (false) act
function lock_controls(on) return { lock = on } end
-- the enemies of this kind become hostile (or not)
function make_hostile(kind, on) return { hostile = kind, set = on ~= false } end
-- shows or hides the entity i of the room
function show_entity(i, on) return { visible = i, set = on ~= false } end
function hide_entity(i) return { hide = i } end
function show_credits() return { credits = true } end
-- a caption of the intro (data/cutscenes/intro_strings.yaml)
function intro_caption(n) return { intro_string = n } end
-- a screen effect: "fade_in", "fade_out", "flash"...
function effect(name) return { fx = name } end
function pause_frames(n) return { wait = n } end
-- what Friday does: "appear", "hide", "hover", "leave"
function friday_does(what) return { friday = what } end
-- blocks cells of the room: { {x, y}, ... }
function block_cells(cells) return { block = cells } end
-- if a story flag is set / if the player answered yes to the last question
function if_flag(n, then_ops, else_ops) return { ["if"] = { flag = n }, ["then"] = then_ops, ["else"] = else_ops } end
function if_yes(then_ops, else_ops) return { ["if"] = { yesno = true }, ["then"] = then_ops, ["else"] = else_ops } end

-- Playbacks: button presses the story replays in place of the player (PlaybackInput): the scenes where Nigel walks by himself. `playback(n)` in a behaviour
-- starts the sequence n; the sequence 9 (nothing pressed for 254 frames) is the pause of the rafts (pause_player).
--
--   playback_seq(4, { press("down+right", 17), press("up+left", 2) })
--
-- Two direction keys together are a diagonal of the screen (up+right north-east, up+left north-west, down+right south-east, down+left south-west).
PLAYBACKS = {}

local PAD = { up = 0x01, down = 0x02, left = 0x04, right = 0x08, b = 0x10, c = 0x20, a = 0x40, start = 0x80 }

-- the pad keys held (a list like "up+right", "" for none) for that many frames (60 per second)
function press(keys, frames)
  local bits = 0
  for k in string.gmatch(keys or "", "[a-z]+") do bits = bits | (PAD[k] or 0) end
  return { buttons = bits, frames = frames }
end

function playback_seq(n, steps) PLAYBACKS[n] = steps end
