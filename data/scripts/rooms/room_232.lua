-- Behaviours of room 232 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[73] = function(e)
  ::L0::
  move_wait(e, 120)
  pause(e, 30)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
