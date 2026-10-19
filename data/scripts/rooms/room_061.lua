-- Behaviours of room 61 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[397] = function(e)
  playback(20)
  move_noclip(e, 24)
  move_noclip(e, 72)
  e.visible = false; e.base_visible = false; do return end
end
