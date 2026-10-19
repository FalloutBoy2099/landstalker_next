-- Behaviours of room 323 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[678] = function(e)
  show_when_clear(e)
  sfx(88)
  wait_trigger(e, 95)
  sfx(88)
  e.visible = false; e.base_visible = false; do return end
end
