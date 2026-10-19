-- Behaviours of room 90 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[414] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 2)
  set_flag(257)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[414] = true
WAITS_FIRST[414] = true

B[415] = function(e)
  pause_player()
  move_wait(e, 70)
  cutscene(e, 169)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[415] = true

B[416] = function(e)
  move_wait(e, 64)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[416] = true

B[417] = function(e)
  wait_trigger(e, 75)
  set_flag(8)
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[417] = true

-- story actions
story(169, {
  say(151),
})
