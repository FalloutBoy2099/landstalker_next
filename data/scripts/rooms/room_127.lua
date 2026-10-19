-- Behaviours of room 127 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[728] = function(e)
  ::L0::
  wait_flag(e, 9, true)
  show_when_clear(e)
  wait_flag(e, 9, false)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[728] = true
WAITS_FIRST[728] = true
