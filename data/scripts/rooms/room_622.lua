-- Behaviours of room 622 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[162] = function(e)
  wait_flag(e, 8, true)
  pause_player()
  turn(e, 1)
  move_noclip(e, 112)
  turn(e, 0)
  move_noclip(e, 16)
  pause(e, 50)
  turn(e, 2)
  move_noclip(e, 16)
  turn(e, 3)
  move_noclip(e, 112)
  cutscene(e, 35)
  pause(e, 1)
  cutscene(e, 34)
  pause(e, 0)
end
SCRIPTED[162] = true
WAITS_FIRST[162] = true

-- story actions
story(34, {
  say(33),
  story_flag(35, true),
})
