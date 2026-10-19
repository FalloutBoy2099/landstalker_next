-- Behaviours of room 756 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[195] = function(e)
  wait_trigger(e, 2)
  sfx(92)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(25)
  pause(e, 0)
end
