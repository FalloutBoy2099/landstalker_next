-- Behaviours of room 653 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[343] = function(e)
  wait_trigger(e, 52)
  set_flag(170)
  pause(e, 0)
end

B[344] = function(e)
  wait_flag(e, 170, true)
  cutscene(e, 123)
  pause(e, 0)
end
SCRIPTED[344] = true
WAITS_FIRST[344] = true
