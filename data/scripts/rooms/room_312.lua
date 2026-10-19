-- Behaviours of room 312 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[659] = function(e)
  ::L0::
  wait_trigger(e, 1)
  pause(e, 30)
  move_down_relative(e, 23)
  e.visible = false
  pause(e, 180)
  do return end  -- unknown command 4D
end

B[772] = function(e)
  wait_flag(e, 8, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(141)
  pause(e, 0)
end
SCRIPTED[772] = true
WAITS_FIRST[772] = true

B[773] = function(e)
  show_when_clear(e)
  wait_flag(e, 141, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[773] = true

B[802] = function(e)
  wait_flag(e, 141, true)
  show_when_clear(e)
  ::L4::
  move_wait(e, 104)
  pause(e, 20)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L4
end
SCRIPTED[802] = true
WAITS_FIRST[802] = true
