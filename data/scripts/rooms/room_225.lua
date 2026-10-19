-- Behaviours of room 225 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[329] = function(e)
  pause_player()
  rotate_player(1)
  cutscene(e, 113)
  pause(e, 90)
  move_noclip(e, 80)
  rotate_player(0)
  turn(e, 2)
  move_noclip(e, 8)
  wait_flag(e, 8, true)
  move_noclip(e, 8)
  pause(e, 1)
  cutscene(e, 115)
  move_noclip(e, 48)
  turn(e, 3)
  move_until_collision(e)
  rotate_player(1)
  pause(e, 1)
  cutscene(e, 116)
  turn(e, 1)
  move_timed(e, 16)
  turn(e, 0)
  move_noclip(e, 48)
  turn(e, 2)
  pause(e, 1)
  cutscene(e, 117)
  set_flag(9)
  turn(e, 0)
  move_noclip(e, 16)
  turn(e, 1)
  move_noclip(e, 80)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[329] = true

B[330] = function(e)
  pause(e, 90)
  move_noclip(e, 80)
  turn(e, 2)
  wait_flag(e, 9, true)
  turn(e, 0)
  move_noclip(e, 16)
  turn(e, 1)
  move_noclip(e, 80)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[330] = true

B[331] = function(e)
  pause(e, 90)
  move_noclip(e, 128)
  turn(e, 2)
  move_until_collision(e)
  move_timed(e, 16)
  cutscene(e, 114)
  turn(e, 0)
  move_noclip(e, 48)
  turn(e, 2)
  set_flag(8)
  wait_flag(e, 9, true)
  turn(e, 0)
  move_noclip(e, 16)
  turn(e, 1)
  move_noclip(e, 128)
  cutscene(e, 118)
  set_flag(167)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[331] = true

-- story actions
story(113, {
  play_sound(22),
})

story(114, {
  say(40),
  friday_does("hover"),
})

story(115, {
  say(41),
})

story(116, {
  say(42),
})

story(117, {
  say(43),
  friday_does("hover"),
})

story(118, {
  say(44),
  friday_does("hover"),
  friday_does("leave"),
})
