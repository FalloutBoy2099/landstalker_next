-- Behaviours of room 179 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[39] = function(e)
  ::L0::
  move_wait(e, 128)
  turn_wait(e, e.orientation + 2)
  jump_guard(e) goto L0
end

B[96] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(10)
  pause(e, 0)
end

B[167] = function(e)
  wait_flag(e, 10, true)
  move_up_to(e, 112)
  pause(e, 0)
end
SCRIPTED[167] = true
WAITS_FIRST[167] = true
