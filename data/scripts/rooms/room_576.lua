-- Behaviours of room 576 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[363] = function(e)
  wait_flag(e, 114, true)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 32)
  turn_wait(e, e.orientation + 3)
  pause(e, 0)
end
SCRIPTED[363] = true
WAITS_FIRST[363] = true

B[364] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(114)
  pause(e, 0)
end
