-- Behaviours of room 589 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[458] = function(e)
  wait_flag(e, 267, false)
  wait_trigger(e, 93)
  set_flag(267)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[458] = true
WAITS_FIRST[458] = true
