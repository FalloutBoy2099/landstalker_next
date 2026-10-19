-- Behaviours of room 805 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[633] = function(e)
  ::L0::
  wait_flag(e, 129, true)
  move_down_until_collision(e)
  wait_flag(e, 129, false)
  jump_guard(e) goto L0
end
SCRIPTED[633] = true
WAITS_FIRST[633] = true

B[634] = function(e)
  ::L0::
  wait_flag(e, 130, true)
  move_down_until_collision(e)
  wait_flag(e, 130, false)
  jump_guard(e) goto L0
end
SCRIPTED[634] = true
WAITS_FIRST[634] = true

B[635] = function(e)
  ::L0::
  wait_flag(e, 131, true)
  move_down_until_collision(e)
  wait_flag(e, 131, false)
  jump_guard(e) goto L0
end
SCRIPTED[635] = true
WAITS_FIRST[635] = true

B[636] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(132)
  pause(e, 0)
end

B[637] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(133)
  pause(e, 0)
end

B[639] = function(e)
  wait_flag(e, 133, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(131)
  pause(e, 0)
end
SCRIPTED[639] = true
WAITS_FIRST[639] = true

B[640] = function(e)
  pause(e, 50)
  show_when_clear(e)
  e.falls = true; e.fall_v = 0
end
