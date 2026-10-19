-- Behaviours of room 43 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[408] = function(e)
  wait_trigger(e, 74)
  pause_player()
  pause(e, 24)
  cutscene(e, 166)
  turn(e, 0)
  move_noclip(e, 6)
  pause(e, 30)
  move_noclip(e, 12)
  pause(e, 30)
  move_noclip(e, 14)
  turn(e, 2)
  cutscene(e, 167)
  e.speed = 2
  move_noclip(e, 56)
  set_flag(155)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[408] = true

-- story actions
story(166, {
  say(148),
})

story(167, {
  say(149),
})
