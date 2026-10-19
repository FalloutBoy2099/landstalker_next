-- Behaviours of room 348 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[617] = function(e)
  show_when_clear(e)
  pause(e, 0)
end

B[618] = function(e)
  pause(e, 16)
  cutscene(e, 242)
  sfx(47)
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[618] = true

B[619] = function(e)
  cutscene(e, 244)
  wait_trigger(e, 98)
  wait_trigger(e, 107)
  pause(e, 4)
  cutscene(e, 243)
  pause(e, 0)
  do return end  -- unknown command 5B
end
SCRIPTED[619] = true

B[621] = function(e)
  wait_flag(e, 461, true)
  wait_trigger(e, 108)
  pause_player()
  move_to(e, 22, 19)
  cutscene(e, 245)
  set_flag(9)
  turn(e, 3)
  move_noclip(e, 16)
  pause(e, 16)
  cutscene(e, 246)
  pause(e, 0)
end
SCRIPTED[621] = true
WAITS_FIRST[621] = true

B[917] = function(e)
  wait_flag(e, 461, true)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end
SCRIPTED[917] = true
WAITS_FIRST[917] = true

-- story actions
story(242, {
  say(218),
  pause_frames(6),
})

story(243, {
  say(219),
  make_hostile(4, false),
  show_entity(4, true),
})

story(244, {
  make_hostile(4, false),
})

story(245, {
  play_sound(84),
  play_sound(87),
})

story(246, {
  say(220),
  say(221),
  run(346),
  say(222),
  friday_does("hide"),
  lock_controls(false),
  run(154),
})
