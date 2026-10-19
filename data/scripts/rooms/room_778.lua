-- Behaviours of room 778 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[377] = function(e)
  show_when_clear(e)
  wait_trigger(e, 2)
  set_flag(8)
  e.visible = false; e.base_visible = false; do return end
end

B[378] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 63)
  turn(e, 0)
  move_wait(e, 8)
  pause(e, 8)
  set_flag(186)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[378] = true
WAITS_FIRST[378] = true
