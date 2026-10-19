-- Behaviours of room 254 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[724] = function(e)
  wait_trigger(e, 2)
  sfx(88)
  set_flag(99)
  e.visible = false; e.base_visible = false; do return end
end
