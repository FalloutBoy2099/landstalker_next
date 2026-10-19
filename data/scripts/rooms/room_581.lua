-- Behaviours of room 581 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[459] = function(e)
  wait_flag(e, 268, false)
  wait_trigger(e, 93)
  set_flag(268)
  tile_swap(1)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[459] = true
WAITS_FIRST[459] = true
