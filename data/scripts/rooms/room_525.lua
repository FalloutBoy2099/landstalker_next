-- Behaviours of room 525 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[560] = function(e)
  wait_flag(e, 327, true)
  wait_flag(e, 336, false)
  wait_trigger(e, 104)
  pause_player()
  rotate_player(0)
  pause(e, 30)
  cutscene(e, 235)
  move_to(e, 31, 36)
  set_flag(8)
  move_wait(e, 128)
  set_flag(9)
  turn(e, 1)
  move_wait(e, 160)
  pause(e, 0)
end
SCRIPTED[560] = true
WAITS_FIRST[560] = true

B[561] = function(e)
  wait_flag(e, 8, true)
  pause(e, 64)
  move_to(e, 31, 36)
  move_wait(e, 128)
  set_flag(10)
  turn(e, 1)
  move_until_collision(e)
  pause(e, 0)
end
SCRIPTED[561] = true
WAITS_FIRST[561] = true

B[562] = function(e)
  wait_flag(e, 8, true)
  pause(e, 128)
  move_to(e, 31, 36)
  move_wait(e, 128)
  set_flag(11)
  turn(e, 1)
  move_until_collision(e)
  pause(e, 0)
end
SCRIPTED[562] = true
WAITS_FIRST[562] = true

B[563] = function(e)
  wait_flag(e, 8, true)
  pause(e, 192)
  move_to(e, 31, 36)
  move_wait(e, 128)
  set_flag(12)
  turn(e, 1)
  move_until_collision(e)
  pause(e, 0)
end
SCRIPTED[563] = true
WAITS_FIRST[563] = true

B[564] = function(e)
  wait_flag(e, 8, true)
  pause(e, 200)
  pause(e, 56)
  move_to(e, 31, 36)
  set_flag(13)
  move_wait(e, 128)
  turn(e, 1)
  move_until_collision(e)
  set_flag(14)
  pause(e, 0)
end
SCRIPTED[564] = true
WAITS_FIRST[564] = true

B[565] = function(e)
  wait_flag(e, 8, true)
  move_to(e, 31, 36)
  wait_flag(e, 9, true)
  turn_wait(e, 1)
  wait_flag(e, 14, true)
  turn_wait(e, 2)
  cutscene(e, 236)
  turn(e, 0)
  move_until_collision(e)
  turn(e, 1)
  move_noclip(e, 32)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[565] = true
WAITS_FIRST[565] = true

B[566] = function(e)
  wait_flag(e, 8, true)
  pause(e, 64)
  move_to(e, 31, 36)
  wait_flag(e, 10, true)
  turn_wait(e, 1)
  wait_flag(e, 14, true)
  pause(e, 4)
  move_noclip(e, 32)
  turn(e, 0)
  move_until_collision(e)
  turn(e, 1)
  move_noclip(e, 32)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[566] = true
WAITS_FIRST[566] = true

B[567] = function(e)
  wait_flag(e, 8, true)
  pause(e, 128)
  move_to(e, 31, 36)
  wait_flag(e, 11, true)
  turn_wait(e, 1)
  wait_flag(e, 14, true)
  pause(e, 4)
  move_noclip(e, 64)
  turn(e, 0)
  move_until_collision(e)
  turn(e, 1)
  move_noclip(e, 32)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[567] = true
WAITS_FIRST[567] = true

B[568] = function(e)
  wait_flag(e, 8, true)
  pause(e, 192)
  move_to(e, 31, 36)
  wait_flag(e, 12, true)
  turn_wait(e, 1)
  wait_flag(e, 14, true)
  pause(e, 4)
  move_noclip(e, 96)
  turn(e, 0)
  move_until_collision(e)
  turn(e, 1)
  move_noclip(e, 32)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[568] = true
WAITS_FIRST[568] = true

B[569] = function(e)
  wait_flag(e, 8, true)
  pause(e, 200)
  pause(e, 56)
  move_to(e, 31, 36)
  wait_flag(e, 13, true)
  turn_wait(e, 1)
  wait_flag(e, 14, true)
  pause(e, 4)
  move_noclip(e, 128)
  turn(e, 0)
  move_until_collision(e)
  turn(e, 1)
  move_noclip(e, 32)
  cutscene(e, 237)
  set_flag(336)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[569] = true
WAITS_FIRST[569] = true

-- story actions
story(235, {
  run(346),
  say(215),
})

story(236, {
  say(216),
})

story(237, {
  say(217),
  friday_does("leave"),
})
