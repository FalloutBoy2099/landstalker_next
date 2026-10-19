-- Behaviours of room 563 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[434] = function(e)
  wait_trigger(e, 80)
  cutscene(e, 184)
  playback(41)
  move_noclip(e, 48)
  turn(e, e.orientation + 3)
  repeat_begin(e, 8)
  ::L11::
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  if repeat_next(e) then goto L11 end
  move_noclip(e, 48)
  turn(e, e.orientation + 3)
  repeat_begin(e, 8)
  ::L20::
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  if repeat_next(e) then goto L20 end
  cutscene(e, 184)
  move_noclip(e, 80)
  turn(e, 0)
  move_noclip(e, 32)
  turn_wait(e, 3)
  repeat_begin(e, 8)
  ::L34::
  turn(e, e.orientation + 3)
  pause(e, 2)
  if repeat_next(e) then goto L34 end
  cutscene(e, 185)
  e.speed = 4
  turn(e, 0)
  repeat_begin(e, 3)
  ::L45::
  move_noclip(e, 24)
  if repeat_next(e) then goto L45 end
  set_flag(272)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[434] = true

B[435] = function(e)
  wait_trigger(e, 80)
  repeat_begin(e, 4)
  ::L4::
  move_noclip(e, 48)
  turn(e, e.orientation + 3)
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  turn_wait(e, e.orientation + 3)
  pause(e, 2)
  if repeat_next(e) then goto L4 end
  turn(e, math.random(0, 3))
  e.speed = 4
  repeat_begin(e, 3)
  ::L36::
  move_noclip(e, 24)
  if repeat_next(e) then goto L36 end
  e.visible = false; e.base_visible = false; do return end
end

-- story actions
story(184, {
  say(163),
})

story(185, {
  say(164),
})
