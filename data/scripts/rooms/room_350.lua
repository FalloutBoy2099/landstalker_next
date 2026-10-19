-- Behaviours of room 350 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[733] = function(e)
  pause(e, 255)
  pause(e, 255)
  pause(e, 255)
  pause(e, 255)
  show_when_clear(e)
  wait_trigger(e, 1)
  set_flag(101)
  set_flag(8)
  activate_switch(e)
  pause(e, 0)
end
