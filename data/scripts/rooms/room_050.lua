-- Behaviours of room 50 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[591] = function(e)
  show_when_clear(e)
  wait_trigger(e, 2)
  set_flag(93)
  e.visible = false; e.base_visible = false; do return end
end
