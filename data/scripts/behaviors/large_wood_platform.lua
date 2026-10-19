-- Behaviours that several rooms use for the entities of this kind (large wood platform).

B[43] = function(e)
  ::L0::
  move_down_relative(e, 40)
  move_up_relative(e, 70)
  move_down_relative(e, 30)
  jump_guard(e) goto L0
end

B[44] = function(e)
  ::L0::
  move_up_relative(e, 40)
  move_down_relative(e, 70)
  move_up_relative(e, 30)
  jump_guard(e) goto L0
end

B[94] = function(e)
  ::L0::
  move_down_until_collision(e)
  move_up_to(e, 112)
  jump_guard(e) goto L0
end

B[95] = function(e)
  ::L0::
  move_up_relative(e, 32)
  move_down_relative(e, 80)
  move_up_relative(e, 48)
  jump_guard(e) goto L0
end

B[245] = function(e)
  ::L0::
  move_down_relative(e, 100)
  pause(e, 30)
  move_up_relative(e, 240)
  move_up_relative(e, 20)
  pause(e, 30)
  move_down_relative(e, 160)
  jump_guard(e) goto L0
end

B[246] = function(e)
  ::L0::
  move_down_relative(e, 216)
  pause(e, 30)
  move_up_relative(e, 242)
  move_up_relative(e, 54)
  pause(e, 30)
  move_down_relative(e, 80)
  jump_guard(e) goto L0
end

B[247] = function(e)
  ::L0::
  move_up_relative(e, 100)
  pause(e, 30)
  move_down_relative(e, 242)
  move_down_relative(e, 98)
  pause(e, 30)
  move_up_relative(e, 240)
  jump_guard(e) goto L0
end

B[861] = function(e)
  wait_flag(e, 454, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  pause(e, 30)
  e.falls = true; e.fall_v = 0
  move_down_until_collision(e)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[861] = true
WAITS_FIRST[861] = true
