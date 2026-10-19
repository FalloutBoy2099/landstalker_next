-- Behaviours of room 652 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[355] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 56)
  set_flag(174)
  pause(e, 0)
end
SCRIPTED[355] = true
WAITS_FIRST[355] = true

B[356] = function(e)
  wait_flag(e, 174, true)
  cutscene(e, 123)
  pause(e, 0)
end
SCRIPTED[356] = true
WAITS_FIRST[356] = true
