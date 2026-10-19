-- Behaviours of room 449 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[135] = function(e)
  wait_flag(e, 8, true)
  move_wait(e, 8)
  e.speed = 1
  pause(e, 8)
  cutscene(e, 14)
  set_flag(9)
  move_wait(e, 48)
  pause(e, 30)
  turn(e, 1)
  e.speed = 2
  move_timed(e, 32)
  pause(e, 9)
  cutscene(e, 15)
  turn(e, 2)
  move_noclip(e, 24)
  turn(e, 0)
  set_flag(22)
  pause(e, 0)
end
SCRIPTED[135] = true
WAITS_FIRST[135] = true

B[136] = function(e)
  wait_flag(e, 9, true)
  pause(e, 20)
  move_wait(e, 1)
  pause(e, 40)
  move_wait(e, 2)
  pause(e, 20)
  move_timed(e, 5)
  pause(e, 5)
  move_wait(e, 40)
  pause(e, 0)
end
SCRIPTED[136] = true
WAITS_FIRST[136] = true

B[137] = function(e)
  wait_flag(e, 8, true)
  move_wait(e, 8)
  e.speed = 1
  pause(e, 8)
  move_wait(e, 48)
  pause(e, 30)
  turn(e, 1)
  e.speed = 2
  move_wait(e, 32)
  pause(e, 8)
  turn(e, 2)
  move_noclip(e, 16)
  turn(e, 1)
  move_noclip(e, 9)
  turn(e, 0)
  pause(e, 0)
end
SCRIPTED[137] = true
WAITS_FIRST[137] = true

-- story actions
story(14, {
  say(23),
})

story(15, {
  say(24),
})
