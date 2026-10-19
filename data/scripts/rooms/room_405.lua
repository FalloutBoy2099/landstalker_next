-- Behaviours of room 405 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[819] = function(e)
  ::L0::
  wait_flag(e, 9, true)
  show_when_clear(e)
  wait_flag(e, 9, false)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[819] = true
WAITS_FIRST[819] = true

B[820] = function(e)
  ::L0::
  wait_flag(e, 10, true)
  show_when_clear(e)
  wait_flag(e, 10, false)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[820] = true
WAITS_FIRST[820] = true
