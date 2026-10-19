-- Behaviours of room 52 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[279] = function(e)
  wait_flag(e, 8, true)
  set_flag(144)
  playback(3)
  turn(e, 2)
  move_noclip(e, 8)
  e.visible = false; e.base_visible = false; do return end
end
SCRIPTED[279] = true
WAITS_FIRST[279] = true
