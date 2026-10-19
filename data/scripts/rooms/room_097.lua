-- Behaviours of room 97 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[285] = function(e)
  playback(26)
  move_noclip(e, 48)
  turn_wait(e, 1)
  pause(e, 10)
  cutscene(e, 58)
  turn(e, 0)
  move_noclip(e, 8)
  turn(e, 1)
  move_noclip(e, 80)
  clear_flag(144)
  set_flag(146)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[285] = true

B[287] = function(e)
  cutscene(e, 84)
  turn(e, 1)
  move_noclip(e, 80)
  e.visible = false; e.base_visible = false
  cutscene(e, 85)
  clear_flag(149)
  set_flag(150)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[287] = true

B[301] = function(e)
  wait_trigger(e, 41)
  pause_player()
  pause(e, 1)
  cutscene(e, 98)
  wait_flag(e, 8, true)
  pause(e, 150)
  cutscene(e, 99)
  pause(e, 0)
end
SCRIPTED[301] = true

-- story actions
story(58, {
  say(91),
})

story(84, {
  say(96),
})

story(85, {
  run(346),
  if_flag(151, {
      say(98),
      friday_does("leave"),
    }, {
      friday_does("leave"),
    }),
})

story(98, {
  run(346),
  say(105),
  if_yes({
      story_flag(8, true),
    }, {
      friday_does("leave"),
      lock_controls(false),
    }),
})

story(99, {
  play_sound(65533),
  pause_frames(180),
  play_sound(40),
  pause_frames(120),
  say(106),
  friday_does("hover"),
  say(107),
  friday_does("hover"),
  say(108),
  play_sound(65533),
  friday_does("hide"),
  pause_frames(120),
  story_flag(160, true),
  go_to(-1, 20, 14, -1),
  lock_controls(false),
})
