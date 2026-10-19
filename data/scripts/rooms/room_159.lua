-- Behaviours of room 159 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[41] = function(e)
  wait_trigger(e, 0)
  move_wait(e, 48)
  turn(e, e.orientation + 1)
  move_wait(e, 240)
  move_wait(e, 240)
  pause(e, 0)
end
