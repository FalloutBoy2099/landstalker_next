-- Behaviours of room 333 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[796] = function(e)
  pause(e, 8)
  playback(58)
  set_flag(8)
  move_wait(e, 96)
  pause(e, 20)
  turn_wait(e, 1)
  pause(e, 30)
  set_flag(10)
  pause(e, 30)
  cutscene(e, 0x100 + 65)
  turn(e, 0)
  set_flag(11)
  repeat_begin(e, 8)
  ::L27::
  move_wait(e, 4)
  pause(e, 30)
  if repeat_next(e) then goto L27 end
  turn_wait(e, 1)
  cutscene(e, 0x100 + 66)
  turn(e, 0)
  move_until_collision(e)
  e.visible = false
  pause(e, 32)
  set_flag(351)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[796] = true

B[797] = function(e)
  wait_flag(e, 8, true)
  move_wait(e, 96)
  wait_flag(e, 10, true)
  turn_wait(e, 1)
  wait_flag(e, 11, true)
  turn(e, 0)
  repeat_begin(e, 8)
  ::L15::
  move_wait(e, 4)
  pause(e, 30)
  if repeat_next(e) then goto L15 end
  move_until_collision(e)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[797] = true
WAITS_FIRST[797] = true

-- story actions
story(321, {
  say(306),
})

story(322, {
  say(307),
})
