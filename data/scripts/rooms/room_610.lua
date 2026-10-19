-- Behaviours of room 610 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[139] = function(e)
  wait_flag(e, 8, true)
  playback(12)
  move_noclip(e, 32)
  rotate_player(0)
  cutscene(e, 21)
  turn(e, 3)
  move_noclip(e, 48)
  rotate_player(3)
  turn(e, 2)
  move_noclip(e, 8)
  turn(e, 3)
  move_noclip(e, 40)
  set_flag(23)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[139] = true
WAITS_FIRST[139] = true

B[505] = function(e)
  ::L0::
  wait_flag(e, 8, false)
  wait_flag(e, 23, false)
  cutscene(e, 205)
  pause(e, 240)
  jump_guard(e) goto L0
end
SCRIPTED[505] = true
WAITS_FIRST[505] = true

-- story actions
story(21, {
  say(26),
})

story(299, {
  say(288),
})
