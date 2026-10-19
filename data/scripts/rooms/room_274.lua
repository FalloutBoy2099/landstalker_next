-- Behaviours of room 274 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[807] = function(e)
  wait_trigger(e, 1)
  set_flag(12)
end

B[808] = function(e)
  wait_flag(e, 12, true)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 2)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[808] = true
WAITS_FIRST[808] = true
