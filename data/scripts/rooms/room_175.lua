-- Behaviours of room 175 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[168] = function(e)
  wait_flag(e, 37, true)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 24)
  turn_wait(e, e.orientation + 3)
  pause(e, 0)
end
SCRIPTED[168] = true
WAITS_FIRST[168] = true

B[319] = function(e)
  wait_flag(e, 37, true)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 24)
  turn_wait(e, e.orientation + 1)
  pause(e, 0)
end
SCRIPTED[319] = true
WAITS_FIRST[319] = true
