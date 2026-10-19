-- Behaviours of room 18 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[217] = function(e)
  move_wait(e, 80)
  pause(e, 0)
end

B[218] = function(e)
  ::L0::
  wait_flag(e, 8, true)
  move_up_relative(e, 32)
  wait_flag(e, 8, false)
  move_down_relative(e, 32)
  jump_guard(e) goto L0
end
SCRIPTED[218] = true
WAITS_FIRST[218] = true

B[219] = function(e)
  ::L0::
  wait_flag(e, 9, true)
  move_up_relative(e, 32)
  wait_flag(e, 9, false)
  move_down_relative(e, 32)
  jump_guard(e) goto L0
end
SCRIPTED[219] = true
WAITS_FIRST[219] = true
