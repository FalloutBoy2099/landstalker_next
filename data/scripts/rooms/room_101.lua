-- Behaviours of room 101 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[286] = function(e)
  wait_trigger(e, 38)
  set_flag(149)
  pause_player()
  pause(e, 2)
  cutscene(e, 83)
  pause(e, 0)
end
SCRIPTED[286] = true

-- story actions
story(83, {
  say(94),
  if_yes({
      run(346),
      say(95),
      effect("fade_out"),
      friday_does("hide"),
      go_to(-1, 15, 20, -1),
      effect("fade_in"),
    }),
})
