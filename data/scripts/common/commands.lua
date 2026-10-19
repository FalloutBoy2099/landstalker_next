-- The other commands of the behaviour scripts: orientation, visibility, flags, switches, cutscenes, the player.

---------------------------------------------------------------------------------------------------------------
-- orientation, visibility, flags
---------------------------------------------------------------------------------------------------------------

function turn(e, dir) e.orientation = dir & 3 end
function turn_wait(e, dir) turn(e, dir); tick_wait(e) end

-- 0x45: appears (a gate closing behind the player) once the player is out of its box
function show_when_clear(e)
  if not e.visible then
    while overlaps_player(e) do tick_wait(e) end
  end
  e.visible = true
end

function wait_flag(e, f, want)
  while flag(f) ~= want do tick_wait(e) end
end

function activate_switch(e)
  turn(e, 2)
  sfx(0x6C)   -- SND_Switch (SND_PermanentSwitch, 0x74, is the one of the dialogue actions)
end

function move_to(e, x, y)
  e.x, e.y = x + 0.5, y + 0.5
  if e.gravity then e.z = floor_height(x, y) end
end

function jump_guard(e)
  e.guard = (e.guard or 0) + 1
  if e.guard >= 32 then tick_wait(e) end
end

function repeat_begin(e, n)
  if n ~= 0 then e.repeat_count = n end
end
function repeat_next(e)
  e.repeat_count = (e.repeat_count or 1) - 1
  return e.repeat_count > 0
end

-- not ported yet (the game does nothing, like the C++ engine for most of them)
-- PlaybackInput(n): the player follows a recorded input (the scenes of the story)
function playback(n) request_playback(n) end
-- EB_Pause4s: PlaybackInput(9), the player cannot move for 254 frames (the raft that carries him)
function pause_player() request_pause() end
function rotate_player(dir) P.turn_to = dir & 3 end
-- TriggerTileSwap(n): sets the flag that makes the swap n happen (once)
function tile_swap(n)
  for _, sf in ipairs(swap_flags) do
    if sf.trigger == n and not sf.locked and not flag(sf.flag) then
      set_flag(sf.flag)
      break
    end
  end
end
-- TriggerCutscene(n) (the command 0x4E, n < 256) / StartHiCutscene(p) (0x64, n = 0x100 + p): the story runs an action. A scene that waited for a
-- flag plays once: entering the room again must not start it again (its scripts restart with the room), remembered as a private flag.
function cutscene(e, n)
  if n < 0x100 and WAITS_FIRST[e.behaviour] then
    local played = 4096 + n
    if flag(played) then
      e.co = nil
      while true do coroutine.yield() end
    end
    set_flag(played)
  end
  request_cutscene(e.i, n)
end
