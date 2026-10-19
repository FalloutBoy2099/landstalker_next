-- Behaviours of room 535 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[468] = function(e)
  set_flag(296)
  wait_flag(e, 295, true)
  wait_flag(e, 297, true)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[468] = true
