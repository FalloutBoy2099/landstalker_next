-- Behaviours of room 53 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[396] = function(e)
  cutscene(e, 157)
  move_noclip(e, 112)
  move_noclip(e, 44)
  turn(e, 3)
  move_noclip(e, 28)
  turn(e, 1)
  wait_trigger(e, 72)
  rotate_player(1)
  set_flag(8)
  wait_flag(e, 9, true)
  turn(e, 0)
  move_noclip(e, 8)
  turn(e, 1)
  pause(e, 0)
end
SCRIPTED[396] = true

B[398] = function(e)
  wait_flag(e, 8, true)
  pause(e, 60)
  move_noclip(e, 48)
  turn(e, 3)
  pause(e, 30)
  cutscene(e, 158)
  set_flag(9)
  wait_flag(e, 10, true)
  move_noclip(e, 16)
  pause(e, 5)
  cutscene(e, 159)
  turn(e, 2)
  move_noclip(e, 160)
  set_flag(11)
  wait_flag(e, 12, true)
  pause(e, 60)
  turn(e, 0)
  move_noclip(e, 96)
  move_noclip(e, 32)
  pause(e, 2)
  cutscene(e, 162)
  set_flag(13)
  wait_flag(e, 14, true)
  pause(e, 15)
  cutscene(e, 163)
  set_flag(15)
  set_flag(152)
  turn(e, 2)
  move_noclip(e, 128)
  pause(e, 0)
end
SCRIPTED[398] = true
WAITS_FIRST[398] = true

B[399] = function(e)
  wait_flag(e, 9, true)
  turn(e, 0)
  move_noclip(e, 24)
  turn(e, 1)
  wait_flag(e, 13, true)
  turn(e, 2)
  wait_flag(e, 14, true)
  turn(e, 1)
  wait_flag(e, 15, true)
  turn(e, 2)
  move_noclip(e, 4)
  playback(37)
  turn(e, 1)
  move_wait(e, 8)
  pause(e, 0)
end
SCRIPTED[399] = true
WAITS_FIRST[399] = true

B[400] = function(e)
  wait_flag(e, 9, true)
  turn(e, 0)
  move_noclip(e, 24)
  turn(e, 1)
  wait_flag(e, 13, true)
  turn(e, 0)
  wait_flag(e, 14, true)
  turn(e, 1)
  wait_flag(e, 15, true)
  turn(e, 0)
  move_noclip(e, 4)
  turn(e, 1)
  move_wait(e, 8)
  pause(e, 0)
end
SCRIPTED[400] = true
WAITS_FIRST[400] = true

B[401] = function(e)
  wait_flag(e, 9, true)
  turn(e, 0)
  move_noclip(e, 10)
  turn(e, 3)
  move_noclip(e, 56)
  turn(e, 0)
  move_noclip(e, 18)
  turn(e, 3)
  wait_flag(e, 13, true)
  turn(e, 2)
  wait_flag(e, 14, true)
  turn(e, 1)
  wait_flag(e, 15, true)
  turn(e, 2)
  pause(e, 0)
end
SCRIPTED[401] = true
WAITS_FIRST[401] = true

B[402] = function(e)
  wait_flag(e, 9, true)
  move_noclip(e, 56)
  turn(e, 0)
  move_noclip(e, 24)
  turn(e, 3)
  set_flag(10)
  wait_flag(e, 13, true)
  turn(e, 0)
  pause(e, 40)
  turn(e, 1)
  set_flag(14)
  wait_flag(e, 15, true)
  turn(e, 0)
  pause(e, 0)
end
SCRIPTED[402] = true
WAITS_FIRST[402] = true

B[403] = function(e)
  wait_flag(e, 11, true)
  sfx(22)
  move_to(e, 30, 38)
  move_noclip(e, 96)
  move_noclip(e, 64)
  turn(e, 3)
  move_noclip(e, 16)
  pause(e, 60)
  cutscene(e, 160)
  sfx(88)
  tile_swap(1)
  cutscene(e, 161)
  set_flag(12)
  move_noclip(e, 30)
  sfx(84)
  e.falls = true; e.fall_v = 0
  pause(e, 30)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[403] = true
WAITS_FIRST[403] = true

B[404] = function(e)
  wait_flag(e, 11, true)
  move_to(e, 30, 40)
  move_noclip(e, 128)
  move_noclip(e, 80)
  turn(e, 3)
  wait_flag(e, 12, true)
  move_noclip(e, 30)
  e.falls = true; e.fall_v = 0
  pause(e, 30)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[404] = true
WAITS_FIRST[404] = true

B[405] = function(e)
  wait_flag(e, 11, true)
  move_to(e, 30, 41)
  move_noclip(e, 144)
  move_noclip(e, 64)
  turn(e, 3)
  wait_flag(e, 12, true)
  move_noclip(e, 30)
  e.falls = true; e.fall_v = 0
  pause(e, 30)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[405] = true
WAITS_FIRST[405] = true

-- story actions
story(158, {
  say(139),
})

story(159, {
  say(140),
})

story(160, {
  say(141),
  say(142),
})

story(161, {
  say(143),
})

story(162, {
  say(144),
})

story(163, {
  say(145),
})
