-- Behaviours of room 25 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[127] = function(e)
  wait_trigger(e, 2)
  e.falls = true; e.fall_v = 0
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  pause(e, 0)
end

B[626] = function(e)
  show_when_clear(e)
  wait_flag(e, 8, true)
  set_flag(92)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[626] = true
