-- Behaviours of room 240 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[548] = function(e)
  ::L0::
  move_wait(e, 42)
  turn(e, e.orientation + 3)
  move_wait(e, 22)
  turn(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[549] = function(e)
  ::L0::
  move_wait(e, 20)
  turn(e, e.orientation + 1)
  move_wait(e, 48)
  turn(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[550] = function(e)
  ::L0::
  move_wait(e, 28)
  turn(e, e.orientation + 1)
  move_wait(e, 40)
  turn(e, e.orientation + 1)
  jump_guard(e) goto L0
end
