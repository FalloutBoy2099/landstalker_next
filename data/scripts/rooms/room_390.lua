-- Behaviours of room 390 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[500] = function(e)
  wait_flag(e, 116, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[500] = true
WAITS_FIRST[500] = true
