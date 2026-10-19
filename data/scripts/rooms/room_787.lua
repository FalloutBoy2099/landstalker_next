-- Behaviours of room 787 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[697] = function(e)
  wait_flag(e, 125, true)
  move_down_until_collision(e)
  pause(e, 0)
end
SCRIPTED[697] = true
WAITS_FIRST[697] = true

B[704] = function(e)
  wait_flag(e, 126, true)
  move_down_until_collision(e)
  pause(e, 0)
end
SCRIPTED[704] = true
WAITS_FIRST[704] = true

B[705] = function(e)
  wait_flag(e, 127, true)
  move_down_until_collision(e)
  pause(e, 0)
end
SCRIPTED[705] = true
WAITS_FIRST[705] = true

B[706] = function(e)
  wait_flag(e, 128, true)
  move_down_until_collision(e)
  pause(e, 0)
end
SCRIPTED[706] = true
WAITS_FIRST[706] = true
