-- Behaviours of room 117 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[763] = function(e)
  wait_flag(e, 138, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[763] = true
WAITS_FIRST[763] = true
