-- Behaviours of room 648 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[502] = function(e)
  wait_trigger(e, 1)
  set_flag(175)
  pause(e, 0)
end

B[503] = function(e)
  wait_flag(e, 8, true)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end
SCRIPTED[503] = true
WAITS_FIRST[503] = true

B[504] = function(e)
  wait_flag(e, 175, true)
  pause(e, 3)
  cutscene(e, 123)
  pause(e, 0)
end
SCRIPTED[504] = true
WAITS_FIRST[504] = true
