-- Behaviours of room 486 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[627] = function(e)
  wait_flag(e, 8, true)
  sfx(89)
  repeat_begin(e, 16)
  ::L7::
  e.speed = 1
  turn(e, 3)
  move_wait(e, 1)
  pause(e, 2)
  if repeat_next(e) then goto L7 end
  pause(e, 0)
end
SCRIPTED[627] = true
WAITS_FIRST[627] = true

B[628] = function(e)
  wait_flag(e, 343, false)
  wait_trigger(e, 109)
  pause_player()
  pause(e, 30)
  move_to(e, 43, 62)
  rotate_player(2)
  e.speed = 2
  cutscene(e, 247)
  move_wait(e, 96)
  cutscene(e, 248)
  playback(18)
  e.speed = 8
  move_wait(e, 12)
  cutscene(e, 249)
  set_flag(343)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[628] = true
WAITS_FIRST[628] = true

-- story actions
story(248, {
  say(223),
})

story(249, {
  run(346),
  say(224),
  friday_does("leave"),
})
