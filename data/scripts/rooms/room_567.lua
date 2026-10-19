-- Behaviours of room 567 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[60] = function(e)
  ::L0::
  pause(e, 30)
  move_up_to(e, 112)
  pause(e, 30)
  move_down_to(e, 48)
  jump_guard(e) goto L0
end

B[61] = function(e)
  ::L0::
  move_wait(e, 32)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 48)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[62] = function(e)
  ::L0::
  pause(e, 30)
  move_up_to(e, 96)
  pause(e, 30)
  move_down_to(e, 32)
  jump_guard(e) goto L0
end

B[440] = function(e)
  e.visible = false; e.base_visible = false
  wait_trigger(e, 83)
  pause_player()
  move_to(e, 34, 38)
  cutscene(e, 189)
  move_up_relative(e, 2)
  e.speed = 2
  move_noclip(e, 16)
  turn(e, 2)
  pause(e, 12)
  playback(43)
  move_wait(e, 16)
  pause(e, 18)
  cutscene(e, 190)
  set_flag(276)
  pause(e, 0)
end
SCRIPTED[440] = true

B[441] = function(e)
  e.visible = false; e.base_visible = false
  wait_trigger(e, 83)
  pause_player()
  move_to(e, 34, 38)
  cutscene(e, 191)
  set_flag(277)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[441] = true

B[442] = function(e)
  e.visible = false; e.base_visible = false
  wait_trigger(e, 84)
  pause_player()
  move_to(e, 58, 23)
  cutscene(e, 192)
  set_flag(278)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[442] = true

B[443] = function(e)
  wait_flag(e, 279, false)
  wait_trigger(e, 85)
  cutscene(e, 193)
  set_flag(279)
  pause(e, 0)
end
SCRIPTED[443] = true
WAITS_FIRST[443] = true

B[535] = function(e)
  wait_flag(e, 325, false)
  wait_trigger(e, 100)
  sfx(22)
  pause_player()
  move_to(e, 27, 54)
  move_noclip(e, 96)
  turn_wait(e, 0)
  cutscene(e, 224)
  turn(e, 3)
  move_noclip(e, 96)
  set_flag(325)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[535] = true
WAITS_FIRST[535] = true

B[536] = function(e)
  wait_flag(e, 325, false)
  wait_trigger(e, 100)
  pause(e, 16)
  move_to(e, 27, 53)
  move_noclip(e, 96)
  turn(e, 3)
  move_noclip(e, 96)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[536] = true
WAITS_FIRST[536] = true

B[537] = function(e)
  wait_flag(e, 325, false)
  wait_trigger(e, 100)
  pause(e, 16)
  move_to(e, 28, 53)
  move_noclip(e, 96)
  turn(e, 3)
  move_noclip(e, 96)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[537] = true
WAITS_FIRST[537] = true

-- story actions
story(189, {
  say(174),
})

story(190, {
  say(175),
})

story(191, {
  say(176),
})

story(192, {
  say(177),
})

story(193, {
  say(178),
})

story(224, {
  say(212),
  say(213),
})
