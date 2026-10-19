-- Behaviours of room 328 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[674] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  ::L3::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  wait_trigger(e, 30)
  turn(e, 0)
  clear_flag(8)
  jump_guard(e) goto L3
end
