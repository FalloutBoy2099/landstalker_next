-- Behaviours of room 640 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[335] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(113)
  pause(e, 2)
  sfx(116)
  pause(e, 60)
  sfx(89)
  pause(e, 0)
end
