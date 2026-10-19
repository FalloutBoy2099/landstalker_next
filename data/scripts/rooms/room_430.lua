-- Behaviours of room 430 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[221] = function(e)
  ::L0::
  move_down_relative(e, 32)
  move_up_relative(e, 32)
  jump_guard(e) goto L0
end
