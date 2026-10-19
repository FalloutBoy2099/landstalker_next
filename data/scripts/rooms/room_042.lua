-- Behaviours of room 42 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[407] = function(e)
  wait_flag(e, 154, true)
  pause_player()
  move_noclip(e, 48)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[407] = true
WAITS_FIRST[407] = true
