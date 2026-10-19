-- Behaviours of room 663 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[749] = function(e)
  wait_flag(e, 331, true)
  turn(e, 3)
  move_until_collision(e)
  turn_wait(e, 1)
  pause(e, 0)
end
SCRIPTED[749] = true
WAITS_FIRST[749] = true
