-- Behaviours of room 664 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[745] = function(e)
  wait_flag(e, 8, true)
  wait_trigger(e, 1)
  pause_player()
  pause(e, 30)
  move_until_collision(e)
  pause(e, 60)
  set_flag(9)
  turn(e, 3)
  e.speed = 8
  cutscene(e, 0x100 + 46)
  repeat_begin(e, 0)
  ::L20::
  move_until_collision(e)
  turn_wait(e, e.orientation + 1)
  if repeat_next(e) then goto L20 end
  e.speed = 4
  cutscene(e, 0x100 + 47)
  repeat_begin(e, 0)
  ::L28::
  move_until_collision(e)
  turn_wait(e, e.orientation + 1)
  if repeat_next(e) then goto L28 end
  e.speed = 2
  cutscene(e, 0x100 + 48)
  repeat_begin(e, 0)
  ::L36::
  move_until_collision(e)
  turn_wait(e, e.orientation + 1)
  if repeat_next(e) then goto L36 end
  e.speed = 1
  cutscene(e, 0x100 + 49)
  repeat_begin(e, 0)
  ::L44::
  move_wait(e, 16)
  if repeat_next(e) then goto L44 end
  cutscene(e, 0x100 + 51)
  pause(e, 0)
end
SCRIPTED[745] = true
WAITS_FIRST[745] = true

B[746] = function(e)
  wait_flag(e, 9, true)
  ::L3::
  wait_trigger(e, 116)
  sfx(66)
  pause(e, 1)
  jump_guard(e) goto L3
end
SCRIPTED[746] = true
WAITS_FIRST[746] = true

-- story actions
story(307, {
  story_flag(10, true),
  say(291),
})
