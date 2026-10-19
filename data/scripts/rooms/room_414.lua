-- Behaviours of room 414 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[750] = function(e)
  wait_flag(e, 332, false)
  wait_flag(e, 332, true)
  move_noclip(e, 16)
  turn(e, 1)
  move_noclip(e, 16)
  turn_wait(e, 3)
  pause(e, 0)
end
SCRIPTED[750] = true
WAITS_FIRST[750] = true
