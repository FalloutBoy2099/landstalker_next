-- Behaviours of room 306 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[786] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(400)
  pause(e, 0)
end

B[787] = function(e)
  ::L0::
  wait_flag(e, 400, true)
  e.hostile = true; e.dormant = false
  wait_flag(e, 400, false)
  e.hostile = false
  jump_guard(e) goto L0
end
SCRIPTED[787] = true
WAITS_FIRST[787] = true
