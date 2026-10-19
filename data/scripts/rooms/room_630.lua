-- Behaviours of room 630 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[179] = function(e)
  wait_flag(e, 40, false)
  wait_flag(e, 40, true)
  pause(e, 60)
  move_to(e, 19, 36)
  pause(e, 15)
  turn(e, 2)
  move_noclip(e, 32)
  turn(e, 1)
  move_noclip(e, 16)
  cutscene(e, 42)
  rotate_player(2)
  e.speed = 2
  move_noclip(e, 32)
  pause(e, 20)
  cutscene(e, 43)
  turn(e, 2)
  e.speed = 1
  move_noclip(e, 64)
  turn(e, 3)
  move_noclip(e, 16)
  turn(e, 0)
  move_noclip(e, 32)
  e.speed = 2
  turn_wait(e, 2)
  move_noclip(e, 16)
  e.speed = 1
  turn(e, 3)
  move_noclip(e, 32)
  turn(e, 0)
  move_noclip(e, 32)
  turn(e, 3)
  cutscene(e, 44)
  move_noclip(e, 32)
  turn_wait(e, 1)
  move_to(e, 35, 29)
  pause(e, 0)
end
SCRIPTED[179] = true
WAITS_FIRST[179] = true

B[180] = function(e)
  wait_flag(e, 29, false)
  wait_trigger(e, 28)
  turn(e, 1)
  move_wait(e, 10)
  turn(e, 2)
  pause(e, 0)
end
SCRIPTED[180] = true
WAITS_FIRST[180] = true

B[181] = function(e)
  wait_flag(e, 29, false)
  wait_trigger(e, 28)
  pause_player()
  turn(e, 3)
  move_wait(e, 10)
  turn(e, 2)
  pause(e, 0)
end
SCRIPTED[181] = true
WAITS_FIRST[181] = true

-- story actions
story(42, {
  say(51),
})

story(43, {
  say(52),
})

story(223, {
  say(205),
  friday_does("hover"),
  friday_does("leave"),
  say(206),
})
