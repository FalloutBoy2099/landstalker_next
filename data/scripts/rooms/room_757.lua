-- Behaviours of room 757 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[864] = function(e)
  wait_flag(e, 455, true)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[864] = true
WAITS_FIRST[864] = true
