-- Behaviours of room 680 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[264] = function(e)
  wait_flag(e, 8, true)
  set_flag(26)
  pause_player()
  move_noclip(e, 48)
  turn(e, 0)
  move_noclip(e, 32)
  rotate_player(2)
  pause(e, 1)
  cutscene(e, 63)
  turn(e, 2)
  move_noclip(e, 32)
  turn(e, 3)
  move_noclip(e, 48)
  turn(e, 0)
  move_wait(e, 16)
  move_wait(e, 16)
  move_wait(e, 16)
  move_wait(e, 16)
  pause(e, 8)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[264] = true
WAITS_FIRST[264] = true

-- story actions
story(62, {
  say(74),
  story_flag(8, true),
})

story(63, {
  say(75),
})
