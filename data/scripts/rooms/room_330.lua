-- Behaviours of room 330 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[774] = function(e)
  wait_flag(e, 117, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[774] = true
WAITS_FIRST[774] = true

B[901] = function(e)
  show_when_clear(e)
  sfx(88)
  ::L3::
  wait_flag(e, 9, true)
  sfx(88)
  e.visible = false
  wait_flag(e, 9, false)
  sfx(88)
  show_when_clear(e)
  jump_guard(e) goto L3
end
SCRIPTED[901] = true
