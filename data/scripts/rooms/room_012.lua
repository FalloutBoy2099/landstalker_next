-- Behaviours of room 12 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[166] = function(e)
  show_when_clear(e)
  wait_trigger(e, 2)
  set_flag(87)
  e.visible = false; e.base_visible = false; do return end
end

B[170] = function(e)
  wait_trigger(e, 1)
  set_flag(2)
  activate_switch(e)
  pause(e, 1)
  sfx(116)
  pause(e, 0)
end
