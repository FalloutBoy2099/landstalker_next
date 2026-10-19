-- Behaviours of room 48 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[592] = function(e)
  show_when_clear(e)
  wait_trigger(e, 2)
  set_flag(94)
  e.visible = false; e.base_visible = false; do return end
end
