-- Behaviours of room 86 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[283] = function(e)
  playback(24)
  move_noclip(e, 8)
  turn(e, 1)
  move_noclip(e, 80)
  turn(e, 0)
  move_wait(e, 24)
  move_wait(e, 24)
  move_wait(e, 16)
  move_wait(e, 16)
  move_noclip(e, 32)
  e.visible = false; e.base_visible = false; do return end
end
