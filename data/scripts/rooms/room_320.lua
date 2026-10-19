-- Behaviours of room 320 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[803] = function(e)
  show_when_clear(e)
  pause(e, 250)
  pause(e, 250)
  e.visible = false; e.base_visible = false; do return end
end
