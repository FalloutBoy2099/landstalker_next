-- Behaviours of room 197 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[240] = function(e)
  ::L0::
  pause(e, 20)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
