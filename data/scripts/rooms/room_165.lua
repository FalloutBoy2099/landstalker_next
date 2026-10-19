-- Behaviours of room 165 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[36] = function(e)
  pause_player()
  move_wait(e, 240)
  move_wait(e, 16)
  pause(e, 0)
end
