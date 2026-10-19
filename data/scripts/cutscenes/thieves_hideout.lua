-- Story actions of thieves hideout (started by several of its rooms).

-- story actions
story(23, {
  swap_tiles(16),
})

story(24, {
  run(346),
  say(28),
  friday_does("leave"),
  if_yes({
      swap_tiles(8),
      story_flag(8, true),
    }, {
      lock_controls(false),
    }),
})

story(27, {
  swap_tiles(8),
})
