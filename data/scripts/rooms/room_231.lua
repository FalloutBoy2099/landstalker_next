-- Behaviours of room 231 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[75] = function(e)
  ::L0::
  move_wait(e, 144)
  move_down_to(e, 0)
  pause(e, 30)
  move_up_to(e, 16)
  turn_wait(e, 1)
  move_wait(e, 144)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[76] = function(e)
  ::L0::
  move_wait(e, 80)
  turn_wait(e, 0)
  move_down_to(e, 0)
  move_wait(e, 176)
  turn_wait(e, 3)
  move_wait(e, 80)
  turn_wait(e, 2)
  move_wait(e, 176)
  move_up_to(e, 48)
  turn_wait(e, 1)
  jump_guard(e) goto L0
end

B[545] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(121)
  pause(e, 0)
end
