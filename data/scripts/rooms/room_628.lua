-- Behaviours of room 628 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[508] = function(e)
  wait_flag(e, 306, true)
  pause_player()
  rotate_player(0)
  turn(e, 2)
  move_noclip(e, 48)
  cutscene(e, 206)
  turn(e, 0)
  move_noclip(e, 80)
  turn(e, 1)
  move_noclip(e, 48)
  turn(e, 2)
  move_noclip(e, 16)
  cutscene(e, 207)
  pause(e, 0)
end
SCRIPTED[508] = true
WAITS_FIRST[508] = true

-- story actions
story(206, {
  say(187),
})

story(207, {
  say(188),
  story_flag(8, true),
})
