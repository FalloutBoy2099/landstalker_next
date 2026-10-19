-- Behaviours of room 657 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[351] = function(e)
  wait_flag(e, 9, true)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end
SCRIPTED[351] = true
WAITS_FIRST[351] = true

B[352] = function(e)
  wait_trigger(e, 55)
  set_flag(8)
  pause(e, 0)
end

B[353] = function(e)
  wait_flag(e, 9, true)
  wait_flag(e, 173, false)
  pause(e, 240)
  pause(e, 240)
  pause(e, 240)
  pause(e, 240)
  wait_flag(e, 8, false)
  set_flag(173)
  pause(e, 0)
end
SCRIPTED[353] = true
WAITS_FIRST[353] = true

B[354] = function(e)
  wait_flag(e, 173, true)
  cutscene(e, 123)
  pause(e, 0)
end
SCRIPTED[354] = true
WAITS_FIRST[354] = true
