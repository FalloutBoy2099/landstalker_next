-- Behaviours of room 167 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[37] = function(e)
  pause_player()
  move_wait(e, 48)
  turn_wait(e, e.orientation + 1)
  move_wait(e, 240)
  e.speed = 2
  move_wait(e, 192)
  pause(e, 0)
end
