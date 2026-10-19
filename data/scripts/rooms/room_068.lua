-- Behaviours of room 68 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[252] = function(e)
  ::L0::
  move_wait(e, 44)
  turn(e, 0)
  pause(e, 10)
  turn(e, 1)
  e.speed = 4
  move_wait(e, 22)
  turn(e, e.orientation + 1)
  move_wait(e, 4)
  turn(e, e.orientation + 3)
  move_wait(e, 4)
  turn_wait(e, e.orientation + 3)
  pause(e, 10)
  turn(e, e.orientation + 1)
  move_wait(e, 4)
  turn(e, e.orientation + 3)
  pause(e, 10)
  e.speed = 2
  turn_wait(e, e.orientation + 3)
  move_wait(e, 20)
  turn(e, e.orientation + 1)
  move_wait(e, 8)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 24)
  turn(e, e.orientation + 1)
  pause(e, 20)
  turn(e, e.orientation + 1)
  e.speed = 1
  move_wait(e, 24)
  turn(e, e.orientation + 3)
  pause(e, 20)
  turn(e, e.orientation + 1)
  move_wait(e, 16)
  turn(e, e.orientation + 3)
  pause(e, 20)
  e.speed = 2
  turn(e, e.orientation + 1)
  move_wait(e, 8)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
