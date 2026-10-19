-- Behaviours of room 22 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[214] = function(e)
  ::L0::
  wait_flag(e, 9, true)
  move_down_until_collision(e)
  wait_flag(e, 9, false)
  jump_guard(e) goto L0
end
SCRIPTED[214] = true
WAITS_FIRST[214] = true
