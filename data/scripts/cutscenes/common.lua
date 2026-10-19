-- Story actions that several areas start.

-- story actions
story(33, {
  say(32),
  story_flag(8, true),
})

story(47, {
  say(46),
  if_yes({
      story_flag(42, true),
      run(346),
      say(48),
      say(49),
      friday_does("leave"),
    }, {
      say(47),
    }),
})

story(81, {
  story_flag(8, true),
  say(92),
})

story(154, {
  play_sound(77),
  go_to(-1, 22, 28, 2),
  effect("fade_in"),
})

story(165, {
  story_flag(154, true),
  say(147),
})

story(181, {
  say(160),
})

story(210, {
  say(190),
  play_sound(12),
})

story(250, {
  story_flag(8, true),
  say(225),
})

story(346, {
  friday_does("appear"),
})
