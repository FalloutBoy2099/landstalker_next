-- Behaviours of room 51 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[277] = function(e)
  playback(21)
  move_noclip(e, 128)
  move_noclip(e, 40)
  turn(e, 2)
  pause(e, 8)
  cutscene(e, 77)
  turn(e, 3)
  move_noclip(e, 24)
  turn(e, 1)
  set_flag(31)
  pause(e, 0)
end
SCRIPTED[277] = true

B[278] = function(e)
  wait_flag(e, 31, true)
  pause(e, 60)
  move_noclip(e, 12)
  cutscene(e, 78)
  turn(e, 1)
  move_noclip(e, 24)
  turn(e, 2)
  move_noclip(e, 208)
  cutscene(e, 79)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[278] = true
WAITS_FIRST[278] = true

-- story actions
story(77, {
  say(88),
})

story(78, {
  say(89),
})

story(79, {
  story_flag(145, true),
})
