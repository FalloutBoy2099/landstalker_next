-- Behaviours that several rooms and several kinds of entities use.

B[1] = function(e)
  ::L0::
  move_timed(e, 16)
  pause(e, 64)
  turn_wait(e, math.random(0, 3))
  jump_guard(e) goto L0
end

B[2] = function(e)
  do return end  -- unknown command 2E
end

B[4] = function(e)
  ::L0::
  move_until_collision(e)
  turn_wait(e, math.random(0, 3))
  jump_guard(e) goto L0
end

B[46] = function(e)
  ::L0::
  pause(e, 30)
  move_wait(e, 80)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[63] = function(e)
  wait_trigger(e, 1)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end

B[70] = function(e)
  wait_trigger(e, 1)
  pause(e, 30)
  e.falls = true; e.fall_v = 0
  move_down_until_collision(e)
  e.visible = false; e.base_visible = false; do return end
end

B[72] = function(e)
  ::L0::
  move_up_relative(e, 48)
  move_down_relative(e, 48)
  jump_guard(e) goto L0
end

B[74] = function(e)
  ::L0::
  move_down_until_collision(e)
  sfx(88)
  pause(e, 5)
  e.speed = 1
  e.speed = 4
  pause(e, 30)
  jump_guard(e) goto L0
end

B[78] = function(e)
  wait_trigger(e, 2)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end

B[85] = function(e)
  ::L0::
  move_until_collision(e)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[98] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  show_when_clear(e)
  pause(e, 150)
  e.visible = false
  clear_flag(8)
  jump_guard(e) goto L0
end
SCRIPTED[98] = true
WAITS_FIRST[98] = true

B[106] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 2)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end

B[149] = function(e)
  ::L0::
  move_until_collision(e)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[158] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_flag(e, 8, true)
  e.visible = false
  sfx(88)
  pause(e, 0)
end
SCRIPTED[158] = true

B[177] = function(e)
  ::L0::
  move_down_until_collision(e)
  sfx(88)
  e.speed = 1
  pause(e, 10)
  e.speed = 4
  pause(e, 10)
  jump_guard(e) goto L0
end

B[186] = function(e)
  pause(e, 40)
  ::L2::
  move_down_until_collision(e)
  sfx(88)
  do return end  -- unknown command 4D
end

B[187] = function(e)
  pause(e, 70)
  ::L2::
  move_down_until_collision(e)
  sfx(88)
  do return end  -- unknown command 4D
end

B[188] = function(e)
  pause(e, 130)
  ::L2::
  move_down_until_collision(e)
  sfx(88)
  do return end  -- unknown command 4D
end

B[189] = function(e)
  pause(e, 160)
  ::L2::
  move_down_until_collision(e)
  sfx(88)
  do return end  -- unknown command 4D
end

B[190] = function(e)
  pause(e, 160)
  ::L2::
  move_down_until_collision(e)
  sfx(88)
  do return end  -- unknown command 4D
end

B[191] = function(e)
  pause(e, 190)
  ::L2::
  move_down_until_collision(e)
  sfx(88)
  do return end  -- unknown command 4D
end

B[192] = function(e)
  pause(e, 220)
  ::L2::
  move_down_until_collision(e)
  sfx(88)
  do return end  -- unknown command 4D
end

B[201] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  pause(e, 0)
end

B[202] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(9)
  pause(e, 0)
end

B[211] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  wait_trigger(e, 30)
  turn(e, 0)
  clear_flag(8)
  jump_guard(e) goto L0
end

B[212] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(9)
  wait_trigger(e, 30)
  turn(e, 0)
  clear_flag(9)
  jump_guard(e) goto L0
end

B[213] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  move_down_until_collision(e)
  sfx(88)
  wait_flag(e, 8, false)
  jump_guard(e) goto L0
end
SCRIPTED[213] = true
WAITS_FIRST[213] = true

B[215] = function(e)
  ::L0::
  show_when_clear(e)
  wait_flag(e, 8, true)
  e.visible = false
  wait_flag(e, 8, false)
  jump_guard(e) goto L0
end
SCRIPTED[215] = true

