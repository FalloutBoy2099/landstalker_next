-- Behaviours of room 729 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[578] = function(e)
  ::L0::
  pause(e, 250)
  pause(e, 250)
  turn(e, e.orientation + 3)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 16)
  turn(e, e.orientation + 3)
  jump_guard(e) goto L0
end
