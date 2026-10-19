-- Behaviours of room 503 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[572] = function(e)
  ::L0::
  wait_trigger(e, 1)
  move_down_until_collision(e)
  wait_trigger(e, 30)
  jump_guard(e) goto L0
end

B[573] = function(e)
  wait_trigger(e, 1)
  move_down_until_collision(e)
  pause(e, 0)
end

B[574] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  pause(e, 40)
  show_when_clear(e)
  pause(e, 40)
  clear_flag(8)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[574] = true
WAITS_FIRST[574] = true

B[575] = function(e)
  ::L0::
  wait_flag(e, 8, false)
  turn(e, 0)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  jump_guard(e) goto L0
end
SCRIPTED[575] = true
WAITS_FIRST[575] = true

B[576] = function(e)
  ::L0::
  pause(e, 20)
  repeat_begin(e, 144)
  ::L4::
  move_down_relative(e, 1)
  move_wait(e, 1)
  if repeat_next(e) then goto L4 end
  pause(e, 20)
  turn(e, e.orientation + 2)
  repeat_begin(e, 144)
  ::L14::
  move_up_relative(e, 1)
  move_wait(e, 1)
  if repeat_next(e) then goto L14 end
  turn(e, e.orientation + 2)
  jump_guard(e) goto L0
end
