-- Behaviours of room 710 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[273] = function(e)
  playback(17)
  move_wait(e, 32)
  rotate_player(3)
  turn_wait(e, 3)
  move_wait(e, 8)
  turn_wait(e, 3)
  cutscene(e, 70)
  set_flag(8)
  wait_flag(e, 9, true)
  turn(e, 1)
  pause(e, 1)
  e.speed = 1
  cutscene(e, 71)
  playback(18)
  move_wait(e, 8)
  cutscene(e, 72)
  move_wait(e, 8)
  cutscene(e, 73)
  e.speed = 4
  turn_wait(e, 3)
  move_until_collision(e)
  cutscene(e, 74)
  e.speed = 1
  turn(e, 1)
  move_until_collision(e)
  cutscene(e, 75)
  set_flag(29)
  turn(e, 3)
  move_noclip(e, 64)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[273] = true

B[274] = function(e)
  pause(e, 30)
  turn(e, 1)
  wait_flag(e, 8, true)
  turn(e, 0)
  move_wait(e, 8)
  turn(e, 3)
  pause(e, 20)
  sfx(88)
  tile_swap(1)
  pause(e, 10)
  turn(e, 1)
  set_flag(9)
  wait_flag(e, 29, true)
  sfx(88)
  tile_swap(2)
  turn(e, 3)
  e.speed = 1
  move_noclip(e, 16)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[274] = true

-- story actions
story(70, {
  say(83),
})

story(71, {
  say(84),
})

story(73, {
  play_sound(82),
})

story(74, {
  say(85),
})

story(75, {
  say(86),
})