B[226] = function(e)
  ::L0::
  pause(e, 30)
  move_wait(e, 8)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[234] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(10)
  wait_trigger(e, 30)
  turn(e, 0)
  clear_flag(10)
  jump_guard(e) goto L0
end

B[236] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(11)
  wait_trigger(e, 30)
  turn(e, 0)
  clear_flag(11)
  jump_guard(e) goto L0
end

B[259] = function(e)
  clear_flag(27)
  pause(e, 0)
end

B[305] = function(e)
  show_when_clear(e)
  pause(e, 0)
end

B[307] = function(e)
  show_when_clear(e)
  ::L1::
  wait_flag(e, 9, true)
  e.visible = false
  wait_flag(e, 9, false)
  show_when_clear(e)
  jump_guard(e) goto L1
end
SCRIPTED[307] = true

B[313] = function(e)
  wait_flag(e, 8, true)
  move_wait(e, 32)
  move_wait(e, 32)
  pause(e, 0)
end
SCRIPTED[313] = true
WAITS_FIRST[313] = true

B[337] = function(e)
  wait_flag(e, 8, true)
  e.falls = true; e.fall_v = 0
  e.hostile = true; e.dormant = false
  pause(e, 0)
end
SCRIPTED[337] = true
WAITS_FIRST[337] = true

B[362] = function(e)
  ::L0::
  move_down_until_collision(e)
  pause(e, 20)
  pause(e, 20)
  jump_guard(e) goto L0
end

B[410] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  e.hostile = true; e.dormant = false
  wait_flag(e, 8, false)
  e.hostile = false
  jump_guard(e) goto L0
end
SCRIPTED[410] = true
WAITS_FIRST[410] = true

B[480] = function(e)
  wait_flag(e, 8, true)
  show_when_clear(e)
  pause(e, 0)
end
SCRIPTED[480] = true
WAITS_FIRST[480] = true

B[484] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  pause(e, 0)
end

B[490] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  show_when_clear(e)
  wait_flag(e, 8, false)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[490] = true
WAITS_FIRST[490] = true

B[494] = function(e)
  ::L0::
  wait_flag(e, 8, false)
  move_down_until_collision(e)
  sfx(88)
  wait_flag(e, 8, true)
  jump_guard(e) goto L0
end
SCRIPTED[494] = true
WAITS_FIRST[494] = true

B[507] = function(e)
  show_when_clear(e)
  wait_flag(e, 117, true)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[507] = true

B[607] = function(e)
  wait_flag(e, 8, true)
  e.speed = 1
  e.falls = true; e.fall_v = 0
  e.hostile = true; e.dormant = false
  e.dormant = false; do return end
  pause(e, 0)
end
SCRIPTED[607] = true
WAITS_FIRST[607] = true

B[608] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  show_when_clear(e)
  pause(e, 20)
  wait_flag(e, 8, false)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[608] = true
WAITS_FIRST[608] = true

B[625] = function(e)
  ::L0::
  wait_trigger(e, 1)
  pause(e, 30)
  e.falls = true; e.fall_v = 0
  move_down_until_collision(e)
  pause(e, 180)
  do return end  -- unknown command 4D
end

B[701] = function(e)
  ::L0::
  show_when_clear(e)
  wait_flag(e, 10, true)
  e.visible = false
  wait_flag(e, 10, false)
  jump_guard(e) goto L0
end
SCRIPTED[701] = true

B[727] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  show_when_clear(e)
  wait_flag(e, 8, false)
  e.visible = false
  jump_guard(e) goto L0
end
SCRIPTED[727] = true
WAITS_FIRST[727] = true

B[806] = function(e)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  e.visible = false
  pause(e, 0)
end

B[895] = function(e)
  e.dormant = false; do return end
  pause(e, 0)
end

B[896] = function(e)
  ::L0::
  show_when_clear(e)
  sfx(88)
  wait_flag(e, 8, true)
  sfx(88)
  e.visible = false
  wait_flag(e, 8, false)
  jump_guard(e) goto L0
end
SCRIPTED[896] = true

B[916] = function(e)
  ::L0::
  move_down_until_collision(e)
  sfx(88)
  pause(e, 20)
  pause(e, 20)
  jump_guard(e) goto L0
end
