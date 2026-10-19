-- Behaviours of room 272 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[714] = function(e)
  ::L0::
  move_wait(e, 64)
  turn(e, e.orientation + 3)
  move_wait(e, 32)
  turn(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[715] = function(e)
  ::L0::
  move_wait(e, 64)
  turn(e, e.orientation + 1)
  move_wait(e, 32)
  turn(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[716] = function(e)
  ::L0::
  move_wait(e, 32)
  turn(e, e.orientation + 3)
  move_wait(e, 32)
  turn(e, e.orientation + 3)
  move_wait(e, 64)
  turn(e, e.orientation + 3)
  move_wait(e, 32)
  turn(e, e.orientation + 3)
  move_wait(e, 32)
  jump_guard(e) goto L0
end

B[717] = function(e)
  ::L0::
  move_down_relative(e, 64)
  pause(e, 20)
  pause(e, 20)
  jump_guard(e) goto L0
end
