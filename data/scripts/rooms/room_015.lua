-- Behaviours of room 15 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[97] = function(e)
  wait_flag(e, 2, true)
  move_down_to(e, 120)
  pause(e, 0)
end
SCRIPTED[97] = true
WAITS_FIRST[97] = true
