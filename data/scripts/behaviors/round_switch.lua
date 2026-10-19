-- Behaviours that several rooms use for the entities of this kind (round switch).

B[241] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  pause(e, 240)
  turn(e, 0)
  clear_flag(8)
  jump_guard(e) goto L0
end

B[479] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  pause(e, 0)
end

B[588] = function(e)
  wait_flag(e, 8, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(9)
  pause(e, 0)
end
SCRIPTED[588] = true
WAITS_FIRST[588] = true

B[652] = function(e)
  wait_flag(e, 9, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(10)
  pause(e, 0)
end
SCRIPTED[652] = true
WAITS_FIRST[652] = true
