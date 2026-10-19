-- Behaviours of room 183 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[140] = function(e)
  wait_flag(e, 32, true)
  turn(e, 3)
  move_wait(e, 8)
  turn(e, 2)
  pause(e, 0)
end
SCRIPTED[140] = true
WAITS_FIRST[140] = true

B[141] = function(e)
  ::L0::
  wait_trigger(e, 18)
  wait_trigger(e, 16)
  pause_player()
  wait_trigger(e, 17)
  cutscene(e, 22)
  jump_guard(e) goto L0
end
SCRIPTED[141] = true

B[142] = function(e)
  wait_flag(e, 8, true)
  move_noclip(e, 240)
  move_noclip(e, 240)
end
SCRIPTED[142] = true
WAITS_FIRST[142] = true

B[145] = function(e)
  cutscene(e, 25)
  move_noclip(e, 240)
  move_noclip(e, 112)
  cutscene(e, 26)
  wait_flag(e, 8, true)
  turn(e, 0)
  move_noclip(e, 240)
  move_noclip(e, 240)
end
SCRIPTED[145] = true

B[171] = function(e)
  wait_flag(e, 32, true)
  wait_trigger(e, 26)
  pause_player()
  pause(e, 200)
  pause(e, 42)
  cutscene(e, 36)
  move_noclip(e, 208)
  rotate_player(2)
  cutscene(e, 37)
  move_noclip(e, 32)
  cutscene(e, 38)
  move_noclip(e, 128)
  rotate_player(3)
  cutscene(e, 39)
  set_flag(38)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[171] = true
WAITS_FIRST[171] = true

B[172] = function(e)
  wait_flag(e, 38, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[172] = true
WAITS_FIRST[172] = true

-- story actions
story(22, {
  run(346),
  say(27),
  friday_does("leave"),
  if_yes({
      swap_tiles(8),
      story_flag(8, true),
    }, {
      lock_controls(false),
    }),
})

story(25, {
  swap_tiles(8),
})

story(26, {
  swap_tiles(16),
})

story(36, {
  say(34),
})

story(37, {
  say(35),
})

story(38, {
  say(36),
})

story(39, {
  say(37),
})
