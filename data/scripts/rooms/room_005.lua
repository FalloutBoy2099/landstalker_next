-- Behaviours of room 5 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[50] = function(e)
  ::L0::
  move_wait(e, 128)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[730] = function(e)
  wait_flag(e, 2, true)
  move_down_to(e, 88)
  pause(e, 0)
end
SCRIPTED[730] = true
WAITS_FIRST[730] = true
