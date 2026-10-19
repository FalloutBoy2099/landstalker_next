-- Behaviours of room 382 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[491] = function(e)
  ::L0::
  show_when_clear(e)
  pause(e, 30)
  e.visible = false
  jump_guard(e) goto L0
end

B[492] = function(e)
  ::L0::
  move_wait(e, 28)
  turn(e, e.orientation + 3)
  move_wait(e, 44)
  turn(e, e.orientation + 1)
  move_wait(e, 44)
  turn(e, e.orientation + 1)
  move_wait(e, 52)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 52)
  turn(e, e.orientation + 3)
  move_wait(e, 44)
  turn(e, e.orientation + 3)
  move_wait(e, 44)
  turn(e, e.orientation + 1)
  move_wait(e, 28)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[493] = function(e)
  ::L0::
  move_wait(e, 96)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
