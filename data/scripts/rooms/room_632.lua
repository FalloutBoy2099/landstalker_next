-- Behaviours of room 632 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[791] = function(e)
  wait_flag(e, 341, true)
  turn(e, 2)
  ::L4::
  move_timed(e, 16)
  pause(e, 64)
  turn_wait(e, math.random(0, 3))
  jump_guard(e) goto L4
end
SCRIPTED[791] = true
WAITS_FIRST[791] = true
