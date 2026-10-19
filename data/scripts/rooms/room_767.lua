-- Behaviours of room 767 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[184] = function(e)
  move_down_until_collision(e)
  move_wait(e, 4)
  turn(e, e.orientation + 3)
  move_wait(e, 20)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 8)
  ::L9::
  turn_wait(e, e.orientation + 1)
  move_wait(e, 4)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 28)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 4)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 28)
  jump_guard(e) goto L9
end
