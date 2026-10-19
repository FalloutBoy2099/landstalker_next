-- Behaviours of room 763 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[175] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(43)
  pause(e, 0)
end

B[176] = function(e)
  wait_flag(e, 43, true)
  show_when_clear(e)
  pause(e, 0)
end
SCRIPTED[176] = true
WAITS_FIRST[176] = true
