-- Behaviours of room 239 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[776] = function(e)
  wait_trigger(e, 2)
  set_flag(101)
  pause(e, 0)
end
