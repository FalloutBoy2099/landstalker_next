-- Behaviours of room 128 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[762] = function(e)
  wait_trigger(e, 1)
  activate_switch(e)
  set_flag(8)
  wait_trigger(e, 98)
  wait_trigger(e, 2)
  set_flag(139)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
