-- Behaviours of room 307 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[650] = function(e)
  ::L0::
  move_wait(e, 96)
  turn(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[651] = function(e)
  ::L0::
  move_wait(e, 40)
  turn(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[784] = function(e)
  wait_flag(e, 10, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(143)
  pause(e, 0)
end
SCRIPTED[784] = true
WAITS_FIRST[784] = true

B[785] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_flag(e, 143, true)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[785] = true
