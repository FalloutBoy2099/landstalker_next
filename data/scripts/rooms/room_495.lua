-- Behaviours of room 495 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[518] = function(e)
  pause_player()
  turn_wait(e, 0)
  cutscene(e, 213)
  playback(45)
  turn(e, 1)
  move_noclip(e, 8)
  turn(e, 0)
  move_noclip(e, 16)
  turn(e, 1)
  move_noclip(e, 32)
  turn(e, 0)
  move_noclip(e, 56)
  turn_wait(e, 1)
  pause(e, 60)
  turn_wait(e, 2)
  rotate_player(3)
  cutscene(e, 214)
  pause(e, 0)
end
SCRIPTED[518] = true

-- story actions
story(213, {
  say(191),
})

story(214, {
  say(192),
  go_to(543, 45, 40, -1),
})
