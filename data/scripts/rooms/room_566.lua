-- Behaviours of room 566 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[827] = function(e)
  wait_flag(e, 406, true)
  show_when_clear(e)
  pause(e, 0)
end
SCRIPTED[827] = true
WAITS_FIRST[827] = true

B[828] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(406)
  pause(e, 0)
end
