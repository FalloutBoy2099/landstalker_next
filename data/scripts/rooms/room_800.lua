-- Behaviours of room 800 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[698] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  wait_trigger(e, 30)
  pause(e, 80)
  turn(e, 0)
  clear_flag(8)
  jump_guard(e) goto L0
end

B[699] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(9)
  wait_trigger(e, 30)
  pause(e, 60)
  clear_flag(9)
  turn(e, 0)
  jump_guard(e) goto L0
end

B[702] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(10)
  wait_trigger(e, 30)
  pause(e, 50)
  clear_flag(10)
  turn(e, 0)
  jump_guard(e) goto L0
end

B[703] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(126)
  pause(e, 0)
end
