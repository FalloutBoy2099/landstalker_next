-- Behaviours of room 347 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[506] = function(e)
  wait_trigger(e, 95)
  activate_switch(e)
  set_flag(117)
  pause(e, 0)
end
