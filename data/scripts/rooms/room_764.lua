-- Behaviours of room 764 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[209] = function(e)
  ::L0::
  move_up_relative(e, 16)
  pause(e, 20)
  move_down_relative(e, 128)
  move_up_relative(e, 112)
  jump_guard(e) goto L0
end
