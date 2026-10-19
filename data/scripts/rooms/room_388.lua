-- Behaviours of room 388 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[499] = function(e)
  wait_flag(e, 115, true)
  move_down_until_collision(e)
  pause(e, 0)
end
SCRIPTED[499] = true
WAITS_FIRST[499] = true
