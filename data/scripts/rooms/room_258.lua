-- Behaviours of room 258 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[546] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(122)
  pause(e, 0)
end
