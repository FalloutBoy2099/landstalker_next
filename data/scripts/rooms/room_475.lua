-- Behaviours of room 475 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[366] = function(e)
  wait_flag(e, 166, false)
  ::L3::
  wait_trigger(e, 59)
  pause_player()
  pause(e, 10)
  cutscene(e, 134)
  playback(3)
  pause(e, 8)
  jump_guard(e) goto L3
end
SCRIPTED[366] = true
WAITS_FIRST[366] = true

B[367] = function(e)
  wait_flag(e, 166, true)
  wait_flag(e, 180, false)
  wait_trigger(e, 59)
  pause_player()
  pause(e, 10)
  cutscene(e, 135)
  set_flag(180)
  pause(e, 0)
end
SCRIPTED[367] = true
WAITS_FIRST[367] = true

B[919] = function(e)
  move_wait(e, 1)
  e.speed = 8
  ::L3::
  move_wait(e, 1)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L3
end

-- story actions
story(134, {
  say(123),
})

story(135, {
  say(124),
  hide_entity(9),
  hide_entity(11),
})
