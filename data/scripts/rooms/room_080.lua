-- Behaviours of room 80 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[587] = function(e)
  ::L0::
  move_wait(e, 56)
  turn(e, e.orientation + 3)
  move_wait(e, 48)
  turn(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[589] = function(e)
  ::L0::
  wait_flag(e, 9, false)
  show_when_clear(e)
  sfx(88)
  wait_flag(e, 9, true)
  sfx(88)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[589] = true
WAITS_FIRST[589] = true
