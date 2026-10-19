-- Behaviours of room 803 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[638] = function(e)
  wait_flag(e, 132, true)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(130)
  pause(e, 0)
end
SCRIPTED[638] = true
WAITS_FIRST[638] = true
