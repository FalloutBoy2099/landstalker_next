-- Behaviours that several rooms use for the entities of this kind (small grey ball).

B[48] = function(e)
  ::L0::
  move_wait(e, 64)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 36)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[49] = function(e)
  ::L0::
  move_wait(e, 40)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 96)
  turn_wait(e, e.orientation + 1)
  jump_guard(e) goto L0
end

B[229] = function(e)
  ::L0::
  move_down_until_collision(e)
  e.falls = true; e.fall_v = 0
  move_wait(e, 128)
  pause(e, 10)
  e.visible = false; e.base_visible = false; do return end
  do return end  -- unknown command 4D
end

B[230] = function(e)
  pause(e, 112)
  ::L2::
  move_down_until_collision(e)
  e.falls = true; e.fall_v = 0
  move_wait(e, 128)
  pause(e, 10)
  do return end  -- unknown command 4D
end

B[905] = function(e)
  ::L0::
  move_until_collision(e)
  sfx(73)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end
