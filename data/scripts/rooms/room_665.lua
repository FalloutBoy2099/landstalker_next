-- Behaviours of room 665 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[747] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 117)
  pause_player()
  pause(e, 60)
  sfx(87)
  set_flag(9)
  wait_trigger(e, 118)
  set_flag(10)
  cutscene(e, 0x100 + 52)
  pause(e, 0)
end
SCRIPTED[747] = true
WAITS_FIRST[747] = true

B[748] = function(e)
  wait_flag(e, 9, true)
  ::L3::
  cutscene(e, 0x100 + 54)
  repeat_begin(e, 0)
  ::L7::
  move_wait(e, 1)
  if repeat_next(e) then goto L7 end
  jump_guard(e) goto L3
end
SCRIPTED[748] = true
WAITS_FIRST[748] = true

-- story actions
story(308, {
  if_flag(11, {
      say(297),
    }, {
      say(292),
    }),
})
