-- Behaviours of room 142 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[680] = function(e)
  e.visible = false; e.base_visible = false
  wait_flag(e, 9, true)
  repeat_begin(e, 80)
  ::L6::
  move_noclip(e, 1)
  move_up_relative(e, 1)
  if repeat_next(e) then goto L6 end
  turn(e, 0)
  repeat_begin(e, 7)
  ::L14::
  move_noclip(e, 16)
  if repeat_next(e) then goto L14 end
  turn(e, 1)
  move_noclip(e, 16)
  move_down_until_collision(e)
  set_flag(10)
  pause(e, 0)
end
SCRIPTED[680] = true

B[681] = function(e)
  move_until_collision(e)
  turn(e, 3)
  move_until_collision(e)
  set_flag(8)
  pause(e, 0)
end

B[682] = function(e)
  pause_player()
  turn_wait(e, 2)
  wait_flag(e, 8, true)
  turn(e, 3)
  move_wait(e, 16)
  turn(e, 2)
  move_noclip(e, 32)
  move_noclip(e, 40)
  playback(50)
  turn(e, 3)
  move_noclip(e, 216)
  move_noclip(e, 216)
  turn(e, 0)
  move_noclip(e, 52)
  set_flag(9)
  wait_flag(e, 10, true)
  move_to(e, 27, 21)
  cutscene(e, 0x100 + 16)
  turn(e, 2)
  move_noclip(e, 96)
  turn(e, 3)
  playback(51)
  move_noclip(e, 32)
  turn_wait(e, 1)
  pause(e, 30)
  cutscene(e, 0x100 + 17)
  pause(e, 0)
end
SCRIPTED[682] = true

B[684] = function(e)
  wait_flag(e, 11, true)
  sfx(22)
  move_noclip(e, 224)
  pause(e, 5)
  rotate_player(2)
  turn_wait(e, 0)
  cutscene(e, 0x100 + 20)
  e.speed = 2
  pause(e, 8)
  turn(e, 3)
  move_noclip(e, 32)
  turn(e, 0)
  move_noclip(e, 32)
  pause(e, 32)
  cutscene(e, 0x100 + 21)
  playback(52)
  pause(e, 240)
  move_to(e, 42, 32)
  turn(e, 2)
  move_noclip(e, 48)
  cutscene(e, 0x100 + 22)
  move_noclip(e, 128)
  rotate_player(1)
  cutscene(e, 0x100 + 18)
  pause(e, 0)
end
SCRIPTED[684] = true
WAITS_FIRST[684] = true

B[685] = function(e)
  wait_flag(e, 11, true)
  move_noclip(e, 224)
  pause(e, 6)
  e.speed = 2
  turn(e, 2)
  move_noclip(e, 8)
  pause(e, 12)
  turn(e, 0)
  move_noclip(e, 8)
  turn(e, 3)
  move_noclip(e, 28)
  turn(e, 0)
  move_noclip(e, 32)
  pause(e, 240)
  pause(e, 24)
  move_to(e, 42, 31)
  turn(e, 2)
  move_noclip(e, 192)
  pause(e, 0)
end
SCRIPTED[685] = true
WAITS_FIRST[685] = true

B[718] = function(e)
  wait_flag(e, 11, true)
  move_noclip(e, 224)
  pause(e, 6)
  e.speed = 2
  pause(e, 28)
  turn(e, 3)
  move_noclip(e, 24)
  turn(e, 0)
  move_noclip(e, 20)
  pause(e, 240)
  pause(e, 32)
  move_to(e, 43, 31)
  turn(e, 2)
  move_noclip(e, 192)
  pause(e, 0)
end
SCRIPTED[718] = true
WAITS_FIRST[718] = true

B[720] = function(e)
  pause(e, 30)
  cutscene(e, 0x100 + 26)
  pause(e, 240)
  pause(e, 240)
  pause(e, 90)
  cutscene(e, 0x100 + 32)
  pause(e, 240)
  cutscene(e, 0x100 + 31)
  pause(e, 0)
end
SCRIPTED[720] = true

-- story actions
story(273, {
  say(253, 180),
  say(254, 1),
  say(255, 180),
  say(256, 180),
  play_sound(15),
  friday_does("hover"),
  friday_does("hide"),
  say(257, 60),
  pause_frames(30),
  run(346),
  say(258, 60),
  say(259, 60),
  story_flag(11, true),
})

story(274, {
  run(346),
  say(270, 90),
  say(271, 180),
  say(272, 120),
  say(273, 180),
  say(274, 60),
  say(275, 180),
  say(276, 180),
  say(277, 90),
  say(278, 180),
  say(279, 180),
  say(280, 180),
  friday_does("hover"),
  effect("fade_out"),
  friday_does("hover"),
  go_to(143, 33, 25, 2),
  lock_controls(true),
})

story(276, {
  say(260, 180),
  say(261, 60),
  say(262, 60),
  say(263, 180),
  say(264, 180),
  say(265, 60),
  say(266, 180),
})

story(277, {
  say(267, 60),
  friday_does("leave"),
  say(268, 120),
  play_sound(17),
})

story(278, {
  say(269, 90),
})

story(282, {
  intro_caption(5),
})

story(287, {
  intro_caption(9),
})

story(288, {
  intro_caption(10),
})
