-- Behaviours that several rooms use for the entities of this kind (sailor).

B[323] = function(e)
  wait_flag(e, 8, true)
  pause_player()
  cutscene(e, 110)
  move_noclip(e, 32)
  set_flag(165)
  pause(e, 0)
end
SCRIPTED[323] = true
WAITS_FIRST[323] = true
