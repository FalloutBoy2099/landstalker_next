-- Behaviours of room 321 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[890] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  sfx(86)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(407)
  pause(e, 0)
end
