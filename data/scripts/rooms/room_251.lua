-- Behaviours of room 251 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[64] = function(e)
  ::L0::
  move_wait(e, 96)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[612] = function(e)
  ::L0::
  pause(e, 20)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[613] = function(e)
  pause(e, 42)
  ::L2::
  pause(e, 20)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 2)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L2
end
