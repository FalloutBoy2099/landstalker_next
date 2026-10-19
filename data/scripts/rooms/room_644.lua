-- Behaviours of room 644 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[511] = function(e)
  wait_flag(e, 348, true)
  turn(e, 3)
  move_wait(e, 4)
  turn_wait(e, 1)
  pause(e, 0)
end
SCRIPTED[511] = true
WAITS_FIRST[511] = true
