-- Behaviours that several rooms use for the entities of this kind (invisible cube).

B[144] = function(e)
  ::L0::
  wait_trigger(e, 19)
  wait_trigger(e, 20)
  pause_player()
  wait_trigger(e, 21)
  cutscene(e, 24)
  jump_guard(e) goto L0
end
SCRIPTED[144] = true

B[312] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  e.visible = false; e.base_visible = false; do return end
  jump_guard(e) goto L0
end
SCRIPTED[312] = true
WAITS_FIRST[312] = true

B[314] = function(e)
  wait_trigger(e, 45)
  set_flag(8)
  pause(e, 0)
end

B[539] = function(e)
  e.visible = false; e.base_visible = false; do return end
end

B[599] = function(e)
  wait_trigger(e, 1)
  set_flag(8)
end
