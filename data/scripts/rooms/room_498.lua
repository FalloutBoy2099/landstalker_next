-- Behaviours of room 498 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[115] = function(e)
  playback(10)
  move_timed(e, 8)
  pause(e, 8)
  e.falls = true; e.fall_v = 0
  pause(e, 0)
end
