-- Behaviours of room 136 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[753] = function(e)
  ::L0::
  move_wait(e, 32)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 28)
  turn_wait(e, e.orientation + 1)
  jump_guard(e) goto L0
end
