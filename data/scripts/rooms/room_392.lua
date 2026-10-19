-- Behaviours of room 392 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[485] = function(e)
  wait_flag(e, 8, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(9)
  pause(e, 0)
end
SCRIPTED[485] = true
WAITS_FIRST[485] = true

B[486] = function(e)
  wait_flag(e, 9, true)
  show_when_clear(e)
  ::L4::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(10)
  wait_trigger(e, 30)
  turn(e, 0)
  clear_flag(10)
  jump_guard(e) goto L4
end
SCRIPTED[486] = true
WAITS_FIRST[486] = true

B[487] = function(e)
  wait_flag(e, 9, true)
  show_when_clear(e)
  pause(e, 0)
end
SCRIPTED[487] = true
WAITS_FIRST[487] = true

B[897] = function(e)
  wait_flag(e, 408, true)
  sfx(116)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end
SCRIPTED[897] = true
WAITS_FIRST[897] = true

B[898] = function(e)
  ::L0::
  wait_flag(e, 10, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(408)
  wait_flag(e, 10, false)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[898] = true
WAITS_FIRST[898] = true
