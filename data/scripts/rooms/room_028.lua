-- Behaviours of room 28 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[129] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_flag(e, 21, true)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[129] = true

B[130] = function(e)
  wait_trigger(e, 13)
  pause_player()
  cutscene(e, 9)
  set_flag(10)
  move_noclip(e, 56)
  set_flag(11)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[130] = true

B[131] = function(e)
  wait_flag(e, 10, true)
  cutscene(e, 10)
  move_noclip(e, 56)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[131] = true
WAITS_FIRST[131] = true

B[132] = function(e)
  e.hostile = false
  wait_flag(e, 11, true)
  move_noclip(e, 128)
  cutscene(e, 11)
  e.hostile = true; e.dormant = false
  set_flag(12)
  do return end  -- unknown command 26
end
SCRIPTED[132] = true

B[133] = function(e)
  e.hostile = false
  wait_flag(e, 11, true)
  move_noclip(e, 128)
  e.hostile = true; e.dormant = false
  do return end  -- unknown command 26
end
SCRIPTED[133] = true

B[134] = function(e)
  wait_flag(e, 12, true)
  wait_trigger(e, 2)
  wait_trigger(e, 14)
  cutscene(e, 12)
  set_flag(21)
  turn(e, 3)
  e.speed = 2
  e.falls = true; e.fall_v = 0
  pause_player()
  move_noclip(e, 96)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[134] = true
WAITS_FIRST[134] = true

B[915] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  sfx(84)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end

-- story actions
story(9, {
  say(18),
  say(19),
})

story(11, {
  say(20),
})

story(12, {
  say(21),
})
