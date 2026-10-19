-- Behaviours of room 135 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[911] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 30)
  sfx(90)
  set_flag(10)
  pause(e, 0)
end

B[912] = function(e)
  wait_flag(e, 10, true)
  sfx(102)
  pause(e, 10)
  sfx(102)
  pause(e, 10)
  sfx(102)
  pause(e, 30)
  e.speed = 1
  move_timed(e, 16)
  repeat_begin(e, 4)
  ::L24::
  move_timed(e, 48)
  if repeat_next(e) then goto L24 end
  pause(e, 0)
end
SCRIPTED[912] = true
WAITS_FIRST[912] = true

B[913] = function(e)
  wait_flag(e, 9, true)
  pause(e, 16)
  sfx(102)
  pause(e, 16)
  sfx(102)
  pause(e, 16)
  sfx(102)
  pause(e, 16)
  sfx(102)
  pause(e, 40)
  e.speed = 1
  move_timed(e, 16)
  move_timed(e, 48)
  repeat_begin(e, 2)
  ::L33::
  move_timed(e, 48)
  if repeat_next(e) then goto L33 end
  pause(e, 0)
end
SCRIPTED[913] = true
WAITS_FIRST[913] = true

B[914] = function(e)
  wait_flag(e, 8, true)
  sfx(103)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[914] = true
WAITS_FIRST[914] = true
