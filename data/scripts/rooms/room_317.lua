-- Behaviours of room 317 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[789] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(401)
  pause(e, 2)
  sfx(116)
  pause(e, 0)
end

B[790] = function(e)
  wait_flag(e, 401, false)
  show_when_clear(e)
  wait_flag(e, 401, true)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[790] = true
WAITS_FIRST[790] = true
