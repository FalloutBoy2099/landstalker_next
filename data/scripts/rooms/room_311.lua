-- Behaviours of room 311 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[670] = function(e)
  ::L0::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  pause(e, 5)
  turn(e, 0)
  clear_flag(8)
  wait_trigger(e, 30)
  jump_guard(e) goto L0
end

B[775] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(142)
  pause(e, 0)
end
