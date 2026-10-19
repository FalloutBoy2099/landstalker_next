-- Behaviours of room 651 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[361] = function(e)
  wait_trigger(e, 58)
  set_flag(166)
  pause(e, 0)
end

B[555] = function(e)
  wait_flag(e, 166, false)
  show_when_clear(e)
  set_flag(8)
  wait_flag(e, 166, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[555] = true
WAITS_FIRST[555] = true

B[556] = function(e)
  e.visible = false; e.base_visible = false
  wait_flag(e, 8, true)
  pause_player()
  move_to(e, 21, 24)
  cutscene(e, 232)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[556] = true

B[557] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end

-- story actions
story(20, {
  say(25),
  story_flag(8, true),
})

story(232, {
  say(211),
})
