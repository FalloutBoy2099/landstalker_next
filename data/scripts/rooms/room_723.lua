-- Behaviours of room 723 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[528] = function(e)
  wait_flag(e, 321, false)
  pause(e, 15)
  pause_player()
  turn(e, 2)
  move_noclip(e, 24)
  rotate_player(1)
  turn(e, 3)
  move_noclip(e, 16)
  cutscene(e, 221)
  set_flag(321)
  turn(e, 1)
  move_noclip(e, 16)
  turn(e, 0)
  move_noclip(e, 24)
  turn_wait(e, 3)
  pause(e, 0)
end
SCRIPTED[528] = true
WAITS_FIRST[528] = true

-- story actions
story(221, {
  say(204),
})
