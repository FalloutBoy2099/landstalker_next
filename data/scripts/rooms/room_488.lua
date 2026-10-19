-- Behaviours of room 488 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[558] = function(e)
  wait_trigger(e, 103)
  pause_player()
  pause(e, 15)
  turn_wait(e, 2)
  rotate_player(0)
  cutscene(e, 233)
  set_flag(327)
  pause(e, 128)
  cutscene(e, 234)
  pause(e, 0)
end
SCRIPTED[558] = true

B[559] = function(e)
  wait_flag(e, 327, true)
  move_noclip(e, 128)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[559] = true
WAITS_FIRST[559] = true

-- story actions
story(233, {
  say(214),
})
