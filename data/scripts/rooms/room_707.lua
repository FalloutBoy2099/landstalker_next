-- Behaviours of room 707 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[265] = function(e)
  cutscene(e, 64)
  turn(e, 1)
  move_noclip(e, 32)
  ::L5::
  move_to(e, 41, 27)
  wait_trigger(e, 35)
  move_to(e, 20, 27)
  pause_player()
  turn(e, 3)
  move_noclip(e, 40)
  turn(e, 0)
  pause(e, 1)
  cutscene(e, 65)
  move_noclip(e, 24)
  playback(15)
  turn(e, 2)
  move_noclip(e, 24)
  turn(e, 1)
  move_noclip(e, 40)
  jump_guard(e) goto L5
end
SCRIPTED[265] = true

B[272] = function(e)
  wait_flag(e, 29, false)
  wait_flag(e, 30, true)
  wait_trigger(e, 36)
  pause_player()
  move_to(e, 20, 18)
  move_noclip(e, 32)
  turn(e, 2)
  move_noclip(e, 64)
  cutscene(e, 68)
  rotate_player(0)
  e.speed = 2
  move_noclip(e, 32)
  cutscene(e, 69)
  e.speed = 1
  turn_wait(e, 0)
  playback(16)
  move_noclip(e, 32)
  e.speed = 2
  turn(e, 3)
  move_noclip(e, 8)
  turn(e, 2)
  move_noclip(e, 16)
  turn(e, 1)
  move_noclip(e, 8)
  turn(e, 0)
  move_wait(e, 64)
  turn(e, 1)
  move_wait(e, 16)
  pause(e, 0)
end
SCRIPTED[272] = true
WAITS_FIRST[272] = true

-- story actions
story(64, {
  say(76),
})

story(65, {
  say(77),
})

story(68, {
  say(81),
})

story(69, {
  say(82),
})
