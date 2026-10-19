-- Behaviours of room 129 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[653] = function(e)
  wait_flag(e, 10, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(11)
  pause(e, 0)
end
SCRIPTED[653] = true
WAITS_FIRST[653] = true

B[764] = function(e)
  ::L0::
  show_when_clear(e)
  wait_flag(e, 11, true)
  e.visible = false
  wait_flag(e, 11, false)
  jump_guard(e) goto L0
end
SCRIPTED[764] = true
