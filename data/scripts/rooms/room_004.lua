-- Behaviours of room 4 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[869] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 2)
  set_flag(107)
  e.visible = false; e.base_visible = false; do return end
end
