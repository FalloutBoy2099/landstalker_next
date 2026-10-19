-- Behaviours that several rooms use for the entities of this kind (golem statue).

B[454] = function(e)
  wait_flag(e, 8, true)
  move_until_collision(e)
  pause(e, 0)
end
SCRIPTED[454] = true
WAITS_FIRST[454] = true
