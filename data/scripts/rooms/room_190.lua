-- Behaviours of room 190 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[196] = function(e)
  wait_trigger(e, 2)
  tile_swap(1)
  set_flag(88)
  e.visible = false; e.base_visible = false; do return end
end
