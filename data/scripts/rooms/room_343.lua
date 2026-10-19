-- Behaviours of room 343 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[661] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(134)
  pause(e, 0)
end

B[662] = function(e)
  wait_flag(e, 134, true)
  show_when_clear(e)
  pause(e, 0)
end
SCRIPTED[662] = true
WAITS_FIRST[662] = true
