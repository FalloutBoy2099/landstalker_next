-- Story actions no room starts (the story code starts them).

-- story actions
story(49, {
  make_hostile(0, true),
})

story(51, {
  lock_controls(false),
  say(62),
})

story(52, {
  say(63),
  if_yes({
      story_flag(8, true),
    }),
})

story(55, {
  say(67),
})

story(56, {
  say(66),
  if_yes({
      story_flag(26, true),
    }),
})

story(121, {
  story_flag(8, true),
})

story(122, {
  if_flag(168, {}, {
      story_flag(8, true),
    }),
})

story(124, {
  if_flag(169, {}, {
      story_flag(8, true),
    }),
})

story(125, {
  if_flag(171, {}, {
      story_flag(8, true),
    }),
})

story(126, {
  if_flag(172, {}, {
      story_flag(9, true),
    }),
})

story(127, {
  if_flag(173, {}, {
      story_flag(9, true),
    }),
})

story(128, {
  if_flag(174, {}, {
      story_flag(8, true),
    }),
})

story(136, {
  say(125),
  if_yes({
      story_flag(181, true),
    }, {
      story_flag(8, true),
    }),
})

story(137, {
  say(126),
  if_yes({
      story_flag(182, true),
    }, {
      story_flag(8, true),
    }),
})

story(138, {
  say(127),
  if_yes({
      story_flag(183, true),
    }),
})

story(179, {
  say(158),
})

story(204, {
  if_flag(175, {}, {
      story_flag(8, true),
    }),
})

story(230, {
  if_flag(59, {}, {
      story_flag(10, true),
    }),
})

story(306, {
  say(290),
  if_yes({
      story_flag(8, true),
    }),
})

story(309, {
  say(296),
  if_yes({
      story_flag(8, true),
    }),
})

story(311, {
  say(298),
  if_yes({}),
})

story(312, {
  say(299),
  if_yes({}),
})

story(345, {
  say(293),
  if_yes({
      story_flag(8, true),
    }),
})
