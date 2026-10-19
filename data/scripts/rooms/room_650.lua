-- Behaviours of room 650 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[552] = function(e)
  wait_trigger(e, 101)
  pause_player()
  pause(e, 30)
  rotate_player(3)
  turn_wait(e, 1)
  cutscene(e, 225)
  turn(e, 3)
  move_noclip(e, 8)
  turn(e, 0)
  move_noclip(e, 16)
  turn_wait(e, 3)
  cutscene(e, 226)
  turn(e, 2)
  move_noclip(e, 8)
  turn(e, 1)
  move_noclip(e, 36)
  turn(e, 0)
  move_noclip(e, 24)
  rotate_player(0)
  set_flag(8)
  pause(e, 12)
  turn_wait(e, 3)
  wait_flag(e, 9, true)
  sfx(103)
  cutscene(e, 227)
  e.speed = 4
  repeat_begin(e, 16)
  ::L46::
  sfx(103)
  turn(e, math.random(0, 3))
  move_random_time(e, 8, 16)
  if repeat_next(e) then goto L46 end
  cutscene(e, 228)
  repeat_begin(e, 12)
  ::L57::
  sfx(103)
  turn(e, math.random(0, 3))
  move_random_time(e, 8, 16)
  if repeat_next(e) then goto L57 end
  e.speed = 2
  repeat_begin(e, 8)
  ::L67::
  sfx(103)
  turn(e, math.random(0, 3))
  move_random_time(e, 8, 32)
  if repeat_next(e) then goto L67 end
  cutscene(e, 229)
  set_flag(326)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[552] = true

B[553] = function(e)
  wait_flag(e, 8, true)
  move_to(e, 16, 16)
  move_noclip(e, 8)
  move_down_until_collision(e)
  move_noclip(e, 4)
  move_down_until_collision(e)
  move_noclip(e, 8)
  set_flag(9)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[553] = true
WAITS_FIRST[553] = true

B[554] = function(e)
  wait_flag(e, 10, true)
  wait_trigger(e, 102)
  e.hostile = true; e.dormant = false
  cutscene(e, 231)
  move_to(e, 16, 16)
  move_timed(e, 8)
  move_down_until_collision(e)
  move_timed(e, 4)
  move_down_until_collision(e)
  move_until_collision(e)
  turn(e, 2)
  move_until_collision(e)
  turn(e, 3)
  move_until_collision(e)
  move_up_to(e, 16)
  move_until_collision(e)
  move_up_to(e, 64)
  move_timed(e, 4)
  turn(e, 0)
  move_timed(e, 8)
  move_down_until_collision(e)
  move_until_collision(e)
  turn(e, 2)
  move_timed(e, 8)
  turn(e, 1)
  move_timed(e, 8)
  move_down_until_collision(e)
  move_timed(e, 4)
  move_down_until_collision(e)
  move_timed(e, 4)
  repeat_begin(e, 25)
  ::L48::
  turn(e, math.random(0, 3))
  move_random_time(e, 8, 16)
  if repeat_next(e) then goto L48 end
  do return end  -- unknown command 5B
end
SCRIPTED[554] = true
WAITS_FIRST[554] = true

-- story actions
story(225, {
  say(207),
})

story(226, {
  say(208),
})

story(228, {
  say(209),
})

story(229, {
  say(210),
})
