-- Behaviours of room 115 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[729] = function(e)
  wait_flag(e, 8, true)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[729] = true
WAITS_FIRST[729] = true

B[860] = function(e)
  wait_flag(e, 454, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  pause_player()
  move_down_relative(e, 48)
  move_wait(e, 112)
  pause(e, 10)
  sfx(89)
  tile_swap(1)
  pause(e, 10)
  tile_swap(2)
  pause(e, 10)
  tile_swap(3)
  pause(e, 10)
  tile_swap(4)
  pause(e, 10)
  tile_swap(5)
  pause(e, 10)
  tile_swap(6)
  pause(e, 0)
end
SCRIPTED[860] = true
WAITS_FIRST[860] = true
