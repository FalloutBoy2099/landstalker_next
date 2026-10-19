-- Behaviours of room 580 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[456] = function(e)
  wait_flag(e, 265, false)
  wait_trigger(e, 92)
  set_flag(265)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[456] = true
WAITS_FIRST[456] = true
