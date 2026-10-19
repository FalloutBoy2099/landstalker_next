-- Behaviours of room 407 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[768] = function(e)
  cutscene(e, 0x100 + 59)
  ::L2::
  turn_wait(e, 2)
  move_timed(e, 1)
  turn(e, 3)
  move_timed(e, 1)
  jump_guard(e) goto L2
end
SCRIPTED[768] = true
