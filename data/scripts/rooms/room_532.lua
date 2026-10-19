-- Behaviours of room 532 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[455] = function(e)
  wait_flag(e, 264, false)
  wait_trigger(e, 91)
  set_flag(264)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[455] = true
WAITS_FIRST[455] = true
