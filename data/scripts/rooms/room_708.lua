-- Behaviours of room 708 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[271] = function(e)
  wait_flag(e, 30, false)
  cutscene(e, 67)
  set_flag(30)
  pause(e, 0)
end
SCRIPTED[271] = true
WAITS_FIRST[271] = true

-- story actions
story(67, {
  say(78),
  friday_does("hover"),
  say(79),
  story_flag(28, false),
  say(80),
  friday_does("leave"),
})
