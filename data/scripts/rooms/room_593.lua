-- Behaviours of room 593 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[154] = function(e)
  cutscene(e, 28)
  set_flag(8)
  pause(e, 0)
end
SCRIPTED[154] = true

B[155] = function(e)
  wait_flag(e, 8, true)
  turn(e, 2)
  move_noclip(e, 32)
  turn(e, 1)
  move_noclip(e, 32)
  cutscene(e, 29)
  pause(e, 0)
end
SCRIPTED[155] = true
WAITS_FIRST[155] = true

-- story actions
story(28, {
  say(3),
  say(4),
})

story(29, {
  say(5),
  story_flag(7, true),
  effect("fade_out"),
  go_to(-1, 17, 16, -1),
  friday_does("leave"),
  lock_controls(false),
  story_flag(4, true),
})
