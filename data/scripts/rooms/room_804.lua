-- Behaviours of room 804 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[632] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(129)
  pause(e, 0)
end

B[641] = function(e)
  wait_flag(e, 129, true)
  move_up_relative(e, 48)
  pause(e, 0)
end
SCRIPTED[641] = true
WAITS_FIRST[641] = true
