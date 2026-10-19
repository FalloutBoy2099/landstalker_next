-- Behaviours of room 810 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[530] = function(e)
  show_when_clear(e)
  wait_flag(e, 118, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[530] = true

B[531] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(118)
  pause(e, 0)
end

B[532] = function(e)
  ::L0::
  move_wait(e, 32)
  turn_wait(e, e.orientation + 2)
  pause(e, 20)
  jump_guard(e) goto L0
end

B[533] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(119)
  pause(e, 0)
end
