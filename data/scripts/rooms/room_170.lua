-- Behaviours of room 170 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[642] = function(e)
  pause(e, 15)
  cutscene(e, 255)
  playback(46)
  turn(e, 0)
  move_wait(e, 16)
  turn(e, 3)
  move_until_collision(e)
  turn(e, 0)
  move_wait(e, 56)
  turn(e, 3)
  move_until_collision(e)
  turn(e, 0)
  move_until_collision(e)
  turn_wait(e, 2)
  cutscene(e, 0x100 + 0)
  turn_wait(e, 1)
  cutscene(e, 0x100 + 1)
  set_flag(8)
  wait_flag(e, 9, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[642] = true

B[643] = function(e)
  wait_flag(e, 8, true)
  turn_wait(e, 1)
  cutscene(e, 0x100 + 2)
  turn_wait(e, 3)
  cutscene(e, 0x100 + 3)
  set_flag(9)
  pause(e, 2)
  cutscene(e, 0x100 + 6)
  turn(e, 2)
  move_wait(e, 24)
  turn_wait(e, 3)
  rotate_player(1)
  cutscene(e, 0x100 + 4)
  repeat_begin(e, 2)
  ::L26::
  pause(e, 32)
  if repeat_next(e) then goto L26 end
  cutscene(e, 0x100 + 5)
  set_flag(10)
  wait_flag(e, 11, true)
  e.speed = 1
  turn(e, 1)
  move_wait(e, 16)
  turn_wait(e, 2)
  pause(e, 4)
  turn_wait(e, 0)
  wait_flag(e, 12, true)
  pause(e, 48)
  turn(e, 2)
  move_wait(e, 16)
  turn(e, 1)
  move_until_collision(e)
  turn(e, 2)
  move_until_collision(e)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[643] = true
WAITS_FIRST[643] = true

B[644] = function(e)
  wait_flag(e, 10, true)
  move_to(e, 34, 37)
  move_until_collision(e)
  turn(e, 3)
  move_wait(e, 56)
  turn(e, 0)
  move_wait(e, 24)
  turn(e, 3)
  move_wait(e, 8)
  set_flag(11)
  e.speed = 1
  turn(e, 1)
  move_wait(e, 16)
  turn_wait(e, 2)
  pause(e, 2)
  cutscene(e, 0x100 + 7)
  wait_flag(e, 12, true)
  pause(e, 64)
  move_wait(e, 32)
  turn(e, 1)
  move_until_collision(e)
  turn(e, 2)
  move_until_collision(e)
  cutscene(e, 0x100 + 9)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[644] = true
WAITS_FIRST[644] = true

B[645] = function(e)
  wait_flag(e, 10, true)
  pause(e, 16)
  move_to(e, 34, 37)
  move_until_collision(e)
  turn(e, 3)
  move_wait(e, 56)
  turn(e, 0)
  move_wait(e, 8)
  turn(e, 3)
  move_wait(e, 8)
  e.speed = 1
  turn(e, 1)
  move_wait(e, 16)
  turn_wait(e, 0)
  pause(e, 4)
  turn(e, 3)
  move_wait(e, 32)
  rotate_player(2)
  turn_wait(e, 0)
  cutscene(e, 0x100 + 8)
  set_flag(12)
  turn(e, 1)
  move_until_collision(e)
  turn(e, 2)
  move_until_collision(e)
  set_flag(313)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[645] = true
WAITS_FIRST[645] = true

-- story actions
story(255, {
  say(229),
})

story(256, {
  say(230),
})

story(257, {
  say(231),
})

story(258, {
  say(232),
})

story(259, {
  say(233),
  effect("nole_warp"),
})

story(260, {
  say(235),
})

story(261, {
  say(236),
  run(346),
  say(237),
  friday_does("hover"),
  say(238),
  friday_does("hover"),
  say(239),
})

story(262, {
  effect("nole_warp"),
  say(234),
})

story(263, {
  say(240),
})

story(264, {
  say(241),
})

story(265, {
  say(242),
  friday_does("leave"),
})
