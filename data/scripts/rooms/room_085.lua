-- Behaviours of room 85 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[293] = function(e)
  wait_trigger(e, 40)
  pause_player()
  turn(e, 1)
  move_until_collision(e)
  rotate_player(3)
  pause(e, 1)
  cutscene(e, 90)
  e.speed = 2
  playback(30)
  turn(e, 3)
  move_until_collision(e)
  turn(e, 0)
  move_noclip(e, 74)
  turn(e, 1)
  move_noclip(e, 8)
  turn(e, 2)
  pause(e, 24)
  rotate_player(0)
  turn(e, 3)
  move_noclip(e, 8)
  turn(e, 2)
  move_until_collision(e)
  turn(e, 1)
  move_noclip(e, 8)
  turn_wait(e, 0)
  set_flag(8)
  wait_flag(e, 148, true)
  pause(e, 250)
  pause(e, 136)
  e.speed = 1
  turn(e, 1)
  move_noclip(e, 32)
  turn(e, 2)
  move_noclip(e, 32)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[293] = true

B[294] = function(e)
  wait_flag(e, 8, true)
  pause(e, 30)
  move_noclip(e, 16)
  pause(e, 30)
  cutscene(e, 91)
  set_flag(9)
  wait_flag(e, 10, true)
  turn(e, 3)
  pause(e, 60)
  turn(e, 2)
  wait_flag(e, 11, true)
  cutscene(e, 92)
  set_flag(12)
  turn(e, 0)
  move_noclip(e, 16)
  turn(e, 2)
  wait_flag(e, 14, true)
  move_noclip(e, 16)
  turn_wait(e, 3)
  cutscene(e, 96)
  set_flag(15)
  wait_flag(e, 148, true)
  rotate_player(0)
  turn_wait(e, 2)
  pause(e, 1)
  cutscene(e, 97)
  turn(e, 3)
  move_noclip(e, 64)
  rotate_player(3)
  turn(e, 2)
  move_noclip(e, 144)
  rotate_player(2)
  turn(e, 1)
  move_noclip(e, 48)
  turn(e, 2)
  move_noclip(e, 64)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[294] = true
WAITS_FIRST[294] = true

B[295] = function(e)
  wait_flag(e, 9, true)
  move_until_collision(e)
  set_flag(10)
  pause(e, 60)
  turn(e, 3)
  move_wait(e, 96)
  turn(e, 1)
  set_flag(11)
  wait_flag(e, 15, true)
  e.speed = 2
  turn(e, 2)
  move_until_collision(e)
  set_flag(144)
  e.speed = 1
  turn(e, 0)
  move_noclip(e, 32)
  turn(e, 3)
  move_noclip(e, 48)
  clear_flag(144)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[295] = true
WAITS_FIRST[295] = true

B[296] = function(e)
  wait_flag(e, 12, true)
  tile_swap(2)
  move_to(e, 12, 17)
  move_noclip(e, 32)
  turn(e, 2)
  move_noclip(e, 32)
  set_flag(13)
  rotate_player(3)
  turn(e, 1)
  move_noclip(e, 32)
  cutscene(e, 93)
  tile_swap(3)
  turn(e, 3)
  move_noclip(e, 48)
  turn(e, 2)
  move_noclip(e, 56)
  turn_wait(e, 1)
  cutscene(e, 94)
  repeat_begin(e, 18)
  ::L37::
  e.speed = 1
  turn(e, 0)
  move_timed(e, 5)
  turn(e, 1)
  move_timed(e, 15)
  pause(e, 120)
  e.speed = 1
  turn(e, 2)
  move_timed(e, 5)
  turn(e, 1)
  move_timed(e, 15)
  pause(e, 120)
  if repeat_next(e) then goto L37 end
  cutscene(e, 95)
  turn(e, 0)
  e.speed = 1
  move_noclip(e, 40)
  turn(e, 1)
  move_noclip(e, 32)
  set_flag(14)
  wait_flag(e, 144, true)
  tile_swap(2)
  turn(e, 0)
  move_noclip(e, 48)
  turn(e, 3)
  move_noclip(e, 48)
  tile_swap(3)
  set_flag(148)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[296] = true
WAITS_FIRST[296] = true

B[297] = function(e)
  wait_flag(e, 13, true)
  turn(e, 3)
  wait_flag(e, 148, true)
  turn_wait(e, 0)
  pause(e, 0)
end
SCRIPTED[297] = true
WAITS_FIRST[297] = true

B[298] = function(e)
  wait_flag(e, 148, true)
  pause(e, 200)
  move_noclip(e, 16)
  turn(e, 2)
  move_noclip(e, 32)
  turn(e, 1)
  move_noclip(e, 48)
  turn(e, 2)
  move_noclip(e, 64)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[298] = true
WAITS_FIRST[298] = true

B[299] = function(e)
  wait_flag(e, 148, true)
  pause(e, 255)
  pause(e, 8)
  move_noclip(e, 64)
  turn(e, 2)
  move_noclip(e, 64)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[299] = true
WAITS_FIRST[299] = true

B[300] = function(e)
  wait_flag(e, 148, true)
  pause(e, 250)
  pause(e, 36)
  move_timed(e, 20)
  turn(e, 1)
  move_noclip(e, 64)
  turn(e, 2)
  move_noclip(e, 32)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[300] = true
WAITS_FIRST[300] = true

-- story actions
story(90, {
  say(99),
})

story(91, {
  say(100),
})

story(92, {
  say(101),
})

story(93, {
  say(102),
})

story(94, {
  play_sound(65533),
  pause_frames(180),
  play_sound(44),
})

story(95, {
  pause_frames(180),
  play_sound(37),
})

story(96, {
  say(103),
})

story(97, {
  say(104),
})
