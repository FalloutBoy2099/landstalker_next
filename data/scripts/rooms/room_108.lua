-- Behaviours of room 108 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[425] = function(e)
  playback(38)
  pause(e, 12)
  cutscene(e, 173)
  turn(e, 3)
  set_flag(8)
  wait_flag(e, 9, true)
  turn(e, 0)
  pause(e, 2)
  cutscene(e, 175)
  turn(e, 3)
  set_flag(10)
  move_noclip(e, 16)
  cutscene(e, 176)
  move_noclip(e, 16)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[425] = true

B[426] = function(e)
  wait_flag(e, 8, true)
  turn(e, 0)
  move_noclip(e, 8)
  turn(e, 1)
  move_noclip(e, 8)
  pause(e, 2)
  cutscene(e, 174)
  set_flag(9)
  wait_flag(e, 10, true)
  e.speed = 1
  turn(e, 2)
  move_noclip(e, 16)
  turn(e, 3)
  move_noclip(e, 32)
  set_flag(260)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[426] = true
WAITS_FIRST[426] = true

B[427] = function(e)
  wait_flag(e, 260, true)
  ::L3::
  turn(e, 1)
  wait_trigger(e, 78)
  pause_player()
  cutscene(e, 176)
  move_to(e, 12, 19)
  move_noclip(e, 24)
  cutscene(e, 177)
  set_flag(11)
  turn(e, 3)
  playback(39)
  wait_flag(e, 12, true)
  e.speed = 1
  move_noclip(e, 48)
  move_to(e, 13, 2)
  e.speed = 2
  clear_flag(11)
  clear_flag(12)
  jump_guard(e) goto L3
end
SCRIPTED[427] = true
WAITS_FIRST[427] = true

B[428] = function(e)
  ::L0::
  wait_flag(e, 11, true)
  move_to(e, 12, 19)
  e.speed = 2
  move_noclip(e, 16)
  set_flag(12)
  e.speed = 1
  turn(e, 3)
  move_noclip(e, 32)
  move_to(e, 14, 3)
  turn(e, 1)
  wait_flag(e, 11, false)
  jump_guard(e) goto L0
end
SCRIPTED[428] = true
WAITS_FIRST[428] = true

-- story actions
story(173, {
  say(153),
})

story(174, {
  say(154),
})

story(175, {
  say(155),
})

story(177, {
  say(156),
})
