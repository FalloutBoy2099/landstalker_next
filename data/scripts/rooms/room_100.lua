-- Behaviours of room 100 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[288] = function(e)
  playback(28)
  set_flag(151)
  ::L5::
  cutscene(e, 87)
  turn_wait(e, math.random(0, 3))
  move_until_collision(e)
  jump_guard(e) goto L5
end
SCRIPTED[288] = true

B[289] = function(e)
  pause(e, 240)
  cutscene(e, 86)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[289] = true

-- story actions
story(86, {
  effect("fade_out"),
  friday_does("hide"),
  go_to(-1, 15, 20, -1),
  effect("fade_in"),
})

story(87, {
  play_sound(100),
})
