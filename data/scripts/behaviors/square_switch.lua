-- Behaviours that several rooms use for the entities of this kind (square switch).

B[118] = function(e)
  ::L0::
  wait_flag(e, 8, false)
  turn(e, 0)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  jump_guard(e) goto L0
end
SCRIPTED[118] = true
WAITS_FIRST[118] = true
