-- Behaviours of room 247 (what its entities do: the scenes of the room, its platforms and gates). `behaviour: n` of room.yaml picks B[n].

B[623] = function(e)
  wait_trigger(e, 2)
  set_flag(96)
  pause(e, 0)
end
