-- Behaviours of room 391 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[849] = function(e)
  wait_trigger(e, 129)
  set_flag(416)
  show_when_clear(e)
  ::L6::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  wait_trigger(e, 30)
  turn(e, 0)
  clear_flag(8)
  jump_guard(e) goto L6
end

B[850] = function(e)
  wait_trigger(e, 130)
  set_flag(417)
  show_when_clear(e)
  ::L6::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(9)
  wait_trigger(e, 30)
  turn(e, 0)
  clear_flag(9)
  jump_guard(e) goto L6
end

B[851] = function(e)
  wait_trigger(e, 131)
  set_flag(418)
  show_when_clear(e)
  ::L6::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(10)
  wait_trigger(e, 30)
  turn(e, 0)
  clear_flag(10)
  jump_guard(e) goto L6
end

B[852] = function(e)
  ::L0::
  show_when_clear(e)
  wait_flag(e, 8, true)
  e.visible = false
  wait_flag(e, 8, false)
  jump_guard(e) goto L0
end
SCRIPTED[852] = true

B[853] = function(e)
  ::L0::
  show_when_clear(e)
  wait_flag(e, 9, true)
  e.visible = false
  wait_flag(e, 9, false)
  jump_guard(e) goto L0
end
SCRIPTED[853] = true

B[854] = function(e)
  ::L0::
  show_when_clear(e)
  wait_flag(e, 10, true)
  e.visible = false
  wait_flag(e, 10, false)
  jump_guard(e) goto L0
end
SCRIPTED[854] = true
