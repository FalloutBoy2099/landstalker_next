-- Behaviours of room 654 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[340] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 51)
  set_flag(9)
  pause(e, 0)
end
SCRIPTED[340] = true
WAITS_FIRST[340] = true

B[341] = function(e)
  wait_flag(e, 8, true)
  pause(e, 240)
  pause(e, 240)
  pause(e, 240)
  pause(e, 240)
  wait_flag(e, 9, false)
  cutscene(e, 133)
  set_flag(169)
  pause(e, 0)
end
SCRIPTED[341] = true
WAITS_FIRST[341] = true

B[342] = function(e)
  wait_flag(e, 169, true)
  cutscene(e, 123)
  pause(e, 0)
end
SCRIPTED[342] = true
WAITS_FIRST[342] = true

-- story actions
story(133, {
  hide_entity(0),
  hide_entity(1),
})
