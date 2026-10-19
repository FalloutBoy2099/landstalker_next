-- Behaviours of room 253 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[65] = function(e)
  ::L0::
  move_up_to(e, 160)
  move_down_to(e, 80)
  jump_guard(e) goto L0
end

B[66] = function(e)
  ::L0::
  move_down_to(e, 0)
  move_up_to(e, 80)
  jump_guard(e) goto L0
end
