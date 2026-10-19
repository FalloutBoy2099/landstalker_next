-- Behaviours of room 396 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[483] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  show_when_clear(e)
  wait_flag(e, 8, false)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[483] = true
WAITS_FIRST[483] = true
