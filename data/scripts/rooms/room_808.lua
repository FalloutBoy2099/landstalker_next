-- Behaviours of room 808 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[534] = function(e)
  wait_flag(e, 119, true)
  move_up_to(e, 64)
  pause(e, 0)
end
SCRIPTED[534] = true
WAITS_FIRST[534] = true
