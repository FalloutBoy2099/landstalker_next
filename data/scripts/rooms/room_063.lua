-- Behaviours of room 63 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[260] = function(e)
  wait_flag(e, 160, false)
  pause(e, 5)
  move_wait(e, 16)
  pause(e, 0)
end
SCRIPTED[260] = true
WAITS_FIRST[260] = true
