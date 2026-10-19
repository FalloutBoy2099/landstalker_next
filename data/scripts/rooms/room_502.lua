-- Behaviours of room 502 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[570] = function(e)
  ::L0::
  move_wait(e, 24)
  turn(e, e.orientation + 1)
  move_timed(e, 32)
  turn(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[571] = function(e)
  ::L0::
  move_wait(e, 64)
  move_down_relative(e, 64)
  turn(e, e.orientation + 2)
  move_wait(e, 64)
  move_up_relative(e, 64)
  turn(e, e.orientation + 2)
  jump_guard(e) goto L0
end
