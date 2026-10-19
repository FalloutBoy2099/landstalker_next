-- Behaviours that several rooms use for the entities of this kind (raft).

B[38] = function(e)
  pause_player()
  turn_wait(e, 1)
  move_wait(e, 120)
  e.speed = 4
  move_wait(e, 192)
  pause(e, 0)
end

B[143] = function(e)
  cutscene(e, 27)
  move_noclip(e, 48)
  cutscene(e, 23)
  wait_flag(e, 8, true)
  turn(e, 2)
  move_noclip(e, 128)
end
SCRIPTED[143] = true

B[146] = function(e)
  wait_flag(e, 8, true)
  move_noclip(e, 128)
end
SCRIPTED[146] = true
WAITS_FIRST[146] = true
