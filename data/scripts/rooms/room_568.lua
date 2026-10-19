-- Behaviours of room 568 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[825] = function(e)
  show_when_clear(e)
  wait_flag(e, 109, true)
  e.visible = false
  wait_flag(e, 109, false)
  show_when_clear(e)
end
SCRIPTED[825] = true

B[826] = function(e)
  wait_trigger(e, 2)
  set_flag(109)
  pause(e, 0)
end
