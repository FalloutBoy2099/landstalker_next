-- Behaviours of room 658 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[348] = function(e)
  wait_flag(e, 9, true)
  e.falls = true; e.fall_v = 0
  ::L4::
  wait_trigger(e, 1)
  set_flag(10)
  activate_switch(e)
  wait_trigger(e, 6)
  clear_flag(10)
  turn(e, 0)
  jump_guard(e) goto L4
end
SCRIPTED[348] = true
WAITS_FIRST[348] = true

B[349] = function(e)
  ::L0::
  wait_flag(e, 10, true)
  wait_trigger(e, 54)
  set_flag(8)
  wait_flag(e, 10, false)
  jump_guard(e) goto L0
end
SCRIPTED[349] = true
WAITS_FIRST[349] = true

B[350] = function(e)
  wait_flag(e, 172, true)
  cutscene(e, 123)
  pause(e, 0)
end
SCRIPTED[350] = true
WAITS_FIRST[350] = true
