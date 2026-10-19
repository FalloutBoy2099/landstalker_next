-- Behaviours of room 224 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[157] = function(e)
  wait_trigger(e, 43)
  pause_player()
  pause(e, 2)
  cutscene(e, 103)
  set_flag(8)
  ::L10::
  wait_trigger(e, 23)
  cutscene(e, 31)
  jump_guard(e) goto L10
end
SCRIPTED[157] = true

B[308] = function(e)
  wait_flag(e, 8, true)
  move_noclip(e, 48)
  move_to(e, 0, 0)
  wait_trigger(e, 44)
  pause_player()
  move_timed(e, 2)
  cutscene(e, 104)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[308] = true
WAITS_FIRST[308] = true

B[309] = function(e)
  wait_flag(e, 8, true)
  move_noclip(e, 56)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[309] = true
WAITS_FIRST[309] = true

-- story actions
story(31, {
  run(346),
  say(29),
  friday_does("leave"),
})

story(103, {
  say(31),
})

story(104, {
  run(346),
  say(38),
  friday_does("hide"),
  say(39),
  story_flag(33, true),
  friday_does("hover"),
})
