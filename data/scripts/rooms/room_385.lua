-- Behaviours of room 385 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[497] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(115)
  pause(e, 0)
end

B[498] = function(e)
  wait_trigger(e, 2)
  sfx(106)
  show_when_clear(e)
  wait_trigger(e, 1)
  activate_switch(e)
  pause(e, 2)
  sfx(116)
  set_flag(116)
  pause(e, 0)
end

B[909] = function(e)
  wait_trigger(e, 2)
  pause(e, 10)
  sfx(102)
  e.visible = false; e.base_visible = false; do return end
end
