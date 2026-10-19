-- Behaviours of room 263 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[71] = function(e)
  ::L0::
  move_up_to(e, 32)
  pause(e, 60)
  move_down_to(e, 32)
  pause(e, 60)
  jump_guard(e) goto L0
end
