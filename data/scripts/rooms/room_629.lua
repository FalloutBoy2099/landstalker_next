-- Behaviours of room 629 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[510] = function(e)
  wait_flag(e, 8, true)
  turn(e, 3)
  move_noclip(e, 48)
  turn(e, 2)
  move_noclip(e, 80)
  cutscene(e, 208)
  pause(e, 0)
end
SCRIPTED[510] = true
WAITS_FIRST[510] = true

-- story actions
story(208, {
  say(189),
})
