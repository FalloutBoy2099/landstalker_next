-- Behaviours of room 659 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[345] = function(e)
  wait_flag(e, 171, true)
  cutscene(e, 123)
  pause(e, 0)
end
SCRIPTED[345] = true
WAITS_FIRST[345] = true

B[346] = function(e)
  wait_trigger(e, 53)
  set_flag(9)
  pause(e, 0)
end

B[347] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 2)
  wait_flag(e, 9, false)
  set_flag(171)
  pause(e, 0)
end
SCRIPTED[347] = true
WAITS_FIRST[347] = true
