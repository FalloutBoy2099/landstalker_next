-- Behaviours of room 94 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[284] = function(e)
  playback(25)
  move_wait(e, 20)
  turn(e, 2)
  move_timed(e, 168)
  turn(e, 3)
  move_noclip(e, 60)
  e.visible = false; e.base_visible = false; do return end
end
