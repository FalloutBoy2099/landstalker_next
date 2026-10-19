-- Behaviours of room 201 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[109] = function(e)
  ::L0::
  move_wait(e, 64)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 48)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[111] = function(e)
  ::L0::
  move_wait(e, 48)
  move_up_to(e, 48)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 48)
  move_down_to(e, 16)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[451] = function(e)
  ::L0::
  move_wait(e, 48)
  pause(e, 30)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
