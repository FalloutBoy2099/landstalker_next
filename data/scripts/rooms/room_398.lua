-- Behaviours of room 398 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[855] = function(e)
  wait_flag(e, 453, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[855] = true
WAITS_FIRST[855] = true
