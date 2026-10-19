-- Behaviours of room 264 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[67] = function(e)
  ::L0::
  pause(e, 20)
  move_wait(e, 192)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[68] = function(e)
  ::L0::
  pause(e, 10)
  move_up_to(e, 80)
  pause(e, 10)
  move_down_to(e, 0)
  jump_guard(e) goto L0
end

B[69] = function(e)
  ::L0::
  move_wait(e, 160)
  turn_wait(e, e.orientation + 2)
  pause(e, 30)
  move_wait(e, 160)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
