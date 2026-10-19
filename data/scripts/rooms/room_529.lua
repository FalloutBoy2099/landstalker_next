-- Behaviours of room 529 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[577] = function(e)
  ::L0::
  move_up_relative(e, 24)
  pause(e, 20)
  move_down_relative(e, 32)
  pause(e, 168)
  move_up_relative(e, 8)
  jump_guard(e) goto L0
end
