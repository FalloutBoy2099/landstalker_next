-- Behaviours of room 408 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[767] = function(e)
  ::L0::
  wait_flag(e, 346, false)
  set_flag(346)
  cutscene(e, 0x100 + 59)
  jump_guard(e) goto L0
  turn_wait(e, 2)
  move_timed(e, 1)
  turn(e, 3)
  move_timed(e, 1)
  jump_guard(e) goto L0
end
SCRIPTED[767] = true
WAITS_FIRST[767] = true
