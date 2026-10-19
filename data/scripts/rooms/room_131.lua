-- Behaviours of room 131 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[731] = function(e)
  ::L0::
  move_wait(e, 28)
  turn_wait(e, e.orientation + 3)
  move_wait(e, 36)
  turn_wait(e, e.orientation + 3)
  jump_guard(e) goto L0
end

B[755] = function(e)
  set_flag(9)
  ::L3::
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  clear_flag(9)
  wait_trigger(e, 30)
  wait_trigger(e, 1)
  turn(e, 0)
  set_flag(9)
  clear_flag(8)
  wait_trigger(e, 30)
  jump_guard(e) goto L3
end
