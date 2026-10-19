-- Behaviours of room 84 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[431] = function(e)
  pause_player()
  rotate_player(2)
  repeat_begin(e, 3)
  ::L5::
  cutscene(e, 180)
  sfx(82)
  pause(e, 8)
  move_wait(e, 12)
  if repeat_next(e) then goto L5 end
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[431] = true

B[432] = function(e)
  wait_flag(e, 262, false)
  repeat_begin(e, 3)
  ::L7::
  turn(e, 3)
  move_wait(e, 8)
  turn(e, 1)
  pause(e, 12)
  if repeat_next(e) then goto L7 end
  cutscene(e, 222)
  pause(e, 4)
  wait_trigger(e, 2)
  wait_flag(e, 262, true)
  playback(40)
  move_wait(e, 24)
  cutscene(e, 182)
  pause(e, 0)
end
SCRIPTED[432] = true
WAITS_FIRST[432] = true

-- story actions
story(180, {
  play_sound(81),
  pause_frames(4),
})

story(182, {
  say(161),
})

story(222, {
  say(159),
})
