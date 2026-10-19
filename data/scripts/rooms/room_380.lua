-- Behaviours of room 380 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[495] = function(e)
  ::L0::
  move_wait(e, 8)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end
