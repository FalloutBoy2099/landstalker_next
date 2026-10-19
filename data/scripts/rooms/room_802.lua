-- Behaviours of room 802 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[738] = function(e)
  pause_player()
  pause(e, 3)
  cutscene(e, 0x100 + 38)
  move_noclip(e, 24)
  cutscene(e, 0x100 + 39)
  turn(e, 0)
  move_wait(e, 30)
  repeat_begin(e, 5)
  ::L15::
  wait_flag(e, 10, true)
  turn_wait(e, e.orientation + 1)
  clear_flag(10)
  if repeat_next(e) then goto L15 end
  wait_flag(e, 9, true)
  cutscene(e, 0x100 + 41)
  set_flag(319)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[738] = true

B[740] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(328)
  pause(e, 0)
end

B[741] = function(e)
  wait_flag(e, 8, true)
  move_down_until_collision(e)
  repeat_begin(e, 5)
  ::L6::
  set_flag(10)
  sfx(102)
  e.speed = 4
  move_up_relative(e, 8)
  e.speed = 8
  move_down_until_collision(e)
  if repeat_next(e) then goto L6 end
  cutscene(e, 0x100 + 40)
  set_flag(9)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[741] = true
WAITS_FIRST[741] = true

B[742] = function(e)
  wait_flag(e, 328, false)
  wait_flag(e, 328, true)
  pause_player()
  move_down_until_collision(e)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[742] = true
WAITS_FIRST[742] = true

B[894] = function(e)
  wait_flag(e, 319, false)
  wait_trigger(e, 1)
  turn_wait(e, 2)
  sfx(68)
  pause(e, 25)
  set_flag(8)
  pause(e, 0)
end
SCRIPTED[894] = true
WAITS_FIRST[894] = true

-- story actions
story(294, {
  say(244),
})

story(295, {
  say(245),
})

story(296, {
  say(246),
})

story(297, {
  story_flag(28, false),
  run(346),
  say(247),
  friday_does("leave"),
})
