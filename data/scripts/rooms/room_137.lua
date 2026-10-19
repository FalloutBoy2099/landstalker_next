-- Behaviours of room 137 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[758] = function(e)
  ::L0::
  move_wait(e, 80)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 64)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 8)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 56)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 72)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 8)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[759] = function(e)
  ::L0::
  move_wait(e, 56)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 72)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 8)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 80)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 64)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 8)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end
