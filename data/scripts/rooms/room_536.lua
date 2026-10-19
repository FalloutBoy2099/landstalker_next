-- Behaviours of room 536 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[467] = function(e)
  set_flag(287)
  wait_flag(e, 295, true)
  wait_flag(e, 286, true)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[467] = true
