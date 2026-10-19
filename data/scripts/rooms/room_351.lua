-- Behaviours of room 351 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[315] = function(e)
  wait_flag(e, 8, true)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 32)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[315] = true
WAITS_FIRST[315] = true
