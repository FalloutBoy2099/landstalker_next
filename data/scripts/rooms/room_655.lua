-- Behaviours of room 655 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[338] = function(e)
  wait_trigger(e, 48)
  wait_trigger(e, 49)
  wait_trigger(e, 50)
  set_flag(168)
  pause(e, 0)
end

B[339] = function(e)
  wait_flag(e, 168, true)
  cutscene(e, 123)
  pause(e, 0)
end
SCRIPTED[339] = true
WAITS_FIRST[339] = true
