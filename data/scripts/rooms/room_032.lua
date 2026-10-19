-- Behaviours of room 32 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[324] = function(e)
  wait_trigger(e, 47)
  pause_player()
  move_to(e, 31, 34)
  pause(e, 2)
  cutscene(e, 111)
  move_down_to(e, 96)
  move_down_to(e, 48)
  e.falls = true; e.fall_v = 0
  cutscene(e, 112)
  playback(32)
  set_flag(176)
  do return end  -- unknown command 2E
end
SCRIPTED[324] = true

-- story actions
story(111, {
  run(346),
  say(111),
})

story(112, {
  say(112),
  friday_does("leave"),
})
