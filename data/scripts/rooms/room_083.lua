-- Behaviours of room 83 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[280] = function(e)
  ::L0::
  e.speed = 2
  move_until_collision(e)
  e.speed = 1
  turn_wait(e, e.orientation + 2)
  move_wait(e, 48)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[281] = function(e)
  wait_flag(e, 8, true)
  playback(22)
  turn(e, 0)
  move_wait(e, 56)
  turn(e, 1)
  move_wait(e, 8)
  pause(e, 8)
  cutscene(e, 82)
  turn(e, 3)
  move_noclip(e, 8)
  turn(e, 2)
  move_noclip(e, 56)
  turn_wait(e, 3)
  pause(e, 1)
  pause(e, 0)
end
SCRIPTED[281] = true
WAITS_FIRST[281] = true

-- story actions
story(82, {
  say(93),
})
