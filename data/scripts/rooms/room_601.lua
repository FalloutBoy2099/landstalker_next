-- Behaviours of room 601 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[47] = function(e)
  ::L0::
  move_wait(e, 32)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 16)
  pause(e, 20)
  sfx(83)
  pause(e, 32)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 16)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 32)
  jump_guard(e) goto L0
end

B[122] = function(e)
  wait_trigger(e, 10)
  cutscene(e, 4)
  set_flag(19)
  pause(e, 0)
end
SCRIPTED[122] = true

B[123] = function(e)
  wait_trigger(e, 11)
  cutscene(e, 5)
  playback(11)
  wait_trigger(e, 12)
  cutscene(e, 6)
  set_flag(20)
  pause(e, 0)
end
SCRIPTED[123] = true

B[124] = function(e)
  wait_flag(e, 20, true)
  move_noclip(e, 160)
  turn_wait(e, e.orientation + 3)
  pause(e, 34)
  turn(e, e.orientation + 3)
  move_noclip(e, 160)
  pause(e, 0)
end
SCRIPTED[124] = true
WAITS_FIRST[124] = true

B[125] = function(e)
  wait_flag(e, 20, true)
  pause(e, 170)
  move_up_relative(e, 1)
  pause(e, 2)
  move_up_relative(e, 1)
  pause(e, 2)
  move_up_relative(e, 1)
  pause(e, 2)
  move_up_relative(e, 1)
  pause(e, 14)
  move_noclip(e, 160)
  cutscene(e, 7)
  pause(e, 0)
end
SCRIPTED[125] = true
WAITS_FIRST[125] = true

B[126] = function(e)
  wait_flag(e, 20, true)
  move_noclip(e, 160)
  turn_wait(e, e.orientation + 1)
  pause(e, 34)
  turn(e, e.orientation + 1)
  move_noclip(e, 160)
  pause(e, 0)
end
SCRIPTED[126] = true
WAITS_FIRST[126] = true

-- story actions
story(4, {
  run(346),
  say(14),
  friday_does("leave"),
})

story(5, {
  run(346),
  say(15),
})

story(6, {
  say(16),
})

story(7, {
  friday_does("leave"),
})
