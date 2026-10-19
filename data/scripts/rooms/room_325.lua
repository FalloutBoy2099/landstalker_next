-- Behaviours of room 325 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[781] = function(e)
  wait_trigger(e, 2)
  move_down_relative(e, 88)
  pause(e, 0)
end
