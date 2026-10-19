-- Behaviours of room 590 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[457] = function(e)
  wait_flag(e, 266, false)
  wait_trigger(e, 93)
  set_flag(266)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[457] = true
WAITS_FIRST[457] = true
