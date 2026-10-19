-- Behaviours of room 177 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[317] = function(e)
  wait_flag(e, 0, true)
  move_wait(e, 16)
  pause(e, 0)
end
SCRIPTED[317] = true
WAITS_FIRST[317] = true
