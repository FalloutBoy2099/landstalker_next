-- Behaviours of room 511 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[461] = function(e)
  set_flag(281)
  wait_flag(e, 295, true)
  wait_flag(e, 280, true)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[461] = true
