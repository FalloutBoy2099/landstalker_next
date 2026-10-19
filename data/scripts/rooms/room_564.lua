-- Behaviours of room 564 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[472] = function(e)
  wait_flag(e, 294, false)
  wait_trigger(e, 94)
  pause_player()
  rotate_player(0)
  pause(e, 3)
  cutscene(e, 200)
  pause(e, 0)
end
SCRIPTED[472] = true
WAITS_FIRST[472] = true

B[473] = function(e)
  wait_flag(e, 304, true)
  pause_player()
  cutscene(e, 201)
  rotate_player(3)
  move_to(e, 35, 31)
  move_noclip(e, 80)
  cutscene(e, 202)
  playback(44)
  pause(e, 24)
  turn(e, 0)
  move_noclip(e, 8)
  turn(e, 1)
  move_noclip(e, 24)
  e.speed = 4
  rotate_player(0)
  repeat_begin(e, 4)
  ::L32::
  turn_wait(e, 0)
  sfx(81)
  pause(e, 10)
  turn(e, 1)
  move_noclip(e, 8)
  turn_wait(e, 0)
  sfx(81)
  pause(e, 10)
  turn(e, 3)
  move_noclip(e, 4)
  turn_wait(e, 0)
  sfx(81)
  pause(e, 10)
  turn(e, 1)
  move_noclip(e, 8)
  turn_wait(e, 0)
  sfx(81)
  pause(e, 10)
  turn(e, 3)
  move_noclip(e, 12)
  set_flag(9)
  if repeat_next(e) then goto L32 end
  turn_wait(e, 2)
  pause(e, 1)
  cutscene(e, 203)
  set_flag(8)
  e.speed = 2
  turn(e, 3)
  move_noclip(e, 96)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[473] = true
WAITS_FIRST[473] = true

B[474] = function(e)
  wait_flag(e, 304, true)
  move_to(e, 36, 32)
  move_noclip(e, 72)
  wait_flag(e, 8, true)
  turn(e, 3)
  move_noclip(e, 96)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[474] = true
WAITS_FIRST[474] = true

B[475] = function(e)
  wait_flag(e, 9, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[475] = true
WAITS_FIRST[475] = true

B[476] = function(e)
  wait_flag(e, 9, true)
  pause(e, 45)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[476] = true
WAITS_FIRST[476] = true

B[477] = function(e)
  wait_flag(e, 9, true)
  pause(e, 90)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[477] = true
WAITS_FIRST[477] = true

B[478] = function(e)
  wait_flag(e, 9, true)
  pause(e, 135)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[478] = true
WAITS_FIRST[478] = true

B[509] = function(e)
  wait_trigger(e, 96)
  set_flag(306)
  pause(e, 0)
end

-- story actions
story(200, {
  run(346),
  say(179),
  if_yes({
      say(180),
      pause_frames(60),
      say(181),
      friday_does("hover"),
      say(182),
      friday_does("hover"),
      friday_does("leave"),
    }, {
      friday_does("leave"),
    }),
})

story(201, {
  say(184),
})

story(202, {
  say(185),
})

story(203, {
  say(186),
})
