-- Behaviours of room 801 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[707] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  wait_trigger(e, 30)
  pause(e, 110)
  turn(e, 0)
  clear_flag(8)
  jump_guard(e) goto L0
end

B[708] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(9)
  wait_trigger(e, 30)
  pause(e, 110)
  turn(e, 0)
  clear_flag(9)
  jump_guard(e) goto L0
end

B[709] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(10)
  wait_trigger(e, 30)
  pause(e, 170)
  turn(e, 0)
  clear_flag(10)
  jump_guard(e) goto L0
end

B[710] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(11)
  wait_trigger(e, 30)
  pause(e, 184)
  turn(e, 0)
  clear_flag(11)
  jump_guard(e) goto L0
end

B[711] = function(e)
  ::L0::
  wait_flag(e, 11, true)
  e.visible = false
  wait_flag(e, 11, false)
  show_when_clear(e)
  jump_guard(e) goto L0
end
SCRIPTED[711] = true
WAITS_FIRST[711] = true

B[712] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(127)
  pause(e, 0)
end
