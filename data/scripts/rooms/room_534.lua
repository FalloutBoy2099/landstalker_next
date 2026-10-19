-- Behaviours of room 534 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[463] = function(e)
  set_flag(283)
  wait_flag(e, 295, true)
  wait_flag(e, 282, true)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[463] = true
