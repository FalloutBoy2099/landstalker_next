-- Behaviours of room 204 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[108] = function(e)
  ::L0::
  do return end  -- unknown command 2C
end

B[231] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  move_up_to(e, 1)
  wait_flag(e, 8, false)
  move_down_until_collision(e)
  jump_guard(e) goto L0
end
SCRIPTED[231] = true
WAITS_FIRST[231] = true

B[233] = function(e)
  ::L0::
  wait_flag(e, 9, true)
  move_up_to(e, 1)
  wait_flag(e, 9, false)
  move_down_until_collision(e)
  jump_guard(e) goto L0
end
SCRIPTED[233] = true
WAITS_FIRST[233] = true

B[235] = function(e)
  ::L0::
  wait_flag(e, 10, true)
  move_up_to(e, 1)
  wait_flag(e, 10, false)
  move_down_until_collision(e)
  jump_guard(e) goto L0
end
SCRIPTED[235] = true
WAITS_FIRST[235] = true
