-- Behaviours of room 116 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[760] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(138)
  pause(e, 0)
end
