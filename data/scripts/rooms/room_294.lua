-- Behaviours of room 294 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[310] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  pause(e, 20)
  turn(e, 0)
  clear_flag(8)
  wait_trigger(e, 30)
  jump_guard(e) goto L0
end

B[311] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  show_when_clear(e)
  wait_flag(e, 8, false)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[311] = true
WAITS_FIRST[311] = true
