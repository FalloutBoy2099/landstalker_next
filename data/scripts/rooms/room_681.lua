-- Behaviours of room 681 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[321] = function(e)
  wait_flag(e, 163, false)
  wait_trigger(e, 46)
  pause_player()
  move_noclip(e, 16)
  cutscene(e, 105)
  pause(e, 32)
  pause(e, 32)
  set_flag(163)
  pause(e, 0)
end
SCRIPTED[321] = true
WAITS_FIRST[321] = true

-- story actions
story(105, {
  say(113),
  play_sound(82),
  say(114),
})

story(106, {
  story_flag(164, true),
  say(115),
  if_yes({
      story_flag(8, true),
    }),
})

story(109, {
  story_flag(8, true),
  say(117),
})
