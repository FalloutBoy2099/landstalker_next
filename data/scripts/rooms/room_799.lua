-- Behaviours of room 799 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[687] = function(e)
  ::L0::
  wait_flag(e, 8, false)
  turn(e, 0)
  e.visible = false
  wait_flag(e, 8, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(9)
  set_flag(10)
  clear_flag(11)
  clear_flag(12)
  clear_flag(13)
  clear_flag(14)
  set_flag(15)
  clear_flag(136)
  clear_flag(8)
  jump_guard(e) goto L0
end
SCRIPTED[687] = true
WAITS_FIRST[687] = true

B[688] = function(e)
  ::L0::
  wait_flag(e, 9, false)
  turn(e, 0)
  e.visible = false
  wait_flag(e, 9, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  set_flag(10)
  clear_flag(11)
  clear_flag(12)
  clear_flag(13)
  clear_flag(14)
  clear_flag(15)
  set_flag(136)
  clear_flag(9)
  jump_guard(e) goto L0
end
SCRIPTED[688] = true
WAITS_FIRST[688] = true

B[689] = function(e)
  ::L0::
  wait_flag(e, 10, false)
  turn(e, 0)
  e.visible = false
  wait_flag(e, 10, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  clear_flag(8)
  set_flag(9)
  set_flag(11)
  clear_flag(12)
  set_flag(13)
  clear_flag(14)
  clear_flag(15)
  clear_flag(136)
  clear_flag(10)
  jump_guard(e) goto L0
end
SCRIPTED[689] = true
WAITS_FIRST[689] = true

B[690] = function(e)
  ::L0::
  wait_flag(e, 11, false)
  turn(e, 0)
  e.visible = false
  wait_flag(e, 11, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  clear_flag(8)
  clear_flag(9)
  set_flag(10)
  set_flag(12)
  clear_flag(13)
  clear_flag(14)
  clear_flag(15)
  set_flag(136)
  clear_flag(11)
  jump_guard(e) goto L0
end
SCRIPTED[690] = true
WAITS_FIRST[690] = true

B[691] = function(e)
  ::L0::
  wait_flag(e, 12, false)
  turn(e, 0)
  e.visible = false
  wait_flag(e, 12, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  clear_flag(8)
  clear_flag(9)
  set_flag(10)
  set_flag(11)
  set_flag(13)
  clear_flag(14)
  clear_flag(15)
  clear_flag(136)
  clear_flag(12)
  jump_guard(e) goto L0
end
SCRIPTED[691] = true
WAITS_FIRST[691] = true

B[692] = function(e)
  ::L0::
  wait_flag(e, 13, false)
  turn(e, 0)
  e.visible = false
  wait_flag(e, 13, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  clear_flag(8)
  clear_flag(9)
  set_flag(10)
  clear_flag(11)
  set_flag(12)
  set_flag(14)
  clear_flag(15)
  clear_flag(136)
  clear_flag(13)
  jump_guard(e) goto L0
end
SCRIPTED[692] = true
WAITS_FIRST[692] = true

B[693] = function(e)
  ::L0::
  wait_flag(e, 14, false)
  turn(e, 0)
  e.visible = false
  wait_flag(e, 14, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  clear_flag(8)
  clear_flag(9)
  clear_flag(10)
  clear_flag(11)
  clear_flag(12)
  clear_flag(13)
  clear_flag(15)
  set_flag(137)
  clear_flag(14)
  jump_guard(e) goto L0
end
SCRIPTED[693] = true
WAITS_FIRST[693] = true

B[694] = function(e)
  ::L0::
  wait_flag(e, 15, false)
  turn(e, 0)
  e.visible = false
  wait_flag(e, 15, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  set_flag(9)
  clear_flag(10)
  set_flag(11)
  clear_flag(12)
  clear_flag(13)
  clear_flag(14)
  clear_flag(136)
  clear_flag(15)
  jump_guard(e) goto L0
end
SCRIPTED[694] = true
WAITS_FIRST[694] = true

B[695] = function(e)
  ::L0::
  wait_flag(e, 136, false)
  turn(e, 0)
  e.visible = false
  wait_flag(e, 136, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  clear_flag(8)
  set_flag(9)
  set_flag(10)
  set_flag(11)
  clear_flag(12)
  clear_flag(13)
  clear_flag(14)
  clear_flag(15)
  clear_flag(136)
  jump_guard(e) goto L0
end
SCRIPTED[695] = true
WAITS_FIRST[695] = true

B[696] = function(e)
  wait_flag(e, 137, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(125)
  pause(e, 0)
end
SCRIPTED[696] = true
WAITS_FIRST[696] = true
