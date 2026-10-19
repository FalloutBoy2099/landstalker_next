-- Behaviours of room 148 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[138] = function(e)
  wait_flag(e, 5, false)
  pause_player()
  pause(e, 30)
  cutscene(e, 0x100 + 28)
  wait_trigger(e, 15)
  cutscene(e, 17)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[138] = true
WAITS_FIRST[138] = true

-- story actions
story(17, {
  friday_does("appear"),
  say(0),
  friday_does("leave"),
  story_flag(5, true),
})

story(284, {
  say(282, 180),
  say(283, 180),
  say(284, 180),
  say(285, 120),
  say(286, 180),
  say(287, 60),
  lock_controls(false),
  friday_does("leave"),
})
