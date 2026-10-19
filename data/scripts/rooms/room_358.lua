-- Behaviours of room 358 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[810] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(402)
  pause(e, 0)
end

B[811] = function(e)
  wait_flag(e, 402, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[811] = true
WAITS_FIRST[811] = true
