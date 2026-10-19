-- Behaviours of room 280 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[713] = function(e)
  ::L0::
  move_wait(e, 64)
  move_down_relative(e, 80)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 64)
  move_up_relative(e, 80)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
