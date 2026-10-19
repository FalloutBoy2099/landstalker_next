-- Behaviours of room 252 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[611] = function(e)
  wait_trigger(e, 2)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  pause(e, 0)
end

B[622] = function(e)
  wait_flag(e, 8, true)
  set_flag(95)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[622] = true
WAITS_FIRST[622] = true
